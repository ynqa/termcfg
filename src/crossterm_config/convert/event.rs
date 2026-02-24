use crate::{
    crossterm::event::{
        Event, KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
    },
    error::ShortcutError,
    event::event_def::{
        EventDef, KeyCodeDef, KeyEventDef, ModifiersDef, MouseButtonDef, MouseEventDef,
        MouseEventKindDef,
    },
};

impl From<EventDef> for Event {
    fn from(event_def: EventDef) -> Self {
        match event_def {
            EventDef::Key(key_event_def) => Event::Key(KeyEvent::new(
                key_event_def.code.into(),
                key_event_def.modifiers.into(),
            )),
            EventDef::Mouse(mouse_event_def) => Event::Mouse(MouseEvent {
                kind: mouse_event_def.kind.into(),
                column: 0,
                row: 0,
                modifiers: mouse_event_def.modifiers.into(),
            }),
        }
    }
}

impl TryFrom<&Event> for EventDef {
    type Error = ShortcutError;

    fn try_from(event: &Event) -> Result<Self, Self::Error> {
        match event {
            Event::Key(key_event) => Ok(EventDef::Key(KeyEventDef {
                code: key_event.code.try_into()?,
                modifiers: key_event.modifiers.into(),
            })),
            Event::Mouse(mouse_event) => Ok(EventDef::Mouse(MouseEventDef {
                kind: mouse_event.kind.into(),
                modifiers: mouse_event.modifiers.into(),
            })),
            _ => Err(Self::Error::InvalidEventForShortcut),
        }
    }
}

impl From<KeyModifiers> for ModifiersDef {
    fn from(modifiers: KeyModifiers) -> Self {
        let mut result = Self::NONE;
        if modifiers.contains(KeyModifiers::SHIFT) {
            result.insert(Self::SHIFT);
        }
        if modifiers.contains(KeyModifiers::CONTROL) {
            result.insert(Self::CONTROL);
        }
        if modifiers.contains(KeyModifiers::ALT) {
            result.insert(Self::ALT);
        }
        if modifiers.contains(KeyModifiers::SUPER) {
            result.insert(Self::SUPER);
        }
        if modifiers.contains(KeyModifiers::HYPER) {
            result.insert(Self::HYPER);
        }
        if modifiers.contains(KeyModifiers::META) {
            result.insert(Self::META);
        }
        result
    }
}

impl From<ModifiersDef> for KeyModifiers {
    fn from(modifiers: ModifiersDef) -> Self {
        let mut result = KeyModifiers::NONE;
        if modifiers.contains(ModifiersDef::SHIFT) {
            result.insert(KeyModifiers::SHIFT);
        }
        if modifiers.contains(ModifiersDef::CONTROL) {
            result.insert(KeyModifiers::CONTROL);
        }
        if modifiers.contains(ModifiersDef::ALT) {
            result.insert(KeyModifiers::ALT);
        }
        if modifiers.contains(ModifiersDef::SUPER) {
            result.insert(KeyModifiers::SUPER);
        }
        if modifiers.contains(ModifiersDef::HYPER) {
            result.insert(KeyModifiers::HYPER);
        }
        if modifiers.contains(ModifiersDef::META) {
            result.insert(KeyModifiers::META);
        }
        result
    }
}

impl From<KeyCodeDef> for KeyCode {
    fn from(code: KeyCodeDef) -> Self {
        match code {
            KeyCodeDef::Backspace => KeyCode::Backspace,
            KeyCodeDef::Enter => KeyCode::Enter,
            KeyCodeDef::Left => KeyCode::Left,
            KeyCodeDef::Right => KeyCode::Right,
            KeyCodeDef::Up => KeyCode::Up,
            KeyCodeDef::Down => KeyCode::Down,
            KeyCodeDef::Home => KeyCode::Home,
            KeyCodeDef::End => KeyCode::End,
            KeyCodeDef::PageUp => KeyCode::PageUp,
            KeyCodeDef::PageDown => KeyCode::PageDown,
            KeyCodeDef::Tab => KeyCode::Tab,
            KeyCodeDef::BackTab => KeyCode::BackTab,
            KeyCodeDef::Delete => KeyCode::Delete,
            KeyCodeDef::Insert => KeyCode::Insert,
            KeyCodeDef::F(num) => KeyCode::F(num),
            KeyCodeDef::Char(ch) => KeyCode::Char(ch),
            KeyCodeDef::Null => KeyCode::Null,
            KeyCodeDef::Esc => KeyCode::Esc,
            KeyCodeDef::CapsLock => KeyCode::CapsLock,
            KeyCodeDef::ScrollLock => KeyCode::ScrollLock,
            KeyCodeDef::NumLock => KeyCode::NumLock,
            KeyCodeDef::PrintScreen => KeyCode::PrintScreen,
            KeyCodeDef::Pause => KeyCode::Pause,
            KeyCodeDef::Menu => KeyCode::Menu,
            KeyCodeDef::KeypadBegin => KeyCode::KeypadBegin,
        }
    }
}

impl TryFrom<KeyCode> for KeyCodeDef {
    type Error = ShortcutError;

    fn try_from(code: KeyCode) -> Result<Self, Self::Error> {
        match code {
            KeyCode::Backspace => Ok(Self::Backspace),
            KeyCode::Enter => Ok(Self::Enter),
            KeyCode::Left => Ok(Self::Left),
            KeyCode::Right => Ok(Self::Right),
            KeyCode::Up => Ok(Self::Up),
            KeyCode::Down => Ok(Self::Down),
            KeyCode::Home => Ok(Self::Home),
            KeyCode::End => Ok(Self::End),
            KeyCode::PageUp => Ok(Self::PageUp),
            KeyCode::PageDown => Ok(Self::PageDown),
            KeyCode::Tab => Ok(Self::Tab),
            KeyCode::BackTab => Ok(Self::BackTab),
            KeyCode::Delete => Ok(Self::Delete),
            KeyCode::Insert => Ok(Self::Insert),
            KeyCode::F(num) => Ok(Self::F(num)),
            KeyCode::Char(ch) => Ok(Self::Char(ch)),
            KeyCode::Null => Ok(Self::Null),
            KeyCode::Esc => Ok(Self::Esc),
            KeyCode::CapsLock => Ok(Self::CapsLock),
            KeyCode::ScrollLock => Ok(Self::ScrollLock),
            KeyCode::NumLock => Ok(Self::NumLock),
            KeyCode::PrintScreen => Ok(Self::PrintScreen),
            KeyCode::Pause => Ok(Self::Pause),
            KeyCode::Menu => Ok(Self::Menu),
            KeyCode::KeypadBegin => Ok(Self::KeypadBegin),
            KeyCode::Media(_) | KeyCode::Modifier(_) => {
                Err(Self::Error::UnsupportedKeyCodeForShortcut)
            }
        }
    }
}

impl From<MouseButtonDef> for MouseButton {
    fn from(button: MouseButtonDef) -> Self {
        match button {
            MouseButtonDef::Left => MouseButton::Left,
            MouseButtonDef::Right => MouseButton::Right,
            MouseButtonDef::Middle => MouseButton::Middle,
        }
    }
}

impl From<MouseButton> for MouseButtonDef {
    fn from(button: MouseButton) -> Self {
        match button {
            MouseButton::Left => Self::Left,
            MouseButton::Right => Self::Right,
            MouseButton::Middle => Self::Middle,
        }
    }
}

impl From<MouseEventKindDef> for MouseEventKind {
    fn from(kind: MouseEventKindDef) -> Self {
        match kind {
            MouseEventKindDef::Down(button) => MouseEventKind::Down(button.into()),
            MouseEventKindDef::Up(button) => MouseEventKind::Up(button.into()),
            MouseEventKindDef::Drag(button) => MouseEventKind::Drag(button.into()),
            MouseEventKindDef::Moved => MouseEventKind::Moved,
            MouseEventKindDef::ScrollDown => MouseEventKind::ScrollDown,
            MouseEventKindDef::ScrollUp => MouseEventKind::ScrollUp,
            MouseEventKindDef::ScrollLeft => MouseEventKind::ScrollLeft,
            MouseEventKindDef::ScrollRight => MouseEventKind::ScrollRight,
        }
    }
}

impl From<MouseEventKind> for MouseEventKindDef {
    fn from(kind: MouseEventKind) -> Self {
        match kind {
            MouseEventKind::Down(button) => Self::Down(button.into()),
            MouseEventKind::Up(button) => Self::Up(button.into()),
            MouseEventKind::Drag(button) => Self::Drag(button.into()),
            MouseEventKind::Moved => Self::Moved,
            MouseEventKind::ScrollDown => Self::ScrollDown,
            MouseEventKind::ScrollUp => Self::ScrollUp,
            MouseEventKind::ScrollLeft => Self::ScrollLeft,
            MouseEventKind::ScrollRight => Self::ScrollRight,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crossterm::event::{KeyEventKind, KeyEventState, MediaKeyCode};

    mod try_from_event_ref_for_event_def {
        use super::*;

        #[test]
        fn rejects_unsupported_key_code() {
            let event = Event::Key(KeyEvent {
                code: KeyCode::Media(MediaKeyCode::Play),
                modifiers: KeyModifiers::empty(),
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            });

            assert_eq!(
                EventDef::try_from(&event).unwrap_err(),
                ShortcutError::UnsupportedKeyCodeForShortcut
            );
        }

        #[test]
        fn rejects_invalid_event_type() {
            assert_eq!(
                EventDef::try_from(&Event::FocusGained).unwrap_err(),
                ShortcutError::InvalidEventForShortcut
            );
        }

        #[test]
        fn ignores_non_modifier_key_fields() {
            let event = Event::Key(KeyEvent {
                code: KeyCode::Char('x'),
                modifiers: KeyModifiers::empty(),
                kind: KeyEventKind::Repeat,
                state: KeyEventState::NUM_LOCK,
            });

            assert_eq!(
                EventDef::try_from(&event).unwrap(),
                EventDef::Key(KeyEventDef {
                    code: KeyCodeDef::Char('x'),
                    modifiers: ModifiersDef::NONE,
                })
            );
        }

        #[test]
        fn ignores_mouse_position() {
            let event = Event::Mouse(MouseEvent {
                kind: MouseEventKind::Moved,
                column: 10,
                row: 3,
                modifiers: KeyModifiers::empty(),
            });

            assert_eq!(
                EventDef::try_from(&event).unwrap(),
                EventDef::Mouse(MouseEventDef {
                    kind: MouseEventKindDef::Moved,
                    modifiers: ModifiersDef::NONE,
                })
            );
        }
    }
}
