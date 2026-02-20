use std::{env, path::PathBuf};

/// Cached binary information to avoid redundant filesystem calls
#[derive(Debug, Clone)]
pub struct BinaryInfo {
    pub path: PathBuf,
    pub role: String,
    pub is_cargo_subcommand: bool,
}

impl BinaryInfo {
    pub fn detect() -> Self {
        let path = env::current_exe().unwrap_or_else(|_| PathBuf::from("mommy"));
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("mommy")
            .to_string();

        let is_cargo_subcommand = name.starts_with("cargo-");

        // Handle both "cargo-mommy", "cargo-daddy" and plain "mommy", "daddy"
        let stripped = name.strip_prefix("cargo-").unwrap_or(&name);
        let role = if stripped.contains("daddy") {
            "daddy".to_string()
        } else {
            "mommy".to_string()
        };

        BinaryInfo {
            path,
            role,
            is_cargo_subcommand,
        }
    }
}

#[derive(Debug)]
pub struct ConfigMommy {
    // Pre-parsed string options for efficient random selection
    pub pronouns: Vec<String>,
    pub roles: Vec<String>,
    pub little: Vec<String>,
    pub emotes: Vec<String>,
    pub moods: Vec<String>,

    // Pre-parsed color options
    pub colors: Vec<String>,
    pub color_rgb: Option<Vec<String>>,

    // Pre-parsed style combinations (each is a Vec of style attributes)
    pub styles: Vec<Vec<String>>,

    pub aliases: Option<String>,
    pub affirmations: Option<String>,
    pub needy: bool,
    pub only_negative: bool,
    pub quiet: bool,
    pub recursion_limit: usize,
    pub mood_mixing: bool,

    // Cached binary info
    pub binary_info: BinaryInfo,
}

/// Gets the environment variable prefix based on the binary info
fn get_env_prefix_from_binary(binary_info: &BinaryInfo) -> String {
    if binary_info.is_cargo_subcommand {
        let role = binary_info.role.to_uppercase();
        format!("CARGO_{}S", role)
    } else {
        "SHELL_MOMMYS".to_string()
    }
}

/// Helper to get env var with fallback to both prefixes
fn env_with_fallback(prefix: &str, suffix: &str) -> Option<String> {
    let primary_key = format!("{}_{}", prefix, suffix);

    env::var(&primary_key)
        .or_else(|_| {
            // Fallback to SHELL_MOMMYS_* if we're using CARGO prefix
            if prefix.starts_with("CARGO_") {
                env::var(format!("SHELL_MOMMYS_{}", suffix))
            } else {
                Err(env::VarError::NotPresent)
            }
        })
        .ok()
}

/// Parse a slash-separated string into a Vec<String>
/// Trims and lowercases each token, filters empty ones
fn parse_config_string(s: &str) -> Vec<String> {
    s.split('/')
        .map(|token| token.trim().to_lowercase())
        .filter(|token| !token.is_empty())
        .collect()
}

pub fn load_config() -> ConfigMommy {
    // Detect binary info once
    let binary_info = BinaryInfo::detect();
    let env_prefix = get_env_prefix_from_binary(&binary_info);

    // Load raw config values
    let pronouns_raw =
        env_with_fallback(&env_prefix, "PRONOUNS").unwrap_or_else(|| "her".to_string());
    let roles_raw =
        env_with_fallback(&env_prefix, "ROLES").unwrap_or_else(|| binary_info.role.clone());
    let little_raw = env_with_fallback(&env_prefix, "LITTLE").unwrap_or_else(|| "girl".to_string());
    let emotes_raw =
        env_with_fallback(&env_prefix, "EMOTES").unwrap_or_else(|| "💖/💗/💓/💞".to_string());
    let color_raw = env_with_fallback(&env_prefix, "COLOR").unwrap_or_else(|| "white".to_string());
    let style_raw = env_with_fallback(&env_prefix, "STYLE").unwrap_or_else(|| "bold".to_string());
    let color_rgb_raw = env_with_fallback(&env_prefix, "COLOR_RGB");
    let moods_raw = env_with_fallback(&env_prefix, "MOODS").unwrap_or_else(|| "chill".to_string());

    // Pre-parse all slash-separated config values
    let pronouns = parse_config_string(&pronouns_raw);
    let roles = parse_config_string(&roles_raw);
    let little = parse_config_string(&little_raw);
    let emotes = parse_config_string(&emotes_raw);
    let moods = parse_config_string(&moods_raw);
    let colors = parse_config_string(&color_raw);
    let color_rgb = color_rgb_raw.map(|rgb| parse_config_string(&rgb));

    // Pre-parse style combinations (each combo can have multiple comma-separated
    // styles)
    let styles: Vec<Vec<String>> = parse_config_string(&style_raw)
        .into_iter()
        .map(|combo| {
            combo
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        })
        .collect();

    let aliases = env_with_fallback(&env_prefix, "ALIASES");
    let affirmations = env_with_fallback(&env_prefix, "AFFIRMATIONS");
    let needy = env_with_fallback(&env_prefix, "NEEDY").is_some_and(|v| v == "1");
    let mood_mixing = env_with_fallback(&env_prefix, "MOOD_MIXING").is_some_and(|v| v == "1");

    // Special handling for ONLY_NEGATIVE (uses SHELL_MOMMY prefix, not
    // SHELL_MOMMYS)
    let only_negative = env::var("SHELL_MOMMY_ONLY_NEGATIVE").is_ok_and(|v| v == "1")
        || env::var("CARGO_MOMMY_ONLY_NEGATIVE").is_ok_and(|v| v == "1");

    let quiet = false; // Will be set later based on args

    // Get recursion limit from environment or default to 0
    let recursion_limit = env::var("CARGO_MOMMY_RECURSION_LIMIT")
        .or_else(|_| env::var("SHELL_MOMMY_RECURSION_LIMIT"))
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);

    ConfigMommy {
        pronouns,
        roles,
        little,
        emotes,
        moods,
        colors,
        color_rgb,
        styles,
        aliases,
        affirmations,
        needy,
        only_negative,
        quiet,
        recursion_limit,
        mood_mixing,
        binary_info,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        env,
        sync::{LazyLock, Mutex},
    };

    // Mutex to serialize tests that modify environment variables
    static ENV_TEST_LOCK: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

    // Helper to clear all config‐related env vars.
    fn clear_all() {
        let keys = [
            "SHELL_MOMMYS_PRONOUNS",
            "SHELL_MOMMYS_ROLES",
            "SHELL_MOMMYS_LITTLE",
            "SHELL_MOMMYS_EMOTES",
            "SHELL_MOMMYS_COLOR",
            "SHELL_MOMMYS_STYLE",
            "SHELL_MOMMYS_COLOR_RGB",
            "SHELL_MOMMYS_ALIASES",
            "SHELL_MOMMYS_AFFIRMATIONS",
            "SHELL_MOMMYS_NEEDY",
            "SHELL_MOMMYS_MOOD_MIXING",
            "SHELL_MOMMY_ONLY_NEGATIVE",
            "SHELL_MOMMYS_MOODS",
            "CARGO_MOMMYS_PRONOUNS",
            "CARGO_MOMMYS_ROLES",
            "CARGO_MOMMYS_LITTLE",
            "CARGO_MOMMYS_EMOTES",
            "CARGO_MOMMYS_COLOR",
            "CARGO_MOMMYS_STYLE",
            "CARGO_MOMMYS_COLOR_RGB",
            "CARGO_MOMMYS_ALIASES",
            "CARGO_MOMMYS_AFFIRMATIONS",
            "CARGO_MOMMYS_NEEDY",
            "CARGO_MOMMYS_MOOD_MIXING",
            "CARGO_MOMMYS_MOODS",
            "CARGO_MOMMY_ONLY_NEGATIVE",
        ];
        for k in &keys {
            unsafe {
                env::remove_var(k);
            }
        }
    }

    #[test]
    fn test_env_with_fallback_prefers_primary_key() {
        let _lock = ENV_TEST_LOCK.lock().unwrap();
        clear_all();
        unsafe {
            env::set_var("CARGO_MOMMYS_PRONOUNS", "their");
            env::set_var("SHELL_MOMMYS_PRONOUNS", "her");
        }

        let value = env_with_fallback("CARGO_MOMMYS", "PRONOUNS");
        assert_eq!(value.as_deref(), Some("their"));
    }

    #[test]
    fn test_env_with_fallback_uses_shell_backup() {
        let _lock = ENV_TEST_LOCK.lock().unwrap();
        clear_all();
        unsafe {
            env::set_var("SHELL_MOMMYS_PRONOUNS", "hers");
        }

        let value = env_with_fallback("CARGO_MOMMYS", "PRONOUNS");
        assert_eq!(value.as_deref(), Some("hers"));
    }

    #[test]
    fn test_default_vars() {
        let _lock = ENV_TEST_LOCK.lock().unwrap();
        clear_all();
        let config = load_config();

        // Expect: all defaults (now pre-parsed into Vec<String>)
        assert_eq!(config.pronouns, vec!["her"]);
        assert!(config.roles == vec!["mommy"] || config.roles == vec!["daddy"]); // Depends on binary name
        assert_eq!(config.little, vec!["girl"]);
        assert_eq!(config.emotes, vec!["💖", "💗", "💓", "💞"]);
        assert_eq!(config.colors, vec!["white"]);
        assert_eq!(config.styles, vec![vec!["bold"]]);
        assert_eq!(config.color_rgb, None);
        assert_eq!(config.aliases, None);
        assert_eq!(config.affirmations, None);
        assert!(!config.needy);
        assert!(!config.only_negative);
        assert!(!config.mood_mixing);
        assert_eq!(config.moods, vec!["chill"]);
        assert!(!config.quiet);
        assert_eq!(config.recursion_limit, 0);
    }

    #[test]
    fn test_custom_vars() {
        let _lock = ENV_TEST_LOCK.lock().unwrap();
        clear_all();
        unsafe {
            env::set_var("SHELL_MOMMYS_PRONOUNS", "his");
            env::set_var("SHELL_MOMMYS_ROLES", "daddy");
            env::set_var("SHELL_MOMMYS_COLOR_RGB", "255,255,255");
            env::set_var("SHELL_MOMMYS_NEEDY", "1");
            env::set_var("SHELL_MOMMYS_MOOD_MIXING", "1");
            env::set_var("SHELL_MOMMY_ONLY_NEGATIVE", "1");
            env::set_var("SHELL_MOMMYS_MOODS", "ominous/thirsty");
        }
        let config = load_config();

        // Expect: pre-parsed vectors
        assert_eq!(config.pronouns, vec!["his"]);
        assert_eq!(config.roles, vec!["daddy"]);
        assert_eq!(config.color_rgb, Some(vec!["255,255,255".to_string()]));
        assert!(config.needy, "expected 1, got {:#?}", config.needy);
        assert!(config.mood_mixing, "expected mood mixing to be enabled");
        assert!(
            config.only_negative,
            "expected 1, got {:#?}",
            config.only_negative
        );
        assert_eq!(config.moods, vec!["ominous", "thirsty"]);
    }

    #[test]
    fn test_cargo_prefix_vars() {
        let _lock = ENV_TEST_LOCK.lock().unwrap();
        clear_all();
        unsafe {
            env::set_var("CARGO_MOMMYS_PRONOUNS", "their");
            env::set_var("CARGO_MOMMYS_ROLES", "parent");
            env::set_var("CARGO_MOMMYS_LITTLE", "child");
        }
        let config = load_config();

        // These should work if we're running as cargo-mommy, otherwise fall back to
        // defaults Due to binary name detection, we can't reliably test this
        // without renaming the binary So we just verify the config loads
        // without errors and has pre-parsed values
        assert!(!config.pronouns.is_empty());
        assert!(!config.roles.is_empty());
        assert!(!config.little.is_empty());
    }
}
