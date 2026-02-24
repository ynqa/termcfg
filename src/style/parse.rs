use crate::error::StyleError;

use super::style_def::{ColorDef, ContentStyleDef};

/// Parse a content style definition string into a structured `ContentStyleDef`.
/// e.g. "fg=red,bg=#00FF00,ul=#0C0C0C,attr=bold|underlined"
pub fn parse_content_style(input: &str) -> Result<ContentStyleDef, StyleError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Ok(ContentStyleDef::default());
    }

    let mut style = ContentStyleDef::default();

    for entry in trimmed.split(',') {
        let entry = entry.trim();
        if entry.is_empty() {
            return Err(StyleError::InvalidStyleEntry {
                entry: entry.to_string(),
            });
        }

        let (key, value) = entry
            .split_once('=')
            .ok_or_else(|| StyleError::InvalidStyleEntry {
                entry: entry.to_string(),
            })?;

        let key = key.trim().to_ascii_lowercase();
        let value = value.trim();
        if value.is_empty() {
            return Err(StyleError::EmptyStyleValue { key });
        }

        match key.as_str() {
            "fg" => assign_color(&mut style.fg, "fg", value)?,
            "bg" => assign_color(&mut style.bg, "bg", value)?,
            "ul" => assign_color(&mut style.ul, "ul", value)?,
            "attr" => style.attributes.extend(parse_attribute_tokens(value)?),
            _ => return Err(StyleError::UnknownStyleKey { key }),
        }
    }

    Ok(style)
}

/// Helper function to assign a color to a style slot, ensuring no duplicates.
fn assign_color(slot: &mut Option<ColorDef>, key: &str, value: &str) -> Result<(), StyleError> {
    if slot.is_some() {
        return Err(StyleError::DuplicateStyleKey {
            key: key.to_string(),
        });
    }
    *slot = Some(parse_color_value(value)?);
    Ok(())
}

/// Parse a color value, which can be either a named color or a hex code.
fn parse_color_value(value: &str) -> Result<ColorDef, StyleError> {
    if value.starts_with('#') {
        if let Some((r, g, b)) = parse_hex_code(value) {
            return Ok(ColorDef::Rgb { r, g, b });
        }
        return Err(StyleError::UnsupportedColorToken {
            token: value.to_string(),
        });
    }

    Ok(ColorDef::Named(value.to_string()))
}

/// Parse a hex color code in the format "#RRGGBB" into its RGB components.
fn parse_hex_code(value: &str) -> Option<(u8, u8, u8)> {
    if value.len() != 7 || !value.starts_with('#') {
        return None;
    }

    let hex = &value[1..];
    if !hex.chars().all(|ch| ch.is_ascii_hexdigit()) {
        return None;
    }

    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some((r, g, b))
}

/// Parse attribute tokens separated by '|' or '+' into a vector of attribute names.
fn parse_attribute_tokens(value: &str) -> Result<Vec<String>, StyleError> {
    let mut attributes = Vec::new();
    for token in value.split('|') {
        let token = token.trim();
        if token.is_empty() {
            return Err(StyleError::InvalidStyleEntry {
                entry: value.to_string(),
            });
        }
        attributes.push(token.to_string());
    }
    Ok(attributes)
}

#[cfg(test)]
mod tests {
    use super::*;

    mod parse_content_style {
        use super::*;

        #[test]
        fn parses_style() {
            let parsed =
                parse_content_style("fg=red,bg=#00FF00,ul=#0C0C0C,attr=bold|underlined").unwrap();

            assert_eq!(
                parsed,
                ContentStyleDef {
                    fg: Some(ColorDef::Named("red".to_string())),
                    bg: Some(ColorDef::Rgb {
                        r: 0x00,
                        g: 0xFF,
                        b: 0x00,
                    }),
                    ul: Some(ColorDef::Rgb {
                        r: 0x0C,
                        g: 0x0C,
                        b: 0x0C,
                    }),
                    attributes: vec!["bold".to_string(), "underlined".to_string()],
                }
            );
        }

        #[test]
        fn allows_empty_style() {
            assert_eq!(parse_content_style("").unwrap(), ContentStyleDef::default());
        }

        #[test]
        fn rejects_invalid_entry() {
            assert!(matches!(
                parse_content_style("fg=red,,bg=blue").unwrap_err(),
                StyleError::InvalidStyleEntry { .. }
            ));
        }
    }

    mod parse_color_value {
        use super::*;

        #[test]
        fn parses_hash_code() {
            let parsed = parse_color_value("#0A141E").unwrap();
            assert_eq!(
                parsed,
                ColorDef::Rgb {
                    r: 0x0A,
                    g: 0x14,
                    b: 0x1E
                }
            );
        }

        #[test]
        fn treats_numeric_as_named() {
            let parsed = parse_color_value("200").unwrap();
            assert_eq!(parsed, ColorDef::Named("200".to_string()));
        }

        #[test]
        fn rejects_invalid_hex_code() {
            assert!(matches!(
                parse_color_value("#12"),
                Err(StyleError::UnsupportedColorToken { .. })
            ));
        }
    }
}
