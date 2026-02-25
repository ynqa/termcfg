use crate::{
    crossterm::style::{Attribute, Color, ContentStyle},
    error::StyleError,
    style::style_def::{ColorDef, ContentStyleDef},
};

impl TryFrom<ContentStyleDef> for ContentStyle {
    type Error = StyleError;

    fn try_from(style_def: ContentStyleDef) -> Result<Self, Self::Error> {
        let foreground_color = style_def.fg.as_ref().map(parse_color_def).transpose()?;
        let background_color = style_def.bg.as_ref().map(parse_color_def).transpose()?;
        let underline_color = style_def.ul.as_ref().map(parse_color_def).transpose()?;

        let mut style = ContentStyle {
            foreground_color,
            background_color,
            underline_color,
            ..ContentStyle::default()
        };

        for token in style_def.attributes {
            style.attributes.set(parse_attribute_token(&token)?);
        }

        Ok(style)
    }
}

impl TryFrom<&ContentStyle> for ContentStyleDef {
    type Error = StyleError;

    fn try_from(style: &ContentStyle) -> Result<Self, Self::Error> {
        let fg = style.foreground_color.map(color_to_def).transpose()?;
        let bg = style.background_color.map(color_to_def).transpose()?;
        let ul = style.underline_color.map(color_to_def).transpose()?;

        let mut attributes = Vec::new();
        for attribute in Attribute::iterator() {
            if style.attributes.has(attribute)
                && let Some(token) = attribute_to_token(attribute)
            {
                attributes.push(token.to_string());
            }
        }

        Ok(Self {
            fg,
            bg,
            ul,
            attributes,
        })
    }
}

fn parse_color_def(color_def: &ColorDef) -> Result<Color, StyleError> {
    match color_def {
        ColorDef::Named(name) => {
            parse_named_color(name).ok_or_else(|| StyleError::UnsupportedColorToken {
                token: name.to_string(),
            })
        }
        ColorDef::Rgb { r, g, b } => Ok(Color::Rgb {
            r: *r,
            g: *g,
            b: *b,
        }),
    }
}

fn parse_named_color(token: &str) -> Option<Color> {
    let token = token.to_ascii_lowercase();

    let color = match token.as_str() {
        "reset" => Color::Reset,
        "black" => Color::Black,
        "darkgrey" => Color::DarkGrey,
        "red" => Color::Red,
        "darkred" => Color::DarkRed,
        "green" => Color::Green,
        "darkgreen" => Color::DarkGreen,
        "yellow" => Color::Yellow,
        "darkyellow" => Color::DarkYellow,
        "blue" => Color::Blue,
        "darkblue" => Color::DarkBlue,
        "magenta" => Color::Magenta,
        "darkmagenta" => Color::DarkMagenta,
        "cyan" => Color::Cyan,
        "darkcyan" => Color::DarkCyan,
        "white" => Color::White,
        "grey" => Color::Grey,
        _ => return None,
    };

    Some(color)
}

fn color_to_def(color: Color) -> Result<ColorDef, StyleError> {
    let color_def = match color {
        Color::Reset => ColorDef::Named("reset".to_string()),
        Color::Black => ColorDef::Named("black".to_string()),
        Color::DarkGrey => ColorDef::Named("darkgrey".to_string()),
        Color::Red => ColorDef::Named("red".to_string()),
        Color::DarkRed => ColorDef::Named("darkred".to_string()),
        Color::Green => ColorDef::Named("green".to_string()),
        Color::DarkGreen => ColorDef::Named("darkgreen".to_string()),
        Color::Yellow => ColorDef::Named("yellow".to_string()),
        Color::DarkYellow => ColorDef::Named("darkyellow".to_string()),
        Color::Blue => ColorDef::Named("blue".to_string()),
        Color::DarkBlue => ColorDef::Named("darkblue".to_string()),
        Color::Magenta => ColorDef::Named("magenta".to_string()),
        Color::DarkMagenta => ColorDef::Named("darkmagenta".to_string()),
        Color::Cyan => ColorDef::Named("cyan".to_string()),
        Color::DarkCyan => ColorDef::Named("darkcyan".to_string()),
        Color::White => ColorDef::Named("white".to_string()),
        Color::Grey => ColorDef::Named("grey".to_string()),
        Color::Rgb { r, g, b } => ColorDef::Rgb { r, g, b },
        Color::AnsiValue(_) => {
            return Err(StyleError::UnsupportedColorToken {
                token: "ansi color is not supported for style serialization".to_string(),
            });
        }
    };

    Ok(color_def)
}

pub fn parse_attribute_token(token: &str) -> Result<Attribute, StyleError> {
    let token = token.to_ascii_lowercase();

    let attribute = match token.as_str() {
        "reset" => Attribute::Reset,
        "bold" => Attribute::Bold,
        "dim" => Attribute::Dim,
        "italic" => Attribute::Italic,
        "underlined" => Attribute::Underlined,
        "doubleunderlined" => Attribute::DoubleUnderlined,
        "undercurled" => Attribute::Undercurled,
        "underdotted" => Attribute::Underdotted,
        "underdashed" => Attribute::Underdashed,
        "slowblink" => Attribute::SlowBlink,
        "rapidblink" => Attribute::RapidBlink,
        "reverse" => Attribute::Reverse,
        "hidden" => Attribute::Hidden,
        "crossedout" => Attribute::CrossedOut,
        "fraktur" => Attribute::Fraktur,
        "nobold" => Attribute::NoBold,
        "normalintensity" => Attribute::NormalIntensity,
        "noitalic" => Attribute::NoItalic,
        "nounderline" => Attribute::NoUnderline,
        "noblink" => Attribute::NoBlink,
        "noreverse" => Attribute::NoReverse,
        "nohidden" => Attribute::NoHidden,
        "notcrossedout" => Attribute::NotCrossedOut,
        "framed" => Attribute::Framed,
        "encircled" => Attribute::Encircled,
        "overlined" => Attribute::OverLined,
        "notframedorencircled" => Attribute::NotFramedOrEncircled,
        "notoverlined" => Attribute::NotOverLined,
        _ => {
            return Err(StyleError::UnsupportedAttributeToken {
                token: token.to_string(),
            });
        }
    };

    Ok(attribute)
}

pub fn attribute_to_token(attribute: Attribute) -> Option<&'static str> {
    let token = match attribute {
        Attribute::Reset => "reset",
        Attribute::Bold => "bold",
        Attribute::Dim => "dim",
        Attribute::Italic => "italic",
        Attribute::Underlined => "underlined",
        Attribute::DoubleUnderlined => "doubleunderlined",
        Attribute::Undercurled => "undercurled",
        Attribute::Underdotted => "underdotted",
        Attribute::Underdashed => "underdashed",
        Attribute::SlowBlink => "slowblink",
        Attribute::RapidBlink => "rapidblink",
        Attribute::Reverse => "reverse",
        Attribute::Hidden => "hidden",
        Attribute::CrossedOut => "crossedout",
        Attribute::Fraktur => "fraktur",
        Attribute::NoBold => "nobold",
        Attribute::NormalIntensity => "normalintensity",
        Attribute::NoItalic => "noitalic",
        Attribute::NoUnderline => "nounderline",
        Attribute::NoBlink => "noblink",
        Attribute::NoReverse => "noreverse",
        Attribute::NoHidden => "nohidden",
        Attribute::NotCrossedOut => "notcrossedout",
        Attribute::Framed => "framed",
        Attribute::Encircled => "encircled",
        Attribute::OverLined => "overlined",
        Attribute::NotFramedOrEncircled => "notframedorencircled",
        Attribute::NotOverLined => "notoverlined",
        _ => return None,
    };

    Some(token)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod try_from_content_style_def {
        use super::*;

        #[test]
        fn converts_style_def_to_content_style() {
            let style_def = ContentStyleDef {
                fg: Some(ColorDef::Named("red".to_string())),
                bg: Some(ColorDef::Rgb {
                    r: 0x11,
                    g: 0x22,
                    b: 0x33,
                }),
                ul: Some(ColorDef::Rgb {
                    r: 0xAA,
                    g: 0xBB,
                    b: 0xCC,
                }),
                attributes: vec!["bold".to_string(), "underlined".to_string()],
            };

            let style = ContentStyle::try_from(style_def).unwrap();
            assert_eq!(style.foreground_color, Some(Color::Red));
            assert_eq!(
                style.background_color,
                Some(Color::Rgb {
                    r: 0x11,
                    g: 0x22,
                    b: 0x33,
                })
            );
            assert_eq!(
                style.underline_color,
                Some(Color::Rgb {
                    r: 0xAA,
                    g: 0xBB,
                    b: 0xCC,
                })
            );
            assert!(style.attributes.has(Attribute::Bold));
            assert!(style.attributes.has(Attribute::Underlined));
        }
    }

    mod try_from_content_style_ref {
        use super::*;

        #[test]
        fn converts_content_style_to_style_def() {
            let mut style = ContentStyle {
                foreground_color: Some(Color::DarkBlue),
                background_color: Some(Color::Yellow),
                underline_color: Some(Color::Rgb {
                    r: 0,
                    g: 255,
                    b: 127,
                }),
                ..ContentStyle::default()
            };
            style.attributes.set(Attribute::Italic);
            style.attributes.set(Attribute::OverLined);

            let style_def = ContentStyleDef::try_from(&style).unwrap();
            assert_eq!(style_def.fg, Some(ColorDef::Named("darkblue".to_string())));
            assert_eq!(style_def.bg, Some(ColorDef::Named("yellow".to_string())));
            assert_eq!(
                style_def.ul,
                Some(ColorDef::Rgb {
                    r: 0x00,
                    g: 0xFF,
                    b: 0x7F
                })
            );
            assert!(style_def.attributes.contains(&"italic".to_string()));
            assert!(style_def.attributes.contains(&"overlined".to_string()));
        }

        #[test]
        fn rejects_ansi_value_for_serialization() {
            let style = ContentStyle {
                foreground_color: Some(Color::AnsiValue(8)),
                ..ContentStyle::default()
            };

            assert!(matches!(
                ContentStyleDef::try_from(&style),
                Err(StyleError::UnsupportedColorToken { .. })
            ));
        }
    }

    mod parse_color_def {
        #[test]
        fn rejects_unknown_color() {
            assert!(matches!(
                super::super::parse_color_def(&super::super::ColorDef::Named(
                    "mystery".to_string()
                )),
                Err(super::super::StyleError::UnsupportedColorToken { .. })
            ));
        }

        #[test]
        fn rejects_alias_color() {
            assert!(matches!(
                super::super::parse_color_def(&super::super::ColorDef::Named("gray".to_string())),
                Err(super::super::StyleError::UnsupportedColorToken { .. })
            ));
        }
    }

    mod parse_attribute_token {
        #[test]
        fn rejects_unknown_attribute() {
            assert!(matches!(
                super::super::parse_attribute_token("mystery"),
                Err(super::super::StyleError::UnsupportedAttributeToken { .. })
            ));
        }

        #[test]
        fn rejects_alias_attributes() {
            let aliases = ["underline", "blink", "inverse", "strikethrough"];
            for token in aliases {
                assert!(matches!(
                    super::super::parse_attribute_token(token),
                    Err(super::super::StyleError::UnsupportedAttributeToken { .. })
                ));
            }
        }
    }
}
