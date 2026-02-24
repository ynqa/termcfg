use crate::{
    error::ShortcutError,
    event::event_def::{
        EventDef, KeyCodeDef, KeyEventDef, ModifiersDef, MouseButtonDef, MouseEventDef,
        MouseEventKindDef,
    },
    termion::event::{Event, Key, MouseButton, MouseEvent},
};

const DEFAULT_MOUSE_COLUMN: u16 = 1;
const DEFAULT_MOUSE_ROW: u16 = 1;

impl TryFrom<EventDef> for Event {
    type Error = ShortcutError;

    fn try_from(event_def: EventDef) -> Result<Self, Self::Error> {
        match event_def {
            EventDef::Key(key_event_def) => Ok(Event::Key(key_event_def.try_into()?)),
            EventDef::Mouse(mouse_event_def) => Ok(Event::Mouse(mouse_event_def.try_into()?)),
        }
    }
}

impl TryFrom<&Event> for EventDef {
    type Error = ShortcutError;

    fn try_from(event: &Event) -> Result<Self, Self::Error> {
        match event {
            Event::Key(key) => Ok(EventDef::Key((*key).try_into()?)),
            Event::Mouse(mouse_event) => Ok(EventDef::Mouse((*mouse_event).try_into()?)),
            Event::Unsupported(_) => Err(Self::Error::InvalidEventForShortcut),
        }
    }
}

impl TryFrom<KeyEventDef> for Key {
    type Error = ShortcutError;

    fn try_from(key_event_def: KeyEventDef) -> Result<Self, Self::Error> {
        let modifiers = key_event_def.modifiers;

        let key = match (key_event_def.code, modifiers) {
            (KeyCodeDef::Backspace, ModifiersDef::NONE) => Key::Backspace,
            (KeyCodeDef::Enter, ModifiersDef::NONE) => Key::Char('\n'),
            (KeyCodeDef::Left, ModifiersDef::NONE) => Key::Left,
            (KeyCodeDef::Left, ModifiersDef::SHIFT) => Key::ShiftLeft,
            (KeyCodeDef::Left, ModifiersDef::ALT) => Key::AltLeft,
            (KeyCodeDef::Left, ModifiersDef::CONTROL) => Key::CtrlLeft,
            (KeyCodeDef::Right, ModifiersDef::NONE) => Key::Right,
            (KeyCodeDef::Right, ModifiersDef::SHIFT) => Key::ShiftRight,
            (KeyCodeDef::Right, ModifiersDef::ALT) => Key::AltRight,
            (KeyCodeDef::Right, ModifiersDef::CONTROL) => Key::CtrlRight,
            (KeyCodeDef::Up, ModifiersDef::NONE) => Key::Up,
            (KeyCodeDef::Up, ModifiersDef::SHIFT) => Key::ShiftUp,
            (KeyCodeDef::Up, ModifiersDef::ALT) => Key::AltUp,
            (KeyCodeDef::Up, ModifiersDef::CONTROL) => Key::CtrlUp,
            (KeyCodeDef::Down, ModifiersDef::NONE) => Key::Down,
            (KeyCodeDef::Down, ModifiersDef::SHIFT) => Key::ShiftDown,
            (KeyCodeDef::Down, ModifiersDef::ALT) => Key::AltDown,
            (KeyCodeDef::Down, ModifiersDef::CONTROL) => Key::CtrlDown,
            (KeyCodeDef::Home, ModifiersDef::NONE) => Key::Home,
            (KeyCodeDef::Home, ModifiersDef::CONTROL) => Key::CtrlHome,
            (KeyCodeDef::End, ModifiersDef::NONE) => Key::End,
            (KeyCodeDef::End, ModifiersDef::CONTROL) => Key::CtrlEnd,
            (KeyCodeDef::PageUp, ModifiersDef::NONE) => Key::PageUp,
            (KeyCodeDef::PageDown, ModifiersDef::NONE) => Key::PageDown,
            (KeyCodeDef::Tab, ModifiersDef::NONE) => Key::Char('\t'),
            (KeyCodeDef::Tab, ModifiersDef::SHIFT) => Key::BackTab,
            (KeyCodeDef::BackTab, ModifiersDef::NONE) => Key::BackTab,
            (KeyCodeDef::Delete, ModifiersDef::NONE) => Key::Delete,
            (KeyCodeDef::Insert, ModifiersDef::NONE) => Key::Insert,
            (KeyCodeDef::F(num), ModifiersDef::NONE) => Key::F(num),
            (KeyCodeDef::Char(ch), ModifiersDef::NONE) => Key::Char(ch),
            (KeyCodeDef::Char(ch), ModifiersDef::SHIFT) => Key::Char(ch.to_ascii_uppercase()),
            (KeyCodeDef::Char(ch), ModifiersDef::ALT) => Key::Alt(ch),
            (KeyCodeDef::Char(ch), ModifiersDef::CONTROL) => Key::Ctrl(ch),
            (KeyCodeDef::Null, ModifiersDef::NONE) => Key::Null,
            (KeyCodeDef::Esc, ModifiersDef::NONE) => Key::Esc,
            _ => return Err(Self::Error::UnsupportedKeyCodeForShortcut),
        };

        Ok(key)
    }
}

impl TryFrom<Key> for KeyEventDef {
    type Error = ShortcutError;

    fn try_from(key: Key) -> Result<Self, Self::Error> {
        let (code, modifiers) = match key {
            Key::Backspace => (KeyCodeDef::Backspace, ModifiersDef::NONE),
            Key::Left => (KeyCodeDef::Left, ModifiersDef::NONE),
            Key::ShiftLeft => (KeyCodeDef::Left, ModifiersDef::SHIFT),
            Key::AltLeft => (KeyCodeDef::Left, ModifiersDef::ALT),
            Key::CtrlLeft => (KeyCodeDef::Left, ModifiersDef::CONTROL),
            Key::Right => (KeyCodeDef::Right, ModifiersDef::NONE),
            Key::ShiftRight => (KeyCodeDef::Right, ModifiersDef::SHIFT),
            Key::AltRight => (KeyCodeDef::Right, ModifiersDef::ALT),
            Key::CtrlRight => (KeyCodeDef::Right, ModifiersDef::CONTROL),
            Key::Up => (KeyCodeDef::Up, ModifiersDef::NONE),
            Key::ShiftUp => (KeyCodeDef::Up, ModifiersDef::SHIFT),
            Key::AltUp => (KeyCodeDef::Up, ModifiersDef::ALT),
            Key::CtrlUp => (KeyCodeDef::Up, ModifiersDef::CONTROL),
            Key::Down => (KeyCodeDef::Down, ModifiersDef::NONE),
            Key::ShiftDown => (KeyCodeDef::Down, ModifiersDef::SHIFT),
            Key::AltDown => (KeyCodeDef::Down, ModifiersDef::ALT),
            Key::CtrlDown => (KeyCodeDef::Down, ModifiersDef::CONTROL),
            Key::Home => (KeyCodeDef::Home, ModifiersDef::NONE),
            Key::CtrlHome => (KeyCodeDef::Home, ModifiersDef::CONTROL),
            Key::End => (KeyCodeDef::End, ModifiersDef::NONE),
            Key::CtrlEnd => (KeyCodeDef::End, ModifiersDef::CONTROL),
            Key::PageUp => (KeyCodeDef::PageUp, ModifiersDef::NONE),
            Key::PageDown => (KeyCodeDef::PageDown, ModifiersDef::NONE),
            Key::BackTab => (KeyCodeDef::BackTab, ModifiersDef::NONE),
            Key::Delete => (KeyCodeDef::Delete, ModifiersDef::NONE),
            Key::Insert => (KeyCodeDef::Insert, ModifiersDef::NONE),
            Key::F(num) => (KeyCodeDef::F(num), ModifiersDef::NONE),
            Key::Char('\n') => (KeyCodeDef::Enter, ModifiersDef::NONE),
            Key::Char('\t') => (KeyCodeDef::Tab, ModifiersDef::NONE),
            Key::Char(ch) => (KeyCodeDef::Char(ch), ModifiersDef::NONE),
            Key::Alt(ch) => (KeyCodeDef::Char(ch), ModifiersDef::ALT),
            Key::Ctrl(ch) => (KeyCodeDef::Char(ch), ModifiersDef::CONTROL),
            Key::Null => (KeyCodeDef::Null, ModifiersDef::NONE),
            Key::Esc => (KeyCodeDef::Esc, ModifiersDef::NONE),
            _ => return Err(Self::Error::UnsupportedKeyCodeForShortcut),
        };

        Ok(KeyEventDef { code, modifiers })
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

impl TryFrom<MouseEventDef> for MouseEvent {
    type Error = ShortcutError;

    fn try_from(mouse_event_def: MouseEventDef) -> Result<Self, Self::Error> {
        if mouse_event_def.modifiers != ModifiersDef::NONE {
            return Err(Self::Error::UnsupportedKeyCodeForShortcut);
        }

        let event = match mouse_event_def.kind {
            MouseEventKindDef::Down(button) => {
                MouseEvent::Press(button.into(), DEFAULT_MOUSE_COLUMN, DEFAULT_MOUSE_ROW)
            }
            MouseEventKindDef::Up(_) | MouseEventKindDef::Drag(_) | MouseEventKindDef::Moved => {
                return Err(Self::Error::UnsupportedKeyCodeForShortcut);
            }
            MouseEventKindDef::ScrollDown => MouseEvent::Press(
                MouseButton::WheelDown,
                DEFAULT_MOUSE_COLUMN,
                DEFAULT_MOUSE_ROW,
            ),
            MouseEventKindDef::ScrollUp => MouseEvent::Press(
                MouseButton::WheelUp,
                DEFAULT_MOUSE_COLUMN,
                DEFAULT_MOUSE_ROW,
            ),
            MouseEventKindDef::ScrollLeft => MouseEvent::Press(
                MouseButton::WheelLeft,
                DEFAULT_MOUSE_COLUMN,
                DEFAULT_MOUSE_ROW,
            ),
            MouseEventKindDef::ScrollRight => MouseEvent::Press(
                MouseButton::WheelRight,
                DEFAULT_MOUSE_COLUMN,
                DEFAULT_MOUSE_ROW,
            ),
        };

        Ok(event)
    }
}

impl TryFrom<MouseEvent> for MouseEventDef {
    type Error = ShortcutError;

    fn try_from(mouse_event: MouseEvent) -> Result<Self, Self::Error> {
        let kind = match mouse_event {
            MouseEvent::Press(button, _, _) => match button {
                MouseButton::Left => MouseEventKindDef::Down(MouseButtonDef::Left),
                MouseButton::Right => MouseEventKindDef::Down(MouseButtonDef::Right),
                MouseButton::Middle => MouseEventKindDef::Down(MouseButtonDef::Middle),
                MouseButton::WheelDown => MouseEventKindDef::ScrollDown,
                MouseButton::WheelUp => MouseEventKindDef::ScrollUp,
                MouseButton::WheelLeft => MouseEventKindDef::ScrollLeft,
                MouseButton::WheelRight => MouseEventKindDef::ScrollRight,
            },
            MouseEvent::Release(_, _) | MouseEvent::Hold(_, _) => {
                return Err(Self::Error::UnsupportedKeyCodeForShortcut);
            }
        };

        Ok(MouseEventDef {
            kind,
            modifiers: ModifiersDef::NONE,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod try_from_event_ref_for_event_def {
        use super::*;

        #[test]
        fn parses_shift_arrow_key() {
            let event = Event::Key(Key::ShiftDown);
            assert_eq!(
                EventDef::try_from(&event).unwrap(),
                EventDef::Key(KeyEventDef {
                    code: KeyCodeDef::Down,
                    modifiers: ModifiersDef::SHIFT,
                })
            );
        }

        #[test]
        fn rejects_unsupported_event_type() {
            assert_eq!(
                EventDef::try_from(&Event::Unsupported(vec![0x1B])).unwrap_err(),
                ShortcutError::InvalidEventForShortcut
            );
        }

        #[test]
        fn ignores_mouse_position() {
            let event = Event::Mouse(MouseEvent::Press(MouseButton::WheelUp, 10, 3));

            assert_eq!(
                EventDef::try_from(&event).unwrap(),
                EventDef::Mouse(MouseEventDef {
                    kind: MouseEventKindDef::ScrollUp,
                    modifiers: ModifiersDef::NONE,
                })
            );
        }

        #[test]
        fn rejects_release_mouse_event() {
            let event = Event::Mouse(MouseEvent::Release(10, 3));
            assert_eq!(
                EventDef::try_from(&event).unwrap_err(),
                ShortcutError::UnsupportedKeyCodeForShortcut
            );
        }

        #[test]
        fn rejects_hold_mouse_event() {
            let event = Event::Mouse(MouseEvent::Hold(10, 3));
            assert_eq!(
                EventDef::try_from(&event).unwrap_err(),
                ShortcutError::UnsupportedKeyCodeForShortcut
            );
        }
    }

    mod try_from_event_def_for_event {
        use super::*;

        #[test]
        fn rejects_unsupported_key_code_for_termion() {
            let event_def = EventDef::Key(KeyEventDef {
                code: KeyCodeDef::CapsLock,
                modifiers: ModifiersDef::NONE,
            });
            assert_eq!(
                Event::try_from(event_def).unwrap_err(),
                ShortcutError::UnsupportedKeyCodeForShortcut
            );
        }

        #[test]
        fn rejects_unsupported_modifiers_for_termion() {
            let event_def = EventDef::Key(KeyEventDef {
                code: KeyCodeDef::Char('x'),
                modifiers: ModifiersDef::SUPER,
            });
            assert_eq!(
                Event::try_from(event_def).unwrap_err(),
                ShortcutError::UnsupportedKeyCodeForShortcut
            );
        }

        #[test]
        fn rejects_mouse_up_for_termion() {
            let event_def = EventDef::Mouse(MouseEventDef {
                kind: MouseEventKindDef::Up(MouseButtonDef::Left),
                modifiers: ModifiersDef::NONE,
            });
            assert_eq!(
                Event::try_from(event_def).unwrap_err(),
                ShortcutError::UnsupportedKeyCodeForShortcut
            );
        }

        #[test]
        fn rejects_mouse_drag_for_termion() {
            let event_def = EventDef::Mouse(MouseEventDef {
                kind: MouseEventKindDef::Drag(MouseButtonDef::Right),
                modifiers: ModifiersDef::NONE,
            });
            assert_eq!(
                Event::try_from(event_def).unwrap_err(),
                ShortcutError::UnsupportedKeyCodeForShortcut
            );
        }

        #[test]
        fn rejects_mouse_moved_for_termion() {
            let event_def = EventDef::Mouse(MouseEventDef {
                kind: MouseEventKindDef::Moved,
                modifiers: ModifiersDef::NONE,
            });
            assert_eq!(
                Event::try_from(event_def).unwrap_err(),
                ShortcutError::UnsupportedKeyCodeForShortcut
            );
        }
    }
}
