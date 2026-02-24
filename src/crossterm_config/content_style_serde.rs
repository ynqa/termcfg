use serde::{Deserialize, Deserializer, Serializer};

use crate::{
    crossterm::style::ContentStyle,
    style::{
        format::content_style_to_string, parse::parse_content_style, style_def::ContentStyleDef,
    },
};

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
    use crate::crossterm::style::{Attribute, Color};

    #[derive(Deserialize, serde::Serialize)]
    struct Config {
        #[serde(with = "crate::crossterm_config::content_style_serde")]
        style: ContentStyle,
    }

    mod deserialize {
        use super::*;

        #[test]
        fn deserializes_style_string() {
            let content = r#"style = "fg=red,bg=#102030,ul=#0C0C0C,attr=bold|underlined|italic""#;
            let config: Config = toml::from_str(content).unwrap();

            assert_eq!(config.style.foreground_color, Some(Color::Red));
            assert_eq!(
                config.style.background_color,
                Some(Color::Rgb {
                    r: 0x10,
                    g: 0x20,
                    b: 0x30,
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
        }
    }

    mod serialize {
        use super::*;

        #[test]
        fn serializes_style_string() {
            let mut style = ContentStyle {
                foreground_color: Some(Color::Blue),
                background_color: Some(Color::DarkGrey),
                underline_color: Some(Color::Rgb {
                    r: 255,
                    g: 0,
                    b: 255,
                }),
                ..ContentStyle::default()
            };
            style.attributes.set(Attribute::Bold);
            style.attributes.set(Attribute::Underlined);

            let config = Config { style };
            let serialized = toml::to_string(&config).unwrap();

            assert!(serialized.contains("fg=blue"));
            assert!(serialized.contains("bg=darkgrey"));
            assert!(serialized.contains("ul=#FF00FF"));
            assert!(serialized.contains("attr=bold"));
            assert!(serialized.contains("underlined"));
        }
    }
}
