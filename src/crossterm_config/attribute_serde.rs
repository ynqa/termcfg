use serde::{Deserialize, Deserializer, Serializer};

use crate::{
    crossterm::style::Attribute,
    crossterm_config::convert::{attribute_to_token, parse_attribute_token},
    error::StyleError,
};

pub fn serialize<S>(attribute: &Attribute, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let token = attribute_to_token(*attribute).ok_or_else(|| {
        serde::ser::Error::custom(StyleError::UnsupportedAttributeToken {
            token: format!("{attribute:?}"),
        })
    })?;

    serializer.serialize_str(token)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Attribute, D::Error>
where
    D: Deserializer<'de>,
{
    let encoded = String::deserialize(deserializer)?;
    parse_attribute_token(&encoded).map_err(serde::de::Error::custom)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Deserialize, serde::Serialize)]
    struct Config {
        #[serde(with = "crate::crossterm_config::attribute_serde")]
        attr: Attribute,
    }

    mod deserialize {
        use super::*;

        #[test]
        fn deserializes_lowercase_attribute() {
            let content = r#"attr = "underlined""#;
            let config: Config = toml::from_str(content).unwrap();
            assert_eq!(config.attr, Attribute::Underlined);
        }

        #[test]
        fn deserializes_case_insensitive_attribute() {
            let content = r#"attr = "NoBold""#;
            let config: Config = toml::from_str(content).unwrap();
            assert_eq!(config.attr, Attribute::NoBold);
        }
    }

    mod serialize {
        use super::*;

        #[test]
        fn serializes_attribute_to_lowercase_token() {
            let config = Config {
                attr: Attribute::NoBold,
            };
            let serialized = toml::to_string(&config).unwrap();
            assert!(serialized.contains("attr = \"nobold\""));
        }
    }
}
