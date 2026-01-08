use crate::config::ConfigMommy;
use std::io::{self, Write};

/// Pick a random string from a pre-parsed Vec<String>
/// Returns a reference to avoid cloning
#[inline]
pub fn random_vec_pick(vec: &[String]) -> Option<&str> {
    if vec.is_empty() {
        None
    } else {
        let idx = fastrand::usize(..vec.len());
        Some(&vec[idx])
    }
}

#[inline]
pub fn fill_template(template: &str, config: &ConfigMommy) -> String {
    // Pick random values from pre-parsed config vectors
    // Use first element as fallback if vector is somehow empty
    let role = random_vec_pick(&config.roles).unwrap_or("mommy");
    let pronoun = random_vec_pick(&config.pronouns).unwrap_or("her");
    let little = random_vec_pick(&config.little).unwrap_or("girl");
    let emote = random_vec_pick(&config.emotes).unwrap_or("💖");

    // Single-pass replacement to avoid intermediate allocations
    // Pre-allocate with extra capacity for replacements
    let mut result = String::with_capacity(template.len() + 20);
    let mut last_end = 0;
    let bytes = template.as_bytes();

    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'{' {
            // Add everything before this '{', converting newlines to spaces
            for ch in template[last_end..i].chars() {
                if ch == '\n' {
                    result.push(' ');
                } else {
                    result.push(ch);
                }
            }

            // Check which placeholder this is
            let remaining = &template[i..];
            if remaining.starts_with("{roles}") {
                result.push_str(role);
                i += 7;
                last_end = i;
            } else if remaining.starts_with("{pronouns}") {
                result.push_str(pronoun);
                i += 10;
                last_end = i;
            } else if remaining.starts_with("{little}") {
                result.push_str(little);
                i += 8;
                last_end = i;
            } else if remaining.starts_with("{emotes}") {
                result.push_str(emote);
                i += 8;
                last_end = i;
            } else {
                // Not a recognized placeholder, keep the '{'
                i += 1;
            }
        } else {
            i += 1;
        }
    }

    // Add any remaining text after the last replacement, converting newlines to spaces
    for ch in template[last_end..].chars() {
        if ch == '\n' {
            result.push(' ');
        } else {
            result.push(ch);
        }
    }

    result
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
