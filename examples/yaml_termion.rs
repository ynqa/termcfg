use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use termcfg::termion_config::{
    content_style_serde::{self, Color, ContentStyle},
    event_set_serde,
};

#[cfg(feature = "termion_4_0_6")]
use termion_4_0_6 as termion;

use termion::{
    color,
    event::{Event, Key},
    style,
};

#[derive(Deserialize, Serialize)]
struct ExampleConfig {
    #[serde(with = "event_set_serde")]
    keybind: HashSet<Event>,
    #[serde(with = "content_style_serde")]
    style: ContentStyle,
}

fn main() {
    let content = r#"
keybind:
  - "Ctrl+C"
  - "Shift+Down"
style: "fg=lightblue,bg=#112233,attr=bold|underline|italic"
"#;

    let config: ExampleConfig = serde_yaml::from_str(content).expect("failed to parse YAML");

    assert!(config.keybind.contains(&Event::Key(Key::ShiftDown)));
    assert!(config.keybind.contains(&Event::Key(Key::Ctrl('c'))));

    let style_seq: String = config
        .style
        .attributes
        .iter()
        .map(ToString::to_string)
        .collect();

    let serialized = serde_yaml::to_string(&config).expect("failed to serialize YAML");
    println!("Serialized YAML:\n{}", serialized);

    println!(
        "{}{}{}hello termcfg{}{}{}",
        config.style.fg.unwrap_or(color::Fg(Color::Reset)),
        config.style.bg.unwrap_or(color::Bg(Color::Reset)),
        style_seq,
        style::Reset,
        color::Fg(color::Reset),
        color::Bg(color::Reset),
    );
}
