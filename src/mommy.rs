use crate::affirmations::{
    Affirmations, load_affirmations_with_mood, load_custom_affirmations_with_mood,
};
use crate::color::random_style_pick;
use crate::config::{detect_role_from_binary, load_config};
use crate::utils::{fill_template, graceful_print, random_string_pick};
use std::env;
use std::process::{Command, exit};

const RECURSION_LIMIT: usize = 100;

fn choose_template<'a>(
    json_template: Option<&'a Vec<String>>,
    default_template: &'a Vec<String>,
) -> &'a str {
    let templates = json_template.unwrap_or(default_template);
    let idx = fastrand::usize(..templates.len());
    templates[idx].as_str()
}

/// Check if quiet mode is enabled from command line arguments
fn is_quiet_mode_enabled(args: &[String]) -> bool {
    args.iter().any(|arg| arg == "--quiet" || arg == "-q")
}

/// Check if we're running as a cargo subcommand
fn is_cargo_subcommand() -> bool {
    env::current_exe()
        .ok()
        .and_then(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .map(|name| name.starts_with("cargo-"))
        })
        .unwrap_or(false)
}

/// Check if the command contains "i mean" for role transformation
fn check_role_transformation(args: &[String]) -> Option<String> {
    // Look for pattern: "mommy i mean daddy" or similar
    for i in 0..args.len().saturating_sub(2) {
        if args[i] == "i" && args[i + 1] == "mean" && i + 2 < args.len() {
            return Some(args[i + 2].clone());
        }
    }
    None
}

/// Perform role transformation by copying the binary
#[cfg(unix)]
fn perform_role_transformation(new_role: &str) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    let current_exe = env::current_exe()?;
    let parent = current_exe.parent().ok_or("Cannot get parent directory")?;

    // Determine the new binary name
    let new_name = if is_cargo_subcommand() {
        format!("cargo-{}", new_role)
    } else {
        new_role.to_string()
    };

    let new_path = parent.join(&new_name);

    // Copy the binary
    fs::copy(&current_exe, &new_path)?;

    // Make it executable
    let mut perms = fs::metadata(&new_path)?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&new_path, perms)?;

    println!("Created new binary: {}", new_path.display());
    println!("You can now use: {}", new_name);

    Ok(())
}

#[cfg(not(unix))]
fn perform_role_transformation(new_role: &str) -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;

    let current_exe = env::current_exe()?;
    let parent = current_exe.parent().ok_or("Cannot get parent directory")?;

    // Determine the new binary name
    let new_name = if is_cargo_subcommand() {
        format!("cargo-{}.exe", new_role)
    } else {
        format!("{}.exe", new_role)
    };

    let new_path = parent.join(&new_name);

    // Copy the binary
    fs::copy(&current_exe, &new_path)?;

    println!("Created new binary: {}", new_path.display());
    println!("You can now use: {}", new_name);

    Ok(())
}

pub fn mommy() -> Result<i32, Box<dyn std::error::Error>> {
    let mut config = load_config();

    // Check recursion limit
    if config.recursion_limit >= RECURSION_LIMIT {
        eprintln!("Recursion limit exceeded! Mommy is stuck in a loop~");
        return Ok(2); // Special exit code for recursion overflow
    }

    let selected_mood = random_string_pick(&config.moods).unwrap_or_else(|| "chill".to_string());

    let affirmations: Option<Affirmations> = if let Some(ref path) = config.affirmations {
        load_custom_affirmations_with_mood(path, &selected_mood)
    } else {
        load_affirmations_with_mood(&selected_mood)
    };

    let affirmations_error: Vec<String> =
        vec!["{roles} failed to load any affirmations, {little}~ {emotes}".to_string()];

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        let role = detect_role_from_binary();
        let usage = if is_cargo_subcommand() {
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
        perform_role_transformation(&new_role)?;
        return Ok(0);
    }

    // Skip the binary name for processing
    let mut command_args = &args[1..];

    // If running as cargo subcommand, skip "cargo" if it's the first arg
    if is_cargo_subcommand() && !command_args.is_empty() && command_args[0] == "cargo" {
        command_args = &command_args[1..];
    }

    // Handle "please" for begging mode (if enabled)
    #[cfg(feature = "beg")]
    {
        let has_please = command_args.iter().any(|arg| arg == "please");
        // Begging logic would go here - for now, we just note its presence
        // Full implementation would require the begging state tracking from cargo-mommy
    }

    // Filter out "please" from args if present
    let filtered_args: Vec<String> = command_args
        .iter()
        .filter(|arg| *arg != "please")
        .cloned()
        .collect();

    let exit_code: i32 = if config.needy {
        filtered_args.first().ok_or("Missing exit code")?.parse()?
    } else if is_cargo_subcommand() {
        // Running as cargo subcommand - execute cargo with the provided args
        if filtered_args.is_empty() {
            eprintln!("No cargo command provided");
            exit(1);
        }

        // Increment recursion counter
        let new_recursion = config.recursion_limit + 1;

        let status = Command::new("cargo")
            .args(&filtered_args)
            .env("CARGO_MOMMY_RECURSION_LIMIT", new_recursion.to_string())
            .status()?;

        status.code().unwrap_or(1)
    } else {
        // Running as shell command wrapper
        let raw_command = filtered_args.join(" ");
        let run_command = if let Some(ref aliases_path) = config.aliases {
            format!(
                "shopt -s expand_aliases; source \"{}\"; eval {}",
                aliases_path, raw_command
            )
        } else {
            raw_command
        };

        let status = Command::new("bash").arg("-c").arg(&run_command).status()?;

        status.code().unwrap_or(1)
    };

    // Skip output if quiet mode is enabled
    if config.quiet {
        return Ok(exit_code);
    }

    let (template, _affirmation_type) = match (exit_code == 0, config.only_negative) {
        (true, false) => (
            choose_template(
                affirmations.as_ref().map(|aff| &aff.positive),
                &affirmations_error,
            ),
            "positive",
        ),
        (false, true) => (
            choose_template(
                affirmations.as_ref().map(|aff| &aff.negative),
                &affirmations_error,
            ),
            "negative",
        ),
        (false, false) => (
            choose_template(
                affirmations.as_ref().map(|aff| &aff.negative),
                &affirmations_error,
            ),
            "negative",
        ),
        _ => return Ok(exit_code),
    };

    let output = fill_template(template, &config);
    let styled_output = random_style_pick(&config).paint(output);
    graceful_print(styled_output);

    Ok(exit_code)
}
