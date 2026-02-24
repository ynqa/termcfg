use super::event_def::{
    EventDef, KeyCodeDef, KeyEventDef, ModifiersDef, MouseButtonDef, MouseEventDef,
    MouseEventKindDef,
};

/// Convert an `EventDef` into a shortcut string representation.
pub fn event_to_shortcut(event: EventDef) -> String {
    match event {
        EventDef::Key(key_event_def) => key_event_to_shortcut(key_event_def),
        EventDef::Mouse(mouse_event_def) => mouse_event_to_shortcut(mouse_event_def),
    }
}

fn key_event_to_shortcut(key_event_def: KeyEventDef) -> String {
    let mut parts: Vec<String> = Vec::new();
    push_modifiers(&mut parts, key_event_def.modifiers);
    parts.push(key_code_to_token(key_event_def.code));
    parts.join("+")
}

fn mouse_event_to_shortcut(mouse_event_def: MouseEventDef) -> String {
    let mut parts: Vec<String> = Vec::new();
    push_modifiers(&mut parts, mouse_event_def.modifiers);
    parts.push(mouse_event_kind_to_token(mouse_event_def.kind));
    parts.join("+")
}

fn push_modifiers(parts: &mut Vec<String>, modifiers: ModifiersDef) {
    if modifiers.contains(ModifiersDef::CONTROL) {
        parts.push("Ctrl".to_string());
    }
    if modifiers.contains(ModifiersDef::ALT) {
        parts.push("Alt".to_string());
    }
    if modifiers.contains(ModifiersDef::SHIFT) {
        parts.push("Shift".to_string());
    }
    if modifiers.contains(ModifiersDef::SUPER) {
        parts.push("Super".to_string());
    }
    if modifiers.contains(ModifiersDef::HYPER) {
        parts.push("Hyper".to_string());
    }
    if modifiers.contains(ModifiersDef::META) {
        parts.push("Meta".to_string());
    }
}

fn key_code_to_token(code: KeyCodeDef) -> String {
    match code {
        KeyCodeDef::Backspace => "Backspace".to_string(),
        KeyCodeDef::Enter => "Enter".to_string(),
        KeyCodeDef::Left => "Left".to_string(),
        KeyCodeDef::Right => "Right".to_string(),
        KeyCodeDef::Up => "Up".to_string(),
        KeyCodeDef::Down => "Down".to_string(),
        KeyCodeDef::Home => "Home".to_string(),
        KeyCodeDef::End => "End".to_string(),
        KeyCodeDef::PageUp => "PageUp".to_string(),
        KeyCodeDef::PageDown => "PageDown".to_string(),
        KeyCodeDef::Tab => "Tab".to_string(),
        KeyCodeDef::BackTab => "BackTab".to_string(),
        KeyCodeDef::Delete => "Delete".to_string(),
        KeyCodeDef::Insert => "Insert".to_string(),
        KeyCodeDef::F(num) => format!("F{num}"),
        KeyCodeDef::Char(' ') => "Space".to_string(),
        KeyCodeDef::Char(ch) => ch.to_ascii_uppercase().to_string(),
        KeyCodeDef::Null => "Null".to_string(),
        KeyCodeDef::Esc => "Esc".to_string(),
        KeyCodeDef::CapsLock => "CapsLock".to_string(),
        KeyCodeDef::ScrollLock => "ScrollLock".to_string(),
        KeyCodeDef::NumLock => "NumLock".to_string(),
        KeyCodeDef::PrintScreen => "PrintScreen".to_string(),
        KeyCodeDef::Pause => "Pause".to_string(),
        KeyCodeDef::Menu => "Menu".to_string(),
        KeyCodeDef::KeypadBegin => "KeypadBegin".to_string(),
    }
}

fn mouse_event_kind_to_token(kind: MouseEventKindDef) -> String {
    match kind {
        MouseEventKindDef::Down(MouseButtonDef::Left) => "LeftDown".to_string(),
        MouseEventKindDef::Down(MouseButtonDef::Right) => "RightDown".to_string(),
        MouseEventKindDef::Down(MouseButtonDef::Middle) => "MiddleDown".to_string(),
        MouseEventKindDef::Up(MouseButtonDef::Left) => "LeftUp".to_string(),
        MouseEventKindDef::Up(MouseButtonDef::Right) => "RightUp".to_string(),
        MouseEventKindDef::Up(MouseButtonDef::Middle) => "MiddleUp".to_string(),
        MouseEventKindDef::Drag(MouseButtonDef::Left) => "LeftDrag".to_string(),
        MouseEventKindDef::Drag(MouseButtonDef::Right) => "RightDrag".to_string(),
        MouseEventKindDef::Drag(MouseButtonDef::Middle) => "MiddleDrag".to_string(),
        MouseEventKindDef::Moved => "Moved".to_string(),
        MouseEventKindDef::ScrollDown => "ScrollDown".to_string(),
        MouseEventKindDef::ScrollUp => "ScrollUp".to_string(),
        MouseEventKindDef::ScrollLeft => "ScrollLeft".to_string(),
        MouseEventKindDef::ScrollRight => "ScrollRight".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod event_to_shortcut {
        use super::*;

        #[test]
        fn simple_key() {
            let event = EventDef::Key(KeyEventDef {
                code: KeyCodeDef::Char('a'),
                modifiers: ModifiersDef::NONE,
            });
            assert_eq!(event_to_shortcut(event), "A");
        }

        #[test]
        fn with_modifiers() {
            let mut modifiers = ModifiersDef::NONE;
            modifiers.insert(ModifiersDef::CONTROL);
            modifiers.insert(ModifiersDef::ALT);
            let event = EventDef::Key(KeyEventDef {
                code: KeyCodeDef::Char('c'),
                modifiers,
            });
            assert_eq!(event_to_shortcut(event), "Ctrl+Alt+C");
        }

        #[test]
        fn mouse_event() {
            let event = EventDef::Mouse(MouseEventDef {
                kind: MouseEventKindDef::Down(MouseButtonDef::Left),
                modifiers: ModifiersDef::NONE,
            });
            assert_eq!(event_to_shortcut(event), "LeftDown");
        }

        #[test]
        fn mouse_event_with_modifiers() {
            let mut modifiers = ModifiersDef::NONE;
            modifiers.insert(ModifiersDef::SHIFT);
            modifiers.insert(ModifiersDef::ALT);
            let event = EventDef::Mouse(MouseEventDef {
                kind: MouseEventKindDef::ScrollRight,
                modifiers,
            });
            assert_eq!(event_to_shortcut(event), "Alt+Shift+ScrollRight");
        }

        #[test]
        fn supports_all_mouse_tokens() {
            let cases = [
                (MouseEventKindDef::Down(MouseButtonDef::Left), "LeftDown"),
                (MouseEventKindDef::Down(MouseButtonDef::Right), "RightDown"),
                (
                    MouseEventKindDef::Down(MouseButtonDef::Middle),
                    "MiddleDown",
                ),
                (MouseEventKindDef::Up(MouseButtonDef::Left), "LeftUp"),
                (MouseEventKindDef::Up(MouseButtonDef::Right), "RightUp"),
                (MouseEventKindDef::Up(MouseButtonDef::Middle), "MiddleUp"),
                (MouseEventKindDef::Drag(MouseButtonDef::Left), "LeftDrag"),
                (MouseEventKindDef::Drag(MouseButtonDef::Right), "RightDrag"),
                (
                    MouseEventKindDef::Drag(MouseButtonDef::Middle),
                    "MiddleDrag",
                ),
                (MouseEventKindDef::Moved, "Moved"),
                (MouseEventKindDef::ScrollDown, "ScrollDown"),
                (MouseEventKindDef::ScrollUp, "ScrollUp"),
                (MouseEventKindDef::ScrollLeft, "ScrollLeft"),
                (MouseEventKindDef::ScrollRight, "ScrollRight"),
            ];

            for (kind, token) in cases {
                let event = EventDef::Mouse(MouseEventDef {
                    kind,
                    modifiers: ModifiersDef::NONE,
                });
                assert_eq!(event_to_shortcut(event), token);
            }
        }
    }
}
