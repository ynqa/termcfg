use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{
    style::{
        format::content_style_to_string, parse::parse_content_style, style_def::ContentStyleDef,
    },
    termion_config::content_style_serde::ContentStyle,
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
    use crate::termion_config::content_style_serde::{Color, Style};

    #[derive(Deserialize, serde::Serialize)]
    struct Config {
        #[serde(default)]
        #[serde(with = "crate::termion_config::option_content_style_serde")]
        style: Option<ContentStyle>,
    }

    mod deserialize {
        use super::*;

        #[test]
        fn deserializes_some_style_string() {
            let content = r#"style = "fg=red,bg=#102030,attr=bold""#;
            let config: Config = toml::from_str(content).unwrap();

            let style = config.style.expect("style should be Some");
            assert!(matches!(
                style.fg,
                Some(crate::termion::color::Fg(Color::Red))
            ));
            assert!(matches!(
                style.bg,
                Some(crate::termion::color::Bg(Color::Rgb(0x10, 0x20, 0x30)))
            ));
            assert!(style.attributes.iter().any(|s| matches!(s, Style::Bold)));
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
                fg: Some(crate::termion::color::Fg(Color::LightBlue)),
                bg: None,
                attributes: vec![Style::Bold],
            };

            let config = Config { style: Some(style) };
            let serialized = toml::to_string(&config).unwrap();

            assert!(serialized.contains("fg=lightblue"));
            assert!(serialized.contains("attr=bold"));
        }
    }
}
