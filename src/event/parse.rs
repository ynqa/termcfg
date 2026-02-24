use crate::error::ShortcutError;

use super::event_def::{
    EventDef, KeyCodeDef, KeyEventDef, ModifiersDef, MouseButtonDef, MouseEventDef,
    MouseEventKindDef,
};

/// Parse a shortcut string into an `EventDef`.
pub fn parse_shortcut(input: &str) -> Result<EventDef, ShortcutError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(ShortcutError::EmptyShortcut);
    }

    let mut modifiers = ModifiersDef::NONE;
    let mut parsed_event = None;

    for token in trimmed.split('+').map(str::trim) {
        if token.is_empty() {
            return Err(ShortcutError::InvalidShortcut {
                shortcut: input.to_string(),
            });
        }

        if let Some(modifier) = parse_modifier(token) {
            if parsed_event.is_some() {
                return Err(ShortcutError::ModifierAfterKeyCode {
                    shortcut: input.to_string(),
                });
            }
            modifiers.insert(modifier);
            continue;
        }

        if parsed_event.is_some() {
            return Err(ShortcutError::MultipleKeyCodes {
                shortcut: input.to_string(),
            });
        }

        parsed_event = Some(parse_event_token(token, modifiers).ok_or_else(|| {
            ShortcutError::UnsupportedKeyToken {
                token: token.to_string(),
            }
        })?);
    }

    parsed_event.ok_or_else(|| ShortcutError::MissingKeyCode {
        shortcut: input.to_string(),
    })
}

fn parse_modifier(token: &str) -> Option<ModifiersDef> {
    match token.to_ascii_lowercase().as_str() {
        "shift" => Some(ModifiersDef::SHIFT),
        "ctrl" => Some(ModifiersDef::CONTROL),
        "alt" => Some(ModifiersDef::ALT),
        "super" => Some(ModifiersDef::SUPER),
        "hyper" => Some(ModifiersDef::HYPER),
        "meta" => Some(ModifiersDef::META),
        _ => None,
    }
}

fn parse_key_code(token: &str, modifiers: ModifiersDef) -> Option<KeyCodeDef> {
    let normalized = token.to_ascii_lowercase();
    let code = match normalized.as_str() {
        "backspace" => KeyCodeDef::Backspace,
        "enter" => KeyCodeDef::Enter,
        "left" => KeyCodeDef::Left,
        "right" => KeyCodeDef::Right,
        "up" => KeyCodeDef::Up,
        "down" => KeyCodeDef::Down,
        "home" => KeyCodeDef::Home,
        "end" => KeyCodeDef::End,
        "pageup" => KeyCodeDef::PageUp,
        "pagedown" => KeyCodeDef::PageDown,
        "tab" => KeyCodeDef::Tab,
        "backtab" => KeyCodeDef::BackTab,
        "delete" => KeyCodeDef::Delete,
        "insert" => KeyCodeDef::Insert,
        "esc" => KeyCodeDef::Esc,
        "space" => KeyCodeDef::Char(' '),
        "plus" => KeyCodeDef::Char('+'),
        "minus" => KeyCodeDef::Char('-'),
        _ => {
            if let Some(raw) = normalized.strip_prefix('f')
                && let Ok(num) = raw.parse::<u8>()
            {
                return Some(KeyCodeDef::F(num));
            }

            let mut chars = token.chars();
            if let (Some(ch), None) = (chars.next(), chars.next()) {
                if ch.is_ascii_alphabetic() {
                    return if modifiers.contains(ModifiersDef::SHIFT) {
                        Some(KeyCodeDef::Char(ch.to_ascii_uppercase()))
                    } else {
                        Some(KeyCodeDef::Char(ch.to_ascii_lowercase()))
                    };
                }
                return Some(KeyCodeDef::Char(ch));
            }

            return None;
        }
    };

    Some(code)
}

fn parse_mouse_event_kind(token: &str) -> Option<MouseEventKindDef> {
    match token.to_ascii_lowercase().as_str() {
        "leftdown" => Some(MouseEventKindDef::Down(MouseButtonDef::Left)),
        "rightdown" => Some(MouseEventKindDef::Down(MouseButtonDef::Right)),
        "middledown" => Some(MouseEventKindDef::Down(MouseButtonDef::Middle)),
        "leftup" => Some(MouseEventKindDef::Up(MouseButtonDef::Left)),
        "rightup" => Some(MouseEventKindDef::Up(MouseButtonDef::Right)),
        "middleup" => Some(MouseEventKindDef::Up(MouseButtonDef::Middle)),
        "leftdrag" => Some(MouseEventKindDef::Drag(MouseButtonDef::Left)),
        "rightdrag" => Some(MouseEventKindDef::Drag(MouseButtonDef::Right)),
        "middledrag" => Some(MouseEventKindDef::Drag(MouseButtonDef::Middle)),
        "moved" => Some(MouseEventKindDef::Moved),
        "scrolldown" => Some(MouseEventKindDef::ScrollDown),
        "scrollup" => Some(MouseEventKindDef::ScrollUp),
        "scrollleft" => Some(MouseEventKindDef::ScrollLeft),
        "scrollright" => Some(MouseEventKindDef::ScrollRight),
        _ => None,
    }
}

fn parse_event_token(token: &str, modifiers: ModifiersDef) -> Option<EventDef> {
    if let Some(kind) = parse_mouse_event_kind(token) {
        return Some(EventDef::Mouse(MouseEventDef { kind, modifiers }));
    }

    parse_key_code(token, modifiers).map(|code| EventDef::Key(KeyEventDef { code, modifiers }))
}

#[cfg(test)]
mod tests {
    use super::*;

    mod parse_shortcut {
        use super::*;

        #[test]
        fn parses_simple_key() {
            assert_eq!(
                parse_shortcut("a").unwrap(),
                EventDef::Key(KeyEventDef {
                    code: KeyCodeDef::Char('a'),
                    modifiers: ModifiersDef::NONE,
                })
            );
        }

        #[test]
        fn parses_key_with_modifiers() {
            let mut modifiers = ModifiersDef::NONE;
            modifiers.insert(ModifiersDef::CONTROL);
            modifiers.insert(ModifiersDef::SHIFT);
            assert_eq!(
                parse_shortcut("Ctrl+Shift+A").unwrap(),
                EventDef::Key(KeyEventDef {
                    code: KeyCodeDef::Char('A'),
                    modifiers,
                })
            );
        }

        #[test]
        fn parses_mouse_event() {
            assert_eq!(
                parse_shortcut("LeftDown").unwrap(),
                EventDef::Mouse(MouseEventDef {
                    kind: MouseEventKindDef::Down(MouseButtonDef::Left),
                    modifiers: ModifiersDef::NONE,
                })
            );
        }

        #[test]
        fn parses_mouse_event_with_modifiers() {
            assert_eq!(
                parse_shortcut("Ctrl+ScrollRight").unwrap(),
                EventDef::Mouse(MouseEventDef {
                    kind: MouseEventKindDef::ScrollRight,
                    modifiers: ModifiersDef::CONTROL,
                })
            );
        }

        #[test]
        fn parses_all_supported_mouse_tokens() {
            let cases = [
                ("LeftDown", MouseEventKindDef::Down(MouseButtonDef::Left)),
                ("RightDown", MouseEventKindDef::Down(MouseButtonDef::Right)),
                (
                    "MiddleDown",
                    MouseEventKindDef::Down(MouseButtonDef::Middle),
                ),
                ("LeftUp", MouseEventKindDef::Up(MouseButtonDef::Left)),
                ("RightUp", MouseEventKindDef::Up(MouseButtonDef::Right)),
                ("MiddleUp", MouseEventKindDef::Up(MouseButtonDef::Middle)),
                ("LeftDrag", MouseEventKindDef::Drag(MouseButtonDef::Left)),
                ("RightDrag", MouseEventKindDef::Drag(MouseButtonDef::Right)),
                (
                    "MiddleDrag",
                    MouseEventKindDef::Drag(MouseButtonDef::Middle),
                ),
                ("Moved", MouseEventKindDef::Moved),
                ("ScrollDown", MouseEventKindDef::ScrollDown),
                ("ScrollUp", MouseEventKindDef::ScrollUp),
                ("ScrollLeft", MouseEventKindDef::ScrollLeft),
                ("ScrollRight", MouseEventKindDef::ScrollRight),
            ];

            for (shortcut, kind) in cases {
                assert_eq!(
                    parse_shortcut(shortcut).unwrap(),
                    EventDef::Mouse(MouseEventDef {
                        kind,
                        modifiers: ModifiersDef::NONE,
                    })
                );
            }
        }

        #[test]
        fn handles_invalid_shortcuts() {
            assert!(parse_shortcut("").is_err());
            assert!(parse_shortcut("Ctrl++A").is_err());
            assert!(parse_shortcut("A+Ctrl").is_err());
            assert!(parse_shortcut("Ctrl+A+B").is_err());
            assert!(parse_shortcut("UnknownKey").is_err());
        }

        #[test]
        fn rejects_alias_tokens() {
            let aliases = [
                "control+A",
                "option+A",
                "cmd+A",
                "command+A",
                "win+A",
                "windows+A",
                "return",
                "pgup",
                "pgdown",
                "del",
                "ins",
                "escape",
                "hyphen",
            ];

            for shortcut in aliases {
                assert!(parse_shortcut(shortcut).is_err(), "{shortcut}");
            }
        }
    }
}
