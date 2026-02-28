use crate::{
    affirmations::{
        load_affirmations_with_mood_mixing, load_custom_affirmations_with_mood_mixing,
        AffirmationData,
    },
    color::random_style_pick,
    config::{load_config, ConfigMommy},
    utils::{fill_template, graceful_print, random_vec_pick},
};
use owo_colors::OwoColorize;
use std::{
    env,
    process::{exit, Command},
};

const RECURSION_LIMIT: usize = 100;

#[inline]
fn choose_template<'a>(json_template: Option<&'a [String]>, default_template: &'a str) -> &'a str {
    match json_template {
        Some(templates) if !templates.is_empty() => {
            let idx = fastrand::usize(..templates.len());
            templates[idx].as_str()
        }
        _ => default_template,
    }
}

/// Check if quiet mode is enabled from command line arguments
fn is_quiet_mode_enabled(args: &[String]) -> bool {
    args.iter().any(|arg| arg == "--quiet" || arg == "-q")
}

/// Check if the command contains "i mean" for role transformation
fn check_role_transformation(args: &[String]) -> Option<&str> {
    // Look for pattern: "mommy i mean daddy" or similar
    for i in 0..args.len().saturating_sub(2) {
        if args[i] == "i" && args[i + 1] == "mean" && i + 2 < args.len() {
            return Some(&args[i + 2]);
        }
    }
    None
}

/// Perform role transformation by copying the binary
fn perform_role_transformation(
    new_role: &str,
    binary_info: &crate::config::BinaryInfo,
) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;

    let parent = binary_info
        .path
        .parent()
        .ok_or("Cannot get parent directory")?;

    // Determine the new binary name
    let suffix = std::env::consts::EXE_SUFFIX;
    let new_name = if binary_info.is_cargo_subcommand {
        format!("cargo-{}{}", new_role, suffix)
    } else {
        format!("{}{}", new_role, suffix)
    };

    let new_path = parent.join(&new_name);

    // Copy the binary
    fs::copy(&binary_info.path, &new_path)?;

    // Make it executable (Unix only)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&new_path)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&new_path, perms)?;
    }

    println!("Created new binary: {}", new_path.display());
    println!("You can now use: {}", new_name);

    Ok(())
}

fn execute_command(
    config: &ConfigMommy,
    filtered_args: &[&str],
) -> Result<i32, Box<dyn std::error::Error>> {
    if config.needy {
        let code_str = filtered_args.first().ok_or_else(|| "Missing exit code".to_string())?;
        code_str.parse().map_err(|_| {
            format!(
                "Invalid exit code '{}'. Expected a number (e.g., 0 or 1)",
                code_str
            )
            .into()
        })
    } else if config.binary_info.is_cargo_subcommand {
        // Running as cargo subcommand - execute cargo with the provided args
        if filtered_args.is_empty() {
            eprintln!("No cargo command provided");
            exit(1);
        }

        // Increment recursion counter
        let new_recursion = config.recursion_limit + 1;

        let status = Command::new("cargo")
            .args(filtered_args)
            .env("CARGO_MOMMY_RECURSION_LIMIT", new_recursion.to_string())
            .status()?;

        Ok(status.code().unwrap_or(1))
    } else {
        // Running as shell command wrapper
        // Direct join without intermediate Vec allocation
        let raw_command = filtered_args.join(" ");
        let run_command = if let Some(ref aliases_path) = config.aliases {
            // Removed unnecessary eval; execute directly instead to avoid extra shell
            // parsing
            format!(
                "shopt -s expand_aliases; source \"{}\"; {}",
                aliases_path, raw_command
            )
        } else {
            raw_command
        };

        let status = Command::new("bash").arg("-c").arg(&run_command).status()?;

        Ok(status.code().unwrap_or(1))
    }
}

#[cfg(feature = "beg")]
fn handle_begging(
    command_args: &[String],
    config: &ConfigMommy,
) -> Result<(), Box<dyn std::error::Error>> {
    let has_please = command_args.iter().any(|arg| arg == "please");
    let mut state = crate::state::State::load()?;
    if state.mood == crate::state::Mood::Angry {
        if has_please {
            state.mood = crate::state::Mood::Chill;
            if let Err(e) = state.save() {
                eprintln!("mommy failed to remember how she feels: {}", e);
            }
            let output = fill_template("{roles} forgives {pronouns} {little}~ {emotes}", config);
            let styled_output = output.style(random_style_pick(config));
            graceful_print(styled_output);
        } else {
            let output = fill_template(
                "{roles} is waiting for {pronouns} {little} to say please~ {emotes}",
                config,
            );
            let styled_output = output.style(random_style_pick(config));
            graceful_print(styled_output);
            exit(1);
        }
    }
    Ok(())
}

#[cfg(feature = "beg")]
fn update_begging_state(exit_code: i32) -> Result<(), Box<dyn std::error::Error>> {
    let mut state = crate::state::State::load()?;
    state.mood = if exit_code == 0 {
        crate::state::Mood::Chill
    } else {
        crate::state::Mood::Angry
    };
    if let Err(e) = state.save() {
        eprintln!("mommy failed to remember how she feels: {}", e);
    }
    Ok(())
}

fn print_affirmation(
    exit_code: i32,
    config: &ConfigMommy,
) -> Result<(), Box<dyn std::error::Error>> {
    // Skip output if quiet mode is enabled
    if config.quiet {
        return Ok(());
    }

    // Optimization: If the command succeeded and we only want to show negative
    // affirmations, we can skip loading affirmations entirely.
    if exit_code == 0 && config.only_negative {
        return Ok(());
    }
    // Use pre-parsed moods vector
    let selected_mood = random_vec_pick(&config.moods).unwrap_or("chill");

    let affirmations: Option<AffirmationData> = if let Some(ref path) = config.affirmations {
        load_custom_affirmations_with_mood_mixing(path, selected_mood, config.mood_mixing)
    } else {
        load_affirmations_with_mood_mixing(selected_mood, config.mood_mixing)
    };

    // Use const str instead of Vec allocation
    const AFFIRMATIONS_ERROR: &str = "{roles} failed to load any affirmations, {little}~ {emotes}";

    let (template, _affirmation_type) = match (exit_code == 0, config.only_negative) {
        (true, false) => (
            choose_template(
                affirmations.as_ref().map(|aff| aff.positive()),
                AFFIRMATIONS_ERROR,
            ),
            "positive",
        ),
        (false, _) => (
            choose_template(
                affirmations.as_ref().map(|aff| aff.negative()),
                AFFIRMATIONS_ERROR,
            ),
            "negative",
        ),
        _ => return Ok(()),
    };

    let output = fill_template(template, config);
    let styled_output = output.style(random_style_pick(config));
    graceful_print(styled_output);

    Ok(())
}

pub fn mommy() -> Result<i32, Box<dyn std::error::Error>> {
    let mut config = load_config();
    let is_cargo_command = config.binary_info.is_cargo_subcommand;

    // Check recursion limit
    if config.recursion_limit >= RECURSION_LIMIT {
        eprintln!("Recursion limit exceeded! Mommy is stuck in a loop~");
        return Ok(2); // Special exit code for recursion overflow
    }

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        let role = &config.binary_info.role;
        let usage = if is_cargo_command {
            format!("cargo {} <cargo-command> [args...]", role)
        } else if config.needy {
            format!("{} <exit_code>", args[0])
        } else {
            format!("{} <command> [args ...]", args[0])
        };
        eprintln!("Usage: {}", usage);
        exit(1);
    }

    // Check for quiet mode
    config.quiet = is_quiet_mode_enabled(&args);

    // Check for role transformation
    if let Some(new_role) = check_role_transformation(&args) {
        perform_role_transformation(new_role, &config.binary_info)?;
        return Ok(0);
    }

    // Skip the binary name for processing
    let mut command_args = &args[1..];

    // If running as cargo subcommand, skip "cargo" if it's the first arg
    if is_cargo_command && !command_args.is_empty() && command_args[0] == "cargo" {
        command_args = &command_args[1..];
    }

    // Handle "please" for begging mode (if enabled)
    #[cfg(feature = "beg")]
    handle_begging(command_args, &config)?;

    // Filter out "please" and convert to &str in a single pass
    let filtered_args: Vec<&str> = command_args
        .iter()
        .filter(|arg| *arg != "please")
        .map(|s| s.as_str())
        .collect();

    let exit_code = execute_command(&config, &filtered_args)?;

    // Update begging state (if enabled)
    #[cfg(feature = "beg")]
    update_begging_state(exit_code)?;

    print_affirmation(exit_code, &config)?;

    Ok(exit_code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_role_transformation_middle() {
        let args = vec![
            "mommy".to_string(),
            "i".to_string(),
            "mean".to_string(),
            "daddy".to_string(),
        ];
        assert_eq!(check_role_transformation(&args), Some("daddy"));
    }

    #[test]
    fn test_check_role_transformation_start() {
        let args = vec!["i".to_string(), "mean".to_string(), "daddy".to_string()];
        assert_eq!(check_role_transformation(&args), Some("daddy"));
    }

    #[test]
    fn test_check_role_transformation_no_match() {
        let args = vec!["ls".to_string(), "-la".to_string()];
        assert_eq!(check_role_transformation(&args), None);
    }

    #[test]
    fn test_check_role_transformation_incomplete() {
        let args = vec!["i".to_string(), "mean".to_string()];
        assert_eq!(check_role_transformation(&args), None);
    }

    #[test]
    fn test_check_role_transformation_wrong_pattern() {
        let args = vec!["i".to_string(), "think".to_string(), "daddy".to_string()];
        assert_eq!(check_role_transformation(&args), None);
    }

    #[test]
    fn test_check_role_transformation_empty() {
        let args: Vec<String> = vec![];
        assert_eq!(check_role_transformation(&args), None);
    }

    #[test]
    fn test_check_role_transformation_multiple() {
        let args = vec![
            "i".to_string(),
            "mean".to_string(),
            "daddy".to_string(),
            "i".to_string(),
            "mean".to_string(),
            "mommy".to_string(),
        ];
        assert_eq!(check_role_transformation(&args), Some("daddy"));
    }
}
