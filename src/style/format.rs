use super::style_def::{ColorDef, ContentStyleDef};

pub fn content_style_to_string(style: &ContentStyleDef) -> String {
    let mut parts: Vec<String> = Vec::new();

    if let Some(fg) = &style.fg {
        parts.push(format!("fg={}", color_to_string(fg)));
    }
    if let Some(bg) = &style.bg {
        parts.push(format!("bg={}", color_to_string(bg)));
    }
    if let Some(ul) = &style.ul {
        parts.push(format!("ul={}", color_to_string(ul)));
    }
    if !style.attributes.is_empty() {
        parts.push(format!("attr={}", style.attributes.join("|")));
    }

    parts.join(",")
}

fn color_to_string(color: &ColorDef) -> String {
    match color {
        ColorDef::Named(name) => name.clone(),
        ColorDef::Rgb { r, g, b } => format!("#{r:02X}{g:02X}{b:02X}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod content_style_to_string {
        use super::*;

        #[test]
        fn serializes_all_fields() {
            let style = ContentStyleDef {
                fg: Some(ColorDef::Named("red".to_string())),
                bg: Some(ColorDef::Rgb {
                    r: 0x11,
                    g: 0x22,
                    b: 0x33,
                }),
                ul: Some(ColorDef::Rgb {
                    r: 0x0D,
                    g: 0x0D,
                    b: 0x0D,
                }),
                attributes: vec!["bold".to_string(), "underlined".to_string()],
            };

            assert_eq!(
                content_style_to_string(&style),
                "fg=red,bg=#112233,ul=#0D0D0D,attr=bold|underlined"
            );
        }

        #[test]
        fn serializes_empty_style() {
            assert_eq!(content_style_to_string(&ContentStyleDef::default()), "");
        }
    }
}
