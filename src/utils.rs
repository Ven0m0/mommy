use crate::config::ConfigMommy;
use std::io::{self, Write};

/// Deprecated: Config values are now pre-parsed in load_config()
/// This function is kept for backward compatibility
pub fn parse_string(s: &str) -> Vec<String> {
    s.split('/')
        .map(|token| token.trim().to_lowercase())
        .filter(|token| !token.is_empty())
        .collect()
}

/// Pick a random string from a pre-parsed Vec<String>
/// Returns a reference to avoid cloning
fn random_vec_pick(vec: &[String]) -> Option<&str> {
    if vec.is_empty() {
        None
    } else {
        let idx = fastrand::usize(..vec.len());
        Some(&vec[idx])
    }
}

/// Deprecated: Use random_vec_pick with pre-parsed Vec instead
/// This function is kept for backward compatibility
pub fn random_string_pick(input: &str) -> Option<String> {
    let parts = parse_string(input);

    if parts.is_empty() {
        None
    } else {
        let idx = fastrand::usize(..parts.len());
        Some(parts[idx].to_string())
    }
}

pub fn fill_template(template: &str, config: &ConfigMommy) -> String {
    // Pick random values from pre-parsed config vectors
    // Use first element as fallback if vector is somehow empty
    let role = random_vec_pick(&config.roles).unwrap_or("mommy");
    let pronoun = random_vec_pick(&config.pronouns).unwrap_or("her");
    let little = random_vec_pick(&config.little).unwrap_or("girl");
    let emote = random_vec_pick(&config.emotes).unwrap_or("💖");

    template
        .replace("{roles}", role)
        .replace("{pronouns}", pronoun)
        .replace("{little}", little)
        .replace("{emotes}", emote)
}

pub fn graceful_print<T: std::fmt::Display>(s: T) {
    if writeln!(io::stderr(), "{}", s).is_err() {
        std::process::exit(0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::load_config;

    #[test]
    fn test_parse_string() {
        let s = "one/two/three";
        let v = parse_string(s);
        assert_eq!(v, vec!["one", "two", "three"]);
    }

    #[test]
    fn test_parse_string_with_spaces() {
        let s = "one    / two/      three ";
        let v = parse_string(s);
        assert_eq!(v, vec!["one", "two", "three"]);
    }

    #[test]
    fn test_parse_empty_string() {
        let s = "///   / ";
        let v = parse_string(s);
        assert!(v.is_empty());
    }

    #[test]
    fn test_pick_empty_string() {
        assert!(random_string_pick("").is_none());
        assert!(random_string_pick("   ///   ").is_none());
    }

    #[test]
    fn test_pick_some_string() {
        fastrand::seed(42); // Making outputs predictable: https://blog.orhun.dev/zero-deps-random-in-rust/
        let pick = random_string_pick("one/two/three").unwrap();
        let pick2 = random_string_pick("one/two/three").unwrap();
        let pick3 = random_string_pick("one/two/three").unwrap();

        assert_eq!(pick, "three");
        assert_eq!(pick2, "two");
        assert_eq!(pick3, "three");
    }

    #[test]
    fn test_fill_template() {
        fastrand::seed(42);
        let mut config = load_config();
        // Config now has pre-parsed Vec<String> fields
        config.roles = vec!["daddy".to_string(), "mommy".to_string()];
        config.pronouns = vec!["his".to_string(), "her".to_string()];
        config.little = vec!["baby".to_string()];
        config.emotes = vec!["❤️‍🔥".to_string(), "🤓".to_string()];

        let template = fill_template(
            "{roles} thinks {pronouns} {little} earned a big hug~ {emotes}",
            &config,
        );
        assert_eq!(template, "mommy thinks his baby earned a big hug~ ❤️‍🔥");
    }
}
