use crate::config::ConfigMommy;
use owo_colors::{Style, DynColors};

pub fn color_from_name(name: &str) -> Option<DynColors> {
    match name {
        "black" => Some(DynColors::Rgb(0, 0, 0)),
        "red" => Some(DynColors::Rgb(255, 0, 0)),
        "green" => Some(DynColors::Rgb(0, 255, 0)),
        "yellow" => Some(DynColors::Rgb(255, 255, 0)),
        "blue" => Some(DynColors::Rgb(0, 0, 255)),
        "purple" | "magenta" => Some(DynColors::Rgb(255, 0, 255)),
        "cyan" => Some(DynColors::Rgb(0, 255, 255)),
        "white" => Some(DynColors::Rgb(255, 255, 255)),
        _ => None,
    }
}

pub fn color_from_rgb(rgb_str: &str) -> Option<DynColors> {
    let mut parts = rgb_str.split(',').map(str::trim);
    let r = parts.next()?.parse::<u8>().ok()?;
    let g = parts.next()?.parse::<u8>().ok()?;
    let b = parts.next()?.parse::<u8>().ok()?;

    if parts.next().is_some() {
        return None;
    }

    Some(DynColors::Rgb(r, g, b))
}

/// Apply a single style attribute to the Style object
fn apply_style_attr(mut style: Style, attr: &str) -> Style {
    match attr {
        "bold" => style = style.bold(),
        "italic" => style = style.italic(),
        "dimmed" => style = style.dimmed(),
        "underline" => style = style.underline(),
        "blink" => style = style.blink(),
        "reverse" => style = style.reversed(),
        "hidden" => style = style.hidden(),
        _ => {},
    }
    style
}

pub fn random_style_pick(config: &ConfigMommy) -> Style {
    let mut style = Style::new();

    // Use pre-parsed color vectors from config
    if let Some(ref rgb_candidates) = config.color_rgb {
        if !rgb_candidates.is_empty() {
            let idx = fastrand::usize(..rgb_candidates.len());
            if let Some(col) = color_from_rgb(&rgb_candidates[idx]) {
                style = style.color(col);
            }
        }
    } else if !config.colors.is_empty() {
        let idx = fastrand::usize(..config.colors.len());
        if let Some(col) = color_from_name(&config.colors[idx]) {
            style = style.color(col);
        }
    }

    // Use pre-parsed style combinations from config
    if !config.styles.is_empty() {
        let idx = fastrand::usize(..config.styles.len());
        let styles_in_combo = &config.styles[idx];

        // Styles are already parsed, just apply them
        for attr in styles_in_combo {
            style = apply_style_attr(style, attr);
        }
    }

    style
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::load_config;

    #[test]
    fn test_color_names() {
        // Make sure all colors are correctly evaluated:
        assert_eq!(color_from_name("black"), Some(DynColors::Rgb(0, 0, 0)));
        assert_eq!(color_from_name("red"), Some(DynColors::Rgb(255, 0, 0)));
        assert_eq!(color_from_name("green"), Some(DynColors::Rgb(0, 255, 0)));
        assert_eq!(color_from_name("yellow"), Some(DynColors::Rgb(255, 255, 0)));
        assert_eq!(color_from_name("blue"), Some(DynColors::Rgb(0, 0, 255)));
        assert_eq!(color_from_name("purple"), Some(DynColors::Rgb(255, 0, 255)));
        assert_eq!(color_from_name("magenta"), Some(DynColors::Rgb(255, 0, 255)));
        assert_eq!(color_from_name("cyan"), Some(DynColors::Rgb(0, 255, 255)));
        assert_eq!(color_from_name("white"), Some(DynColors::Rgb(255, 255, 255)));
    }

    #[test]
    fn test_invalid_color() {
        // Not valid color name:
        assert_eq!(color_from_name("not a color"), None);
        assert_eq!(color_from_name(""), None);
    }

    #[test]
    fn test_rgb_color_ok() {
        // Well‐formatted:
        let c = color_from_rgb("10,20,30");
        assert_eq!(c, Some(DynColors::Rgb(10, 20, 30)));

        // With random spaces:
        let c2 = color_from_rgb("  0 ,255, 128  ");
        assert_eq!(c2, Some(DynColors::Rgb(0, 255, 128)));
    }

    #[test]
    fn test_rgb_color_err() {
        // Wrong amount:
        assert_eq!(color_from_rgb("10,20"), None);
        assert_eq!(color_from_rgb("1,2,3,4"), None);

        // Non‐numeric:
        assert_eq!(color_from_rgb("a,b,c"), None);

        // Out of range:
        assert_eq!(color_from_rgb("256,0,0"), None);
    }

    #[test]
    fn test_color_style() {
        use owo_colors::OwoColorize;

        // Not RGB and bold:
        let mut config = load_config();
        config.colors = vec!["red".to_string()];
        config.styles = vec![vec!["bold".to_string()]];
        config.color_rgb = None;

        let styled = random_style_pick(&config);
        let output = "Test".style(styled).to_string();

        // Check that output contains ANSI escape codes
        assert!(
            output.contains("\x1b["),
            "expected output to contain ANSI escape codes"
        );
        assert!(
            output.contains("Test"),
            "expected output to contain 'Test'"
        );
    }

    #[test]
    fn test_rgb_with_two_styles() {
        use owo_colors::OwoColorize;

        // RGB and two styles:
        let mut config = load_config();
        config.styles = vec![vec!["underline".to_string(), "italic".to_string()]];
        config.color_rgb = Some(vec!["128,0,255".to_string()]);

        let styled = random_style_pick(&config);
        let output = "Test".style(styled).to_string();

        // Check that output contains ANSI escape codes and RGB color codes
        assert!(
            output.contains("\x1b["),
            "expected output to contain ANSI escape codes"
        );
        assert!(
            output.contains("Test"),
            "expected output to contain 'Test'"
        );
    }
}
