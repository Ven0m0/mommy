use std::env;
use std::process::{Command, exit};
use crate::config::load_config;
use crate::utils::{fill_template, graceful_print, random_string_pick};
use crate::affirmations::{load_affirmations_with_mood, load_custom_affirmations_with_mood, Affirmations};
use crate::color::random_style_pick;

fn choose_template<'a>( json_template: Option<&'a Vec<String>>, default_template: &'a Vec<String> ) -> &'a str {
    let templates = json_template.unwrap_or(default_template);
    let idx = fastrand::usize(..templates.len());
    templates[idx].as_str()
}

pub fn mommy() -> Result<i32, Box<dyn std::error::Error>> {
    let config = load_config();
    
    let selected_mood = random_string_pick(&config.moods).unwrap_or_else(|| "chill".to_string());
    
    let affirmations: Option<Affirmations> = if let Some(ref path) = config.affirmations {
        load_custom_affirmations_with_mood(path, &selected_mood)
    } else {
        load_affirmations_with_mood(&selected_mood)
    };
    
    let affirmations_error: Vec<String> = vec![ "{roles} failed to load any affirmations, {little}~ {emotes}".to_string() ];

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { eprintln!("Usage: {} {}", args[0], if config.needy { "<exit_code>" } else { "<command> [args ...]" }); exit(1); }

    let exit_code: i32 = if config.needy {
        args[1].parse()?
    } else {
        let raw_command = args[1..].join(" ");
        let run_command = if let Some(ref aliases_path) = config.aliases {
            format!("shopt -s expand_aliases; source \"{}\"; eval {}", aliases_path, raw_command)
        } else {
            raw_command
        };

        let status = Command::new("bash")
            .arg("-c")
            .arg(&run_command)
            .status()?;

        status.code().unwrap_or(1)
    };

    let (template, _affirmation_type) = match (exit_code == 0, config.only_negative) {
        (true, false) => ( choose_template(affirmations.as_ref().map(|aff| &aff.positive), &affirmations_error), "positive" ),
        (false, true) => ( choose_template(affirmations.as_ref().map(|aff| &aff.negative), &affirmations_error), "negative" ),
        (false, false) => ( choose_template(affirmations.as_ref().map(|aff| &aff.negative), &affirmations_error), "negative" ),
        _ => return Ok(exit_code),
    };

    let output = fill_template(template, &config);
    let styled_output = random_style_pick(&config).paint(output);
    graceful_print(styled_output);

    Ok(exit_code)
}
