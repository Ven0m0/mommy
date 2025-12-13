use std::fs;
use std::path::Path;
use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MoodSet {
    pub positive: Vec<String>,
    pub negative: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct AffirmationsFile {
    #[serde(default)]
    pub moods: HashMap<String, MoodSet>,
    pub positive: Vec<String>,
    pub negative: Vec<String>,
}

#[derive(Debug)]
pub struct Affirmations {
    pub positive: Vec<String>,
    pub negative: Vec<String>,
}

/// Loads the embedded affirmations without mood support.
/// 
/// This function is kept for backward compatibility with code that doesn't use the mood system.
/// For new code, prefer `load_affirmations_with_mood` which supports mood selection.
#[allow(dead_code)]
pub fn load_affirmations() -> Option<Affirmations> {
    let json_str = include_str!("../assets/affirmations.json");
    let file: AffirmationsFile = serde_json::from_str(json_str).ok()?;
    Some(Affirmations {
        positive: file.positive,
        negative: file.negative,
    })
}

pub fn load_affirmations_with_mood(mood: &str) -> Option<Affirmations> {
    let json_str = include_str!("../assets/affirmations.json");
    let mut file: AffirmationsFile = serde_json::from_str(json_str).ok()?;
    
    if let Some(mut mood_set) = file.moods.remove(mood) {
        Some(Affirmations {
            positive: std::mem::take(&mut mood_set.positive),
            negative: std::mem::take(&mut mood_set.negative),
        })
    } else {
        Some(Affirmations {
            positive: file.positive,
            negative: file.negative,
        })
    }
}

/// Loads custom affirmations from a file without mood support.
/// 
/// This function is kept for backward compatibility with code that doesn't use the mood system.
/// For new code, prefer `load_custom_affirmations_with_mood` which supports mood selection.
#[allow(dead_code)]
pub fn load_custom_affirmations<P: AsRef<Path>>(path: P) -> Option<Affirmations> {
    let json_str = fs::read_to_string(&path).ok()?;
    let file: AffirmationsFile = serde_json::from_str(&json_str).ok()?;
    Some(Affirmations {
        positive: file.positive,
        negative: file.negative,
    })
}

pub fn load_custom_affirmations_with_mood<P: AsRef<Path>>(path: P, mood: &str) -> Option<Affirmations> {
    let json_str = fs::read_to_string(&path).ok()?;
    let mut file: AffirmationsFile = serde_json::from_str(&json_str).ok()?;
    
    if let Some(mut mood_set) = file.moods.remove(mood) {
        Some(Affirmations {
            positive: std::mem::take(&mut mood_set.positive),
            negative: std::mem::take(&mut mood_set.negative),
        })
    } else {
        Some(Affirmations {
            positive: file.positive,
            negative: file.negative,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embedded_affirmations_some() {
        let affirmations = load_affirmations_with_mood("chill");
        
        // Expect: loaded affirmations to be Some(_)
        assert!(affirmations.is_some(), "expected embedded affirmations to be Some(_)");
    }

    #[test]
    fn test_embedded_affirmations_content() {
        let affirmations = load_affirmations_with_mood("chill").expect("embedded JSON didn't parse");

        // Expect: at least one positive and one negative
        assert!(!affirmations.positive.is_empty(), "expected at least one positive affirmation");
        assert!(!affirmations.negative.is_empty(), "expected at least one negative affirmation");

        // Expect: one specific affirmation from the ../assets/affirmations.json
        assert!(affirmations.positive.iter().any(|s| s == "*boops your nose* {emotes}"));
    }

    #[test]
    fn load_custom_affirmations_ok() {
        let aff = load_affirmations_with_mood("chill").unwrap();

        // Expect: one valid positive and negative affirmations
        assert!(aff.positive.iter().any(|s| s == "you're such a smart cookie~ {emotes}"));
        assert!(aff.negative.iter().any(|s| s == "{roles} believes in you~ {emotes}"));
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
        let affirmations = load_affirmations_with_mood("ominous").expect("failed to load ominous mood");

        // Expect: at least one positive and one negative
        assert!(!affirmations.positive.is_empty(), "expected at least one positive affirmation in ominous");
        assert!(!affirmations.negative.is_empty(), "expected at least one negative affirmation in ominous");

        // Expect: ominous-specific content
        assert!(affirmations.positive.iter().any(|s| s.contains("aeons") || s.contains("feared")), 
                "expected ominous-themed positive affirmations");
    }

    #[test]
    fn test_mood_thirsty() {
        let affirmations = load_affirmations_with_mood("thirsty").expect("failed to load thirsty mood");

        // Expect: at least one positive and one negative
        assert!(!affirmations.positive.is_empty(), "expected at least one positive affirmation in thirsty");
        assert!(!affirmations.negative.is_empty(), "expected at least one negative affirmation in thirsty");
    }

    #[test]
    fn test_mood_fallback_on_invalid() {
        let affirmations = load_affirmations_with_mood("nonexistent").expect("should fallback to default");

        // Expect: falls back to default positive/negative arrays
        assert!(!affirmations.positive.is_empty(), "expected fallback positive affirmations");
        assert!(!affirmations.negative.is_empty(), "expected fallback negative affirmations");
    }
}
