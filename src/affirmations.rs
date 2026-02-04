use serde::Deserialize;
use std::{collections::HashMap, fs, path::Path, sync::LazyLock};

#[derive(Debug, Deserialize, Clone)]
pub struct MoodSet {
    pub positive: Vec<String>,
    pub negative: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AffirmationsFile {
    #[serde(default)]
    pub moods: HashMap<String, MoodSet>,
    #[serde(default)]
    pub positive: Vec<String>,
    #[serde(default)]
    pub negative: Vec<String>,
}

#[derive(Debug)]
pub struct Affirmations<'a> {
    pub positive: &'a [String],
    pub negative: &'a [String],
}

#[derive(Debug)]
pub struct AffirmationsOwned {
    pub positive: Vec<String>,
    pub negative: Vec<String>,
}

#[derive(Debug)]
pub enum AffirmationData<'a> {
    Borrowed(Affirmations<'a>),
    Owned(AffirmationsOwned),
}

impl<'a> AffirmationData<'a> {
    pub fn positive(&self) -> &[String] {
        match self {
            AffirmationData::Borrowed(a) => a.positive,
            AffirmationData::Owned(a) => &a.positive,
        }
    }

    pub fn negative(&self) -> &[String] {
        match self {
            AffirmationData::Borrowed(a) => a.negative,
            AffirmationData::Owned(a) => &a.negative,
        }
    }
}

fn parse_affirmations(json_str: &str, mood: Option<&str>) -> Option<AffirmationsOwned> {
    let file: AffirmationsFile = serde_json::from_str(json_str).ok()?;

    Some(affirmations_from_file_owned(&file, mood))
}

/// Helper to get the appropriate mood set from the file.
/// Returns the requested mood or the "chill" mood. Returns `None` if neither is
/// found.
#[inline]
fn get_mood_set<'a>(file: &'a AffirmationsFile, mood: Option<&str>) -> Option<&'a MoodSet> {
    mood.and_then(|m| file.moods.get(m))
        .or_else(|| file.moods.get("chill"))
}

fn affirmations_from_file<'a>(file: &'a AffirmationsFile, mood: Option<&str>) -> Affirmations<'a> {
    // Return references to avoid cloning entirely
    if let Some(mood_set) = get_mood_set(file, mood) {
        Affirmations {
            positive: &mood_set.positive,
            negative: &mood_set.negative,
        }
    } else {
        Affirmations {
            positive: &file.positive,
            negative: &file.negative,
        }
    }
}

fn affirmations_from_file_owned(file: &AffirmationsFile, mood: Option<&str>) -> AffirmationsOwned {
    // For custom affirmations, we need owned data
    if let Some(mood_set) = get_mood_set(file, mood) {
        AffirmationsOwned {
            positive: mood_set.positive.clone(),
            negative: mood_set.negative.clone(),
        }
    } else {
        AffirmationsOwned {
            positive: file.positive.clone(),
            negative: file.negative.clone(),
        }
    }
}

// Cache parsed embedded affirmations to avoid repeated JSON parsing
static EMBEDDED_AFFIRMATIONS: LazyLock<AffirmationsFile> = LazyLock::new(|| {
    serde_json::from_str(include_str!("../assets/affirmations.json"))
        .expect("Failed to parse embedded affirmations")
});

pub fn load_affirmations_with_mood(mood: &str) -> Option<AffirmationData<'static>> {
    // Use cached parsed affirmations instead of parsing JSON every time
    // Returns references to the static embedded affirmations - no cloning!
    Some(AffirmationData::Borrowed(affirmations_from_file(
        &EMBEDDED_AFFIRMATIONS,
        Some(mood),
    )))
}

pub fn load_custom_affirmations_with_mood<P: AsRef<Path>>(
    path: P,
    mood: &str,
) -> Option<AffirmationData<'static>> {
    let json_str = fs::read_to_string(&path).ok()?;
    Some(AffirmationData::Owned(parse_affirmations(
        &json_str,
        Some(mood),
    )?))
}

/// Mixes affirmations from two moods with a specified probability
/// Returns ominous mood with a chance to append thirsty affirmations
fn mix_moods(
    file: &AffirmationsFile,
    primary_mood: &str,
    secondary_mood: &str,
    probability: f32,
) -> Option<AffirmationsOwned> {
    let primary_set = file.moods.get(primary_mood)?;
    let secondary_set = file.moods.get(secondary_mood)?;

    let mut mixed_positive = primary_set.positive.clone();
    let mut mixed_negative = primary_set.negative.clone();

    // Use fastrand for consistency with the rest of the codebase
    if fastrand::f32() < probability {
        // Append a random affirmation from the secondary mood
        if !secondary_set.positive.is_empty() {
            let idx = fastrand::usize(..secondary_set.positive.len());
            if let Some(secondary_affirmation) = secondary_set.positive.get(idx) {
                // Find a random primary affirmation to append to
                if !mixed_positive.is_empty() {
                    let primary_idx = fastrand::usize(..mixed_positive.len());
                    let _ = std::fmt::Write::write_fmt(
                        &mut mixed_positive[primary_idx],
                        format_args!(" {}", secondary_affirmation),
                    );
                }
            }
        }

        if !secondary_set.negative.is_empty() {
            let idx = fastrand::usize(..secondary_set.negative.len());
            if let Some(secondary_affirmation) = secondary_set.negative.get(idx) {
                // Find a random primary affirmation to append to
                if !mixed_negative.is_empty() {
                    let primary_idx = fastrand::usize(..mixed_negative.len());
                    let _ = std::fmt::Write::write_fmt(
                        &mut mixed_negative[primary_idx],
                        format_args!(" {}", secondary_affirmation),
                    );
                }
            }
        }
    }

    Some(AffirmationsOwned {
        positive: mixed_positive,
        negative: mixed_negative,
    })
}

/// Load affirmations with optional mood mixing support
/// When mood_mixing is enabled and mood is "ominous", it has a 20% chance to
/// mix with thirsty
pub fn load_affirmations_with_mood_mixing(
    mood: &str,
    enable_mixing: bool,
) -> Option<AffirmationData<'static>> {
    if enable_mixing && mood == "ominous" {
        // Mix ominous with thirsty (20% chance)
        if let Some(mixed) = mix_moods(&EMBEDDED_AFFIRMATIONS, "ominous", "thirsty", 0.2) {
            return Some(AffirmationData::Owned(mixed));
        }
    }

    // Fall back to regular mood loading
    load_affirmations_with_mood(mood)
}

/// Load custom affirmations with optional mood mixing support
pub fn load_custom_affirmations_with_mood_mixing<P: AsRef<Path>>(
    path: P,
    mood: &str,
    enable_mixing: bool,
) -> Option<AffirmationData<'static>> {
    if enable_mixing && mood == "ominous" {
        // Load the custom file and attempt mixing
        let json_str = fs::read_to_string(&path).ok()?;
        let file: AffirmationsFile = serde_json::from_str(&json_str).ok()?;

        // Mix ominous with thirsty (20% chance)
        if let Some(mixed) = mix_moods(&file, "ominous", "thirsty", 0.2) {
            return Some(AffirmationData::Owned(mixed));
        }
    }

    // Fall back to regular mood loading
    load_custom_affirmations_with_mood(path, mood)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embedded_affirmations_some() {
        let affirmations = load_affirmations_with_mood("chill");

        // Expect: loaded affirmations to be Some(_)
        assert!(
            affirmations.is_some(),
            "expected embedded affirmations to be Some(_)"
        );
    }

    #[test]
    fn test_embedded_affirmations_content() {
        let affirmations =
            load_affirmations_with_mood("chill").expect("embedded JSON didn't parse");

        // Expect: at least one positive and one negative
        assert!(
            !affirmations.positive().is_empty(),
            "expected at least one positive affirmation"
        );
        assert!(
            !affirmations.negative().is_empty(),
            "expected at least one negative affirmation"
        );

        // Expect: one specific affirmation from the ../assets/affirmations.json
        assert!(affirmations
            .positive()
            .iter()
            .any(|s| s == "*boops your nose* {emotes}"));
    }

    #[test]
    fn load_custom_affirmations_ok() {
        let aff = load_affirmations_with_mood("chill").unwrap();

        // Expect: one valid positive and negative affirmations
        assert!(aff
            .positive()
            .iter()
            .any(|s| s == "you're such a smart cookie~ {emotes}"));
        assert!(aff
            .negative()
            .iter()
            .any(|s| s == "{roles} believes in you~ {emotes}"));
    }

    #[test]
    fn load_custom_affirmations_missing_file() {
        let path = "/nonexistent/path/to/file";
        let aff = load_custom_affirmations_with_mood(path, "chill");

        // Expect: None for nonexistent path
        assert!(aff.is_none(), "expected None for bad path, got {:#?}", aff);
    }

    #[test]
    fn test_mood_ominous() {
        let affirmations =
            load_affirmations_with_mood("ominous").expect("failed to load ominous mood");

        // Expect: at least one positive and one negative
        assert!(
            !affirmations.positive().is_empty(),
            "expected at least one positive affirmation in ominous"
        );
        assert!(
            !affirmations.negative().is_empty(),
            "expected at least one negative affirmation in ominous"
        );

        // Expect: ominous-specific content
        assert!(
            affirmations
                .positive()
                .iter()
                .any(|s| s.contains("aeons") || s.contains("feared")),
            "expected ominous-themed positive affirmations"
        );
    }

    #[test]
    fn test_mood_thirsty() {
        let affirmations =
            load_affirmations_with_mood("thirsty").expect("failed to load thirsty mood");

        // Expect: at least one positive and one negative
        assert!(
            !affirmations.positive().is_empty(),
            "expected at least one positive affirmation in thirsty"
        );
        assert!(
            !affirmations.negative().is_empty(),
            "expected at least one negative affirmation in thirsty"
        );
    }

    #[test]
    fn test_mood_fallback_on_invalid() {
        let affirmations =
            load_affirmations_with_mood("nonexistent").expect("should fallback to default");

        // Expect: falls back to default positive/negative arrays
        assert!(
            !affirmations.positive().is_empty(),
            "expected fallback positive affirmations"
        );
        assert!(
            !affirmations.negative().is_empty(),
            "expected fallback negative affirmations"
        );
    }

    #[test]
    fn test_mood_mixing_disabled() {
        let affirmations = load_affirmations_with_mood_mixing("ominous", false)
            .expect("should load ominous mood without mixing");

        // Expect: should be the same as regular ominous mood loading
        let regular_ominous =
            load_affirmations_with_mood("ominous").expect("should load regular ominous mood");

        assert_eq!(
            affirmations.positive().len(),
            regular_ominous.positive().len()
        );
        assert_eq!(
            affirmations.negative().len(),
            regular_ominous.negative().len()
        );
    }

    #[test]
    fn test_mood_mixing_non_ominous() {
        let affirmations =
            load_affirmations_with_mood_mixing("chill", true).expect("should load chill mood");

        // Expect: non-ominous moods should not be affected by mixing
        let regular_chill =
            load_affirmations_with_mood("chill").expect("should load regular chill mood");

        assert_eq!(
            affirmations.positive().len(),
            regular_chill.positive().len()
        );
        assert_eq!(
            affirmations.negative().len(),
            regular_chill.negative().len()
        );
    }

    #[test]
    fn test_mood_mixing_ominous_enabled() {
        // Since mood mixing is probabilistic, we test the function exists and returns
        // data
        let affirmations = load_affirmations_with_mood_mixing("ominous", true)
            .expect("should load ominous mood with mixing enabled");

        // Expect: should have at least one positive and negative affirmation
        assert!(
            !affirmations.positive().is_empty(),
            "expected at least one positive affirmation with mixing"
        );
        assert!(
            !affirmations.negative().is_empty(),
            "expected at least one negative affirmation with mixing"
        );

        // Note: We can't test the exact mixing behavior due to randomness,
        // but we verify the function works and returns valid data
    }

    #[test]
    fn test_mix_moods_function() {
        // Test the internal mix_moods function directly
        let result = mix_moods(&EMBEDDED_AFFIRMATIONS, "ominous", "thirsty", 1.0);
        assert!(
            result.is_some(),
            "mix_moods should return Some when both moods exist"
        );

        let mixed = result.unwrap();
        assert!(
            !mixed.positive.is_empty(),
            "mixed positive should not be empty"
        );
        assert!(
            !mixed.negative.is_empty(),
            "mixed negative should not be empty"
        );

        // Test with probability 0.0 - should return original affirmations
        let no_mix = mix_moods(&EMBEDDED_AFFIRMATIONS, "ominous", "thirsty", 0.0);
        assert!(
            no_mix.is_some(),
            "mix_moods should work with 0.0 probability"
        );
    }

    #[test]
    fn test_custom_mood_mixing() {
        // Create a minimal test JSON file in memory
        let test_json = r#"{
            "moods": {
                "test_primary": {
                    "positive": ["primary positive"],
                    "negative": ["primary negative"]
                },
                "test_secondary": {
                    "positive": ["secondary positive"],
                    "negative": ["secondary negative"]
                }
            }
        }"#;

        let file: AffirmationsFile =
            serde_json::from_str(test_json).expect("test JSON should parse");

        let result = mix_moods(&file, "test_primary", "test_secondary", 1.0);
        assert!(result.is_some(), "mixing should work with test data");

        let mixed = result.unwrap();
        assert_eq!(
            mixed.positive.len(),
            1,
            "should have one positive affirmation"
        );
        assert_eq!(
            mixed.negative.len(),
            1,
            "should have one negative affirmation"
        );
    }
}
