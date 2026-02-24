use std::collections::HashSet;

#[cfg(feature = "crossterm_0_28_1")]
use crossterm_0_28_1 as crossterm;
#[cfg(feature = "crossterm_0_29_0")]
use crossterm_0_29_0 as crossterm;

use crossterm::{
    event::{Event, KeyCode, KeyEvent, KeyModifiers},
    style::{Attribute, Color, ContentStyle},
};
use serde::{Deserialize, Serialize};
use termcfg::crossterm_config::{content_style_serde, event_set_serde};

#[derive(Deserialize, Serialize)]
struct ExampleConfig {
    #[serde(with = "event_set_serde")]
    keybind: HashSet<Event>,
    #[serde(with = "content_style_serde")]
    style: ContentStyle,
}

fn main() {
    let content = r##"
keybind = ["Ctrl+C", "Shift+Down"]
style = "fg=red,bg=#112233,ul=#0C0C0C,attr=bold|underlined|italic"
"##;

    let config: ExampleConfig = toml::from_str(content).expect("failed to parse TOML");

    assert!(config.keybind.contains(&Event::Key(KeyEvent::new(
        KeyCode::Down,
        KeyModifiers::SHIFT,
    ))));
    assert!(config.keybind.contains(&Event::Key(KeyEvent::new(
        KeyCode::Char('c'),
        KeyModifiers::CONTROL,
    ))));
    assert_eq!(config.style.foreground_color, Some(Color::Red));
    assert_eq!(
        config.style.background_color,
        Some(Color::Rgb {
            r: 0x11,
            g: 0x22,
            b: 0x33,
        })
    );
    assert_eq!(
        config.style.underline_color,
        Some(Color::Rgb {
            r: 0x0C,
            g: 0x0C,
            b: 0x0C,
        })
    );
    assert!(config.style.attributes.has(Attribute::Bold));
    assert!(config.style.attributes.has(Attribute::Underlined));
    assert!(config.style.attributes.has(Attribute::Italic));

    let serialized = toml::to_string(&config).expect("failed to serialize TOML");
    println!("Serialized TOML:\n{}", serialized);

    println!("{}", config.style.apply("hello termcfg"));
}
