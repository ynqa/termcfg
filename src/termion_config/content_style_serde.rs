use serde::{Deserialize, Deserializer, Serializer};

use crate::style::{
    format::content_style_to_string, parse::parse_content_style, style_def::ContentStyleDef,
};
use crate::termion::{color, style};

/// Representing `termion` style settings in a structured way,
/// since `termion` does not have an equivalent to `crossterm::ContentStyle`.
#[derive(Debug, Clone, Default)]
pub struct ContentStyle {
    pub fg: Option<color::Fg<Color>>,
    pub bg: Option<color::Bg<Color>>,
    pub attributes: Vec<Style>,
}

// Option<Box<dyn termion::color::Color>>, which would be less type-safe and more complex to serialize/deserialize.
// Using enums allows us to have a clear mapping of supported colors and styles, and makes it easier to implement serialization/deserialization logic.
#[derive(Debug, Clone, Copy)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    LightBlack,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    LightWhite,
    AnsiValue(u8),
    Rgb(u8, u8, u8),
    Reset,
}

#[derive(Clone, Copy)]
pub enum Style {
    Reset,
    Bold,
    Faint,
    Italic,
    Underline,
    Blink,
    Invert,
    CrossedOut,
    NoBold,
    NoFaint,
    NoItalic,
    NoUnderline,
    NoBlink,
    NoInvert,
    NoCrossedOut,
    Framed,
}

impl std::fmt::Debug for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Style::Reset => "Reset",
            Style::Bold => "Bold",
            Style::Faint => "Faint",
            Style::Italic => "Italic",
            Style::Underline => "Underline",
            Style::Blink => "Blink",
            Style::Invert => "Invert",
            Style::CrossedOut => "CrossedOut",
            Style::NoBold => "NoBold",
            Style::NoFaint => "NoFaint",
            Style::NoItalic => "NoItalic",
            Style::NoUnderline => "NoUnderline",
            Style::NoBlink => "NoBlink",
            Style::NoInvert => "NoInvert",
            Style::NoCrossedOut => "NoCrossedOut",
            Style::Framed => "Framed",
        };
        f.write_str(name)
    }
}

impl color::Color for Color {
    fn write_fg(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Color::Black => color::Black.write_fg(f),
            Color::Red => color::Red.write_fg(f),
            Color::Green => color::Green.write_fg(f),
            Color::Yellow => color::Yellow.write_fg(f),
            Color::Blue => color::Blue.write_fg(f),
            Color::Magenta => color::Magenta.write_fg(f),
            Color::Cyan => color::Cyan.write_fg(f),
            Color::White => color::White.write_fg(f),
            Color::LightBlack => color::LightBlack.write_fg(f),
            Color::LightRed => color::LightRed.write_fg(f),
            Color::LightGreen => color::LightGreen.write_fg(f),
            Color::LightYellow => color::LightYellow.write_fg(f),
            Color::LightBlue => color::LightBlue.write_fg(f),
            Color::LightMagenta => color::LightMagenta.write_fg(f),
            Color::LightCyan => color::LightCyan.write_fg(f),
            Color::LightWhite => color::LightWhite.write_fg(f),
            Color::AnsiValue(value) => color::AnsiValue(*value).write_fg(f),
            Color::Rgb(r, g, b) => color::Rgb(*r, *g, *b).write_fg(f),
            Color::Reset => color::Reset.write_fg(f),
        }
    }

    fn write_bg(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Color::Black => color::Black.write_bg(f),
            Color::Red => color::Red.write_bg(f),
            Color::Green => color::Green.write_bg(f),
            Color::Yellow => color::Yellow.write_bg(f),
            Color::Blue => color::Blue.write_bg(f),
            Color::Magenta => color::Magenta.write_bg(f),
            Color::Cyan => color::Cyan.write_bg(f),
            Color::White => color::White.write_bg(f),
            Color::LightBlack => color::LightBlack.write_bg(f),
            Color::LightRed => color::LightRed.write_bg(f),
            Color::LightGreen => color::LightGreen.write_bg(f),
            Color::LightYellow => color::LightYellow.write_bg(f),
            Color::LightBlue => color::LightBlue.write_bg(f),
            Color::LightMagenta => color::LightMagenta.write_bg(f),
            Color::LightCyan => color::LightCyan.write_bg(f),
            Color::LightWhite => color::LightWhite.write_bg(f),
            Color::AnsiValue(value) => color::AnsiValue(*value).write_bg(f),
            Color::Rgb(r, g, b) => color::Rgb(*r, *g, *b).write_bg(f),
            Color::Reset => color::Reset.write_bg(f),
        }
    }
}

impl std::fmt::Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Style::Reset => write!(f, "{}", style::Reset),
            Style::Bold => write!(f, "{}", style::Bold),
            Style::Faint => write!(f, "{}", style::Faint),
            Style::Italic => write!(f, "{}", style::Italic),
            Style::Underline => write!(f, "{}", style::Underline),
            Style::Blink => write!(f, "{}", style::Blink),
            Style::Invert => write!(f, "{}", style::Invert),
            Style::CrossedOut => write!(f, "{}", style::CrossedOut),
            Style::NoBold => write!(f, "{}", style::NoBold),
            Style::NoFaint => write!(f, "{}", style::NoFaint),
            Style::NoItalic => write!(f, "{}", style::NoItalic),
            Style::NoUnderline => write!(f, "{}", style::NoUnderline),
            Style::NoBlink => write!(f, "{}", style::NoBlink),
            Style::NoInvert => write!(f, "{}", style::NoInvert),
            Style::NoCrossedOut => write!(f, "{}", style::NoCrossedOut),
            Style::Framed => write!(f, "{}", style::Framed),
        }
    }
}

pub fn serialize<S>(style: &ContentStyle, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let style_def = ContentStyleDef::try_from(style).map_err(serde::ser::Error::custom)?;
    let encoded = content_style_to_string(&style_def);
    serializer.serialize_str(&encoded)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<ContentStyle, D::Error>
where
    D: Deserializer<'de>,
{
    let encoded = String::deserialize(deserializer)?;
    let style_def = parse_content_style(&encoded).map_err(serde::de::Error::custom)?;
    ContentStyle::try_from(style_def).map_err(serde::de::Error::custom)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Deserialize, serde::Serialize)]
    struct Config {
        #[serde(with = "crate::termion_config::content_style_serde")]
        style: ContentStyle,
    }

    mod deserialize {
        use super::*;

        #[test]
        fn deserializes_style_string() {
            let content = r#"style = "fg=red,bg=#102030,attr=bold|underline|italic""#;
            let config: Config = toml::from_str(content).unwrap();

            assert!(matches!(config.style.fg, Some(color::Fg(Color::Red))));
            assert!(matches!(
                config.style.bg,
                Some(color::Bg(Color::Rgb(0x10, 0x20, 0x30)))
            ));
            assert!(
                config
                    .style
                    .attributes
                    .iter()
                    .any(|s| matches!(s, Style::Bold))
            );
            assert!(
                config
                    .style
                    .attributes
                    .iter()
                    .any(|s| matches!(s, Style::Underline))
            );
            assert!(
                config
                    .style
                    .attributes
                    .iter()
                    .any(|s| matches!(s, Style::Italic))
            );
        }

        #[test]
        fn rejects_underline_color() {
            let content = r#"style = "fg=red,ul=#112233,attr=bold""#;
            assert!(toml::from_str::<Config>(content).is_err());
        }
    }

    mod serialize {
        use super::*;

        #[test]
        fn serializes_style_string() {
            let style = ContentStyle {
                fg: Some(color::Fg(Color::LightBlue)),
                bg: Some(color::Bg(Color::Rgb(255, 0, 255))),
                attributes: vec![Style::Bold, Style::Underline],
            };

            let config = Config { style };
            let serialized = toml::to_string(&config).unwrap();

            assert!(serialized.contains("fg=lightblue"));
            assert!(serialized.contains("bg=#FF00FF"));
            assert!(serialized.contains("attr=bold|underline"));
        }
    }
}
