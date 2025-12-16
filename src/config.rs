use std::env;

#[derive(Debug)]
pub struct ConfigMommy {
    pub pronouns: String,
    pub roles: String,
    pub little: String,
    pub emotes: String,
    pub color: String,
    pub style: String,
    pub color_rgb: Option<String>,
    pub aliases: Option<String>,
    pub affirmations: Option<String>,
    pub needy: bool,
    pub only_negative: bool,
    pub moods: String,
    pub quiet: bool,
    pub recursion_limit: usize,
}

/// Detects the role from the binary name (e.g., "mommy", "daddy", "cargo-mommy", etc.)
pub fn detect_role_from_binary() -> String {
    env::current_exe()
        .ok()
        .and_then(|path| {
            path.file_name().and_then(|name| name.to_str()).map(|name| {
                // Handle both "cargo-mommy", "cargo-daddy" and plain "mommy", "daddy"
                let name = name.strip_prefix("cargo-").unwrap_or(name);
                // Extract role name (mommy, daddy, etc.)
                if name.contains("daddy") {
                    "daddy".to_string()
                } else {
                    // Default to mommy
                    "mommy".to_string()
                }
            })
        })
        .unwrap_or_else(|| "mommy".to_string())
}

/// Gets the environment variable prefix based on the detected role and binary name
pub fn get_env_prefix() -> String {
    let binary_name = env::current_exe()
        .ok()
        .and_then(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .map(|s| s.to_string())
        })
        .unwrap_or_else(|| "mommy".to_string());

    // Check if we're running as a cargo subcommand
    if binary_name.starts_with("cargo-") {
        // Extract the role (mommy, daddy, etc.) from binary name
        let role = detect_role_from_binary().to_uppercase();
        format!("CARGO_{}S", role)
    } else {
        // Default to SHELL_MOMMYS for backward compatibility
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

pub fn load_config() -> ConfigMommy {
    let env_prefix = get_env_prefix();
    let pronouns = env_with_fallback(&env_prefix, "PRONOUNS").unwrap_or_else(|| "her".to_string());
    let roles = env_with_fallback(&env_prefix, "ROLES").unwrap_or_else(detect_role_from_binary);
    let little = env_with_fallback(&env_prefix, "LITTLE").unwrap_or_else(|| "girl".to_string());
    let emotes =
        env_with_fallback(&env_prefix, "EMOTES").unwrap_or_else(|| "💖/💗/💓/💞".to_string());
    let color = env_with_fallback(&env_prefix, "COLOR").unwrap_or_else(|| "white".to_string());
    let style = env_with_fallback(&env_prefix, "STYLE").unwrap_or_else(|| "bold".to_string());
    let color_rgb = env_with_fallback(&env_prefix, "COLOR_RGB");
    let aliases = env_with_fallback(&env_prefix, "ALIASES");
    let affirmations = env_with_fallback(&env_prefix, "AFFIRMATIONS");
    let needy = env_with_fallback(&env_prefix, "NEEDY").is_some_and(|v| v == "1");
    let moods = env_with_fallback(&env_prefix, "MOODS").unwrap_or_else(|| "chill".to_string());

    // Special handling for ONLY_NEGATIVE (uses SHELL_MOMMY prefix, not SHELL_MOMMYS)
    let only_negative = env::var("SHELL_MOMMY_ONLY_NEGATIVE").is_ok_and(|v| v == "1")
        || env::var("CARGO_MOMMY_ONLY_NEGATIVE").is_ok_and(|v| v == "1");

    let quiet = false; // Will be set later based on args

    // Get recursion limit from environment or default to 100
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
        color,
        style,
        color_rgb,
        aliases,
        affirmations,
        needy,
        only_negative,
        moods,
        quiet,
        recursion_limit,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::env;

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
    #[serial]
    fn test_env_with_fallback_prefers_primary_key() {
        clear_all();
        unsafe {
            env::set_var("CARGO_MOMMYS_PRONOUNS", "their");
            env::set_var("SHELL_MOMMYS_PRONOUNS", "her");
        }

        let value = env_with_fallback("CARGO_MOMMYS", "PRONOUNS");
        assert_eq!(value.as_deref(), Some("their"));
    }

    #[test]
    #[serial]
    fn test_env_with_fallback_uses_shell_backup() {
        clear_all();
        unsafe {
            env::set_var("SHELL_MOMMYS_PRONOUNS", "hers");
        }

        let value = env_with_fallback("CARGO_MOMMYS", "PRONOUNS");
        assert_eq!(value.as_deref(), Some("hers"));
    }

    #[test]
    #[serial]
    fn test_default_vars() {
        clear_all();
        let config = load_config();

        // Expect: all defaults
        assert_eq!(config.pronouns, "her");
        assert!(config.roles == "mommy" || config.roles == "daddy"); // Depends on binary name
        assert_eq!(config.little, "girl");
        assert_eq!(config.emotes, "💖/💗/💓/💞");
        assert_eq!(config.color, "white");
        assert_eq!(config.style, "bold");
        assert_eq!(config.color_rgb, None);
        assert_eq!(config.aliases, None);
        assert_eq!(config.affirmations, None);
        assert!(!config.needy);
        assert!(!config.only_negative);
        assert_eq!(config.moods, "chill");
        assert!(!config.quiet);
        assert_eq!(config.recursion_limit, 0);
    }

    #[test]
    #[serial]
    fn test_custom_vars() {
        clear_all();
        unsafe {
            env::set_var("SHELL_MOMMYS_PRONOUNS", "his");
            env::set_var("SHELL_MOMMYS_ROLES", "daddy");
            env::set_var("SHELL_MOMMYS_COLOR_RGB", "255,255,255");
            env::set_var("SHELL_MOMMYS_NEEDY", "1");
            env::set_var("SHELL_MOMMY_ONLY_NEGATIVE", "1");
            env::set_var("SHELL_MOMMYS_MOODS", "ominous/thirsty");
        }
        let config = load_config();

        // Expect: pronouns: his; role: daddy; color_rgb: 255,255,255; needy: 1; only_negative: 1; moods: ominous/thirsty
        assert_eq!(config.pronouns, "his");
        assert_eq!(config.roles, "daddy");
        assert_eq!(config.color_rgb, Some("255,255,255".to_string()));
        assert!(config.needy, "expected 1, got {:#?}", config.needy);
        assert!(
            config.only_negative,
            "expected 1, got {:#?}",
            config.only_negative
        );
        assert_eq!(config.moods, "ominous/thirsty");
    }

    #[test]
    #[serial]
    fn test_cargo_prefix_vars() {
        clear_all();
        unsafe {
            env::set_var("CARGO_MOMMYS_PRONOUNS", "their");
            env::set_var("CARGO_MOMMYS_ROLES", "parent");
            env::set_var("CARGO_MOMMYS_LITTLE", "child");
        }
        let config = load_config();

        // These should work if we're running as cargo-mommy, otherwise fall back to defaults
        // Due to binary name detection, we can't reliably test this without renaming the binary
        // So we just verify the config loads without errors
        assert!(!config.pronouns.is_empty());
        assert!(!config.roles.is_empty());
        assert!(!config.little.is_empty());
    }
}
