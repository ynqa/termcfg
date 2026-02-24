# termcfg

*termcfg* is a library for converting terminal events and styles to and from compact strings for configuration files in Rust.

## Features

- Convert between terminal events and human-readable shortcut strings
- Convert between terminal styles and compact style strings
- `serde` helpers to easily serialize and deserialize terminal events and styles in configuration files
- Supports keyboard and mouse events
- Feature-gated backend modules: `crossterm` and `termion`

## Installation

If you want to use `crossterm` v0.29.0:

```toml
[dependencies]
termcfg = { version = "0.1.0", features = ["crossterm_0_29_0"] }
```

else if you want to use `termion` v4.0.6:

```toml
[dependencies]
termcfg = { version = "0.1.0", features = ["termion_4_0_6"] }
```

## Notation

See [Notations.md](./Notations.md) for the backend compatibility matrix between `crossterm` and `termion`.

### Shortcut

The shortcut string consists of zero or more modifiers followed by an event token.
Modifiers are separated by `+` and can be `Ctrl`, `Alt`, `Shift` etc.
The event token can be a key code (e.g., `A`, `F1`, `Enter`) or a mouse token (e.g., `LeftDown`, `ScrollUp`).

For example, `Ctrl+Shift+C` represents the combination of the Control and Shift modifiers with the 'C' key.

### Styling

The style string consists of zero or more style attributes separated by commas.
Each attribute can be a foreground color (`fg`), background color (`bg`),
underline color (`ul`, only supported in `crossterm`), or text attribute (`attr`).
Colors can be specified as named colors (e.g., `red`, `blue`), RGB values (e.g., `#RRGGBB`).
Text attributes can be `bold`, `underlined`, `italic`, etc., and can be combined using the `|` separator.

For example, `fg=red,bg=#112233,ul=#0C0C0C,attr=bold|underlined|italic` represents a style with a red foreground,
a background color of `#112233`, an underline color of `#0C0C0C`, and text attributes of bold, underlined, and italic.

## Example

This example demonstrates how to use `termcfg` to deserialize a TOML configuration
containing a set of keybinds and a content style for `crossterm`.

```rust
use std::collections::HashSet;

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
```
