use std::collections::HashSet;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{
    event::{event_def::EventDef, format::event_to_shortcut, parse::parse_shortcut},
    termion::event::Event,
};

#[derive(Deserialize)]
#[serde(untagged)]
enum EventSetInput {
    Shortcut(String),
    Shortcuts(Vec<String>),
}

pub fn serialize<S>(events: &HashSet<Event>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut shortcuts: Vec<String> = events
        .iter()
        .map(|event| {
            EventDef::try_from(event)
                .map(event_to_shortcut)
                .map_err(serde::ser::Error::custom)
        })
        .collect::<Result<_, _>>()?;

    shortcuts.sort();
    shortcuts.serialize(serializer)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<HashSet<Event>, D::Error>
where
    D: Deserializer<'de>,
{
    match EventSetInput::deserialize(deserializer)? {
        EventSetInput::Shortcut(shortcut) => {
            let event_def = parse_shortcut(&shortcut).map_err(serde::de::Error::custom)?;
            let event = Event::try_from(event_def).map_err(serde::de::Error::custom)?;
            Ok(HashSet::from([event]))
        }
        EventSetInput::Shortcuts(shortcuts) => {
            let mut events = HashSet::with_capacity(shortcuts.len());
            for shortcut in shortcuts {
                let event_def = parse_shortcut(&shortcut).map_err(serde::de::Error::custom)?;
                let event = Event::try_from(event_def).map_err(serde::de::Error::custom)?;
                events.insert(event);
            }
            Ok(events)
        }
    }
}
