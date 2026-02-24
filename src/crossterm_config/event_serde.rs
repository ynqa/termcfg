use serde::{Deserialize, Deserializer, Serializer};

use crate::{
    crossterm::event::Event,
    event::{event_def::EventDef, format::event_to_shortcut, parse::parse_shortcut},
};

pub fn serialize<S>(event: &Event, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let event_def = EventDef::try_from(event).map_err(serde::ser::Error::custom)?;
    let shortcut = event_to_shortcut(event_def);
    serializer.serialize_str(&shortcut)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Event, D::Error>
where
    D: Deserializer<'de>,
{
    let shortcut = String::deserialize(deserializer)?;
    let event_def = parse_shortcut(&shortcut).map_err(serde::de::Error::custom)?;
    Ok(event_def.into())
}
