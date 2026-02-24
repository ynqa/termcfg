use crate::{
    error::StyleError,
    style::style_def::{ColorDef, ContentStyleDef},
    termion::color,
    termion_config::content_style_serde::{Color, ContentStyle, Style},
};

impl TryFrom<ContentStyleDef> for ContentStyle {
    type Error = StyleError;

    fn try_from(style_def: ContentStyleDef) -> Result<Self, Self::Error> {
        if style_def.ul.is_some() {
            return Err(StyleError::UnsupportedColorToken {
                token: "underline color is not supported for termion".to_string(),
            });
        }

        let fg = style_def
            .fg
            .as_ref()
            .map(parse_color_def)
            .transpose()?
            .map(color::Fg);
        let bg = style_def
            .bg
            .as_ref()
            .map(parse_color_def)
            .transpose()?
            .map(color::Bg);

        let mut attributes = Vec::with_capacity(style_def.attributes.len());
        for token in style_def.attributes {
            attributes.push(parse_attribute_token(&token)?);
        }

        Ok(ContentStyle { fg, bg, attributes })
    }
}

impl TryFrom<&ContentStyle> for ContentStyleDef {
    type Error = StyleError;

    fn try_from(style: &ContentStyle) -> Result<Self, Self::Error> {
        let fg = style.fg.map(|fg| color_to_def(fg.0)).transpose()?;
        let bg = style.bg.map(|bg| color_to_def(bg.0)).transpose()?;

        let mut attribute_tokens = Vec::with_capacity(style.attributes.len());
        for attribute in &style.attributes {
            attribute_tokens.push(attribute_to_token(*attribute).to_string());
        }

        Ok(Self {
            fg,
            bg,
            ul: None,
            attributes: attribute_tokens,
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
        ColorDef::Rgb { r, g, b } => Ok(Color::Rgb(*r, *g, *b)),
    }
}

fn parse_named_color(token: &str) -> Option<Color> {
    let token = token.to_ascii_lowercase();

    let color = match token.as_str() {
        "reset" => Color::Reset,
        "black" => Color::Black,
        "red" => Color::Red,
        "green" => Color::Green,
        "yellow" => Color::Yellow,
        "blue" => Color::Blue,
        "magenta" => Color::Magenta,
        "cyan" => Color::Cyan,
        "white" => Color::White,
        "lightblack" => Color::LightBlack,
        "lightred" => Color::LightRed,
        "lightgreen" => Color::LightGreen,
        "lightyellow" => Color::LightYellow,
        "lightblue" => Color::LightBlue,
        "lightmagenta" => Color::LightMagenta,
        "lightcyan" => Color::LightCyan,
        "lightwhite" => Color::LightWhite,
        _ => return None,
    };

    Some(color)
}

fn color_to_def(color: Color) -> Result<ColorDef, StyleError> {
    let color_def = match color {
        Color::Reset => ColorDef::Named("reset".to_string()),
        Color::Black => ColorDef::Named("black".to_string()),
        Color::Red => ColorDef::Named("red".to_string()),
        Color::Green => ColorDef::Named("green".to_string()),
        Color::Yellow => ColorDef::Named("yellow".to_string()),
        Color::Blue => ColorDef::Named("blue".to_string()),
        Color::Magenta => ColorDef::Named("magenta".to_string()),
        Color::Cyan => ColorDef::Named("cyan".to_string()),
        Color::White => ColorDef::Named("white".to_string()),
        Color::LightBlack => ColorDef::Named("lightblack".to_string()),
        Color::LightRed => ColorDef::Named("lightred".to_string()),
        Color::LightGreen => ColorDef::Named("lightgreen".to_string()),
        Color::LightYellow => ColorDef::Named("lightyellow".to_string()),
        Color::LightBlue => ColorDef::Named("lightblue".to_string()),
        Color::LightMagenta => ColorDef::Named("lightmagenta".to_string()),
        Color::LightCyan => ColorDef::Named("lightcyan".to_string()),
        Color::LightWhite => ColorDef::Named("lightwhite".to_string()),
        Color::Rgb(r, g, b) => ColorDef::Rgb { r, g, b },
        Color::AnsiValue(_) => {
            return Err(StyleError::UnsupportedColorToken {
                token: "ansi color is not supported for style serialization".to_string(),
            });
        }
    };

    Ok(color_def)
}

fn parse_attribute_token(token: &str) -> Result<Style, StyleError> {
    let token = token.to_ascii_lowercase();

    let style = match token.as_str() {
        "reset" => Style::Reset,
        "bold" => Style::Bold,
        "faint" => Style::Faint,
        "italic" => Style::Italic,
        "underline" => Style::Underline,
        "blink" => Style::Blink,
        "invert" => Style::Invert,
        "crossedout" => Style::CrossedOut,
        "nobold" => Style::NoBold,
        "nofaint" => Style::NoFaint,
        "noitalic" => Style::NoItalic,
        "nounderline" => Style::NoUnderline,
        "noblink" => Style::NoBlink,
        "noinvert" => Style::NoInvert,
        "nocrossedout" => Style::NoCrossedOut,
        "framed" => Style::Framed,
        _ => {
            return Err(StyleError::UnsupportedAttributeToken {
                token: token.to_string(),
            });
        }
    };

    Ok(style)
}

fn attribute_to_token(style: Style) -> &'static str {
    match style {
        Style::Reset => "reset",
        Style::Bold => "bold",
        Style::Faint => "faint",
        Style::Italic => "italic",
        Style::Underline => "underline",
        Style::Blink => "blink",
        Style::Invert => "invert",
        Style::CrossedOut => "crossedout",
        Style::NoBold => "nobold",
        Style::NoFaint => "nofaint",
        Style::NoItalic => "noitalic",
        Style::NoUnderline => "nounderline",
        Style::NoBlink => "noblink",
        Style::NoInvert => "noinvert",
        Style::NoCrossedOut => "nocrossedout",
        Style::Framed => "framed",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod try_from_content_style_def {
        use super::*;

        #[test]
        fn converts_style_def_to_content_style() {
            let style_def = ContentStyleDef {
                fg: Some(ColorDef::Named("lightblue".to_string())),
                bg: Some(ColorDef::Rgb {
                    r: 0x11,
                    g: 0x22,
                    b: 0x33,
                }),
                ul: None,
                attributes: vec!["bold".to_string(), "underline".to_string()],
            };

            let style = ContentStyle::try_from(style_def).unwrap();
            assert!(matches!(style.fg, Some(color::Fg(Color::LightBlue))));
            assert!(matches!(
                style.bg,
                Some(color::Bg(Color::Rgb(0x11, 0x22, 0x33)))
            ));
            assert!(style.attributes.iter().any(|s| matches!(s, Style::Bold)));
            assert!(
                style
                    .attributes
                    .iter()
                    .any(|s| matches!(s, Style::Underline))
            );
        }

        #[test]
        fn rejects_underline_color() {
            let style_def = ContentStyleDef {
                fg: None,
                bg: None,
                ul: Some(ColorDef::Named("red".to_string())),
                attributes: vec![],
            };

            assert!(matches!(
                ContentStyle::try_from(style_def),
                Err(StyleError::UnsupportedColorToken { .. })
            ));
        }
    }

    mod try_from_content_style_ref {
        use super::*;

        #[test]
        fn converts_content_style_to_style_def() {
            let style = ContentStyle {
                fg: Some(color::Fg(Color::LightBlue)),
                bg: Some(color::Bg(Color::Yellow)),
                attributes: vec![Style::Italic, Style::Underline],
            };

            let style_def = ContentStyleDef::try_from(&style).unwrap();
            assert_eq!(style_def.fg, Some(ColorDef::Named("lightblue".to_string())));
            assert_eq!(style_def.bg, Some(ColorDef::Named("yellow".to_string())));
            assert_eq!(style_def.ul, None);
            assert!(style_def.attributes.contains(&"italic".to_string()));
            assert!(style_def.attributes.contains(&"underline".to_string()));
        }

        #[test]
        fn rejects_ansi_value_for_serialization() {
            let style = ContentStyle {
                fg: Some(color::Fg(Color::AnsiValue(42))),
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
        fn rejects_underscored_attribute() {
            assert!(matches!(
                super::super::parse_attribute_token("no_underline"),
                Err(super::super::StyleError::UnsupportedAttributeToken { .. })
            ));
        }

        #[test]
        fn rejects_alias_attribute() {
            assert!(matches!(
                super::super::parse_attribute_token("underlined"),
                Err(super::super::StyleError::UnsupportedAttributeToken { .. })
            ));
        }
    }
}
