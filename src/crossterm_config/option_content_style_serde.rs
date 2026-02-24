use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{
    crossterm::style::ContentStyle,
    style::{
        format::content_style_to_string, parse::parse_content_style, style_def::ContentStyleDef,
    },
};

pub fn serialize<S>(style: &Option<ContentStyle>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let encoded = style
        .as_ref()
        .map(|style| {
            let style_def = ContentStyleDef::try_from(style).map_err(serde::ser::Error::custom)?;
            Ok::<_, S::Error>(content_style_to_string(&style_def))
        })
        .transpose()?;

    encoded.serialize(serializer)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<ContentStyle>, D::Error>
where
    D: Deserializer<'de>,
{
    let encoded = Option::<String>::deserialize(deserializer)?;

    encoded
        .map(|encoded| {
            let style_def = parse_content_style(&encoded).map_err(serde::de::Error::custom)?;
            ContentStyle::try_from(style_def).map_err(serde::de::Error::custom)
        })
        .transpose()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crossterm::style::Color;

    #[derive(Deserialize, serde::Serialize)]
    struct Config {
        #[serde(default)]
        #[serde(with = "crate::crossterm_config::option_content_style_serde")]
        style: Option<ContentStyle>,
    }

    mod deserialize {
        use super::*;

        #[test]
        fn deserializes_some_style_string() {
            let content = r#"style = "fg=red,bg=#102030,attr=bold""#;
            let config: Config = toml::from_str(content).unwrap();

            let style = config.style.expect("style should be Some");
            assert_eq!(style.foreground_color, Some(Color::Red));
            assert_eq!(
                style.background_color,
                Some(Color::Rgb {
                    r: 0x10,
                    g: 0x20,
                    b: 0x30,
                })
            );
        }

        #[test]
        fn deserializes_none_when_field_is_missing() {
            let content = r#""#;
            let config: Config = toml::from_str(content).unwrap();
            assert!(config.style.is_none());
        }
    }

    mod serialize {
        use super::*;

        #[test]
        fn serializes_some_style_string() {
            let style = ContentStyle {
                foreground_color: Some(Color::Blue),
                ..ContentStyle::default()
            };

            let config = Config { style: Some(style) };
            let serialized = toml::to_string(&config).unwrap();

            assert!(serialized.contains("fg=blue"));
        }

        #[test]
        fn serializes_none_as_absent_value() {
            let config = Config { style: None };
            let serialized = toml::to_string(&config).unwrap();
            assert!(!serialized.contains("style"));
        }
    }
}
