/// Defines the `EventDef` enum and related structures for representing key and mouse events in a structured way.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventDef {
    Key(KeyEventDef),
    Mouse(MouseEventDef),
}

/// Represents a key event, including the key code and any modifiers (e.g., Ctrl, Alt).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyEventDef {
    pub code: KeyCodeDef,
    pub modifiers: ModifiersDef,
}

/// Represents a mouse event, including the kind of mouse event (e.g., button down, scroll) and any modifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MouseEventDef {
    pub kind: MouseEventKindDef,
    pub modifiers: ModifiersDef,
}

/// Represents modifier keys (e.g., Shift, Ctrl) as a bitmask.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ModifiersDef(u8);

impl ModifiersDef {
    pub const NONE: Self = Self(0);
    pub const SHIFT: Self = Self(1 << 0);
    pub const CONTROL: Self = Self(1 << 1);
    pub const ALT: Self = Self(1 << 2);
    pub const SUPER: Self = Self(1 << 3);
    pub const HYPER: Self = Self(1 << 4);
    pub const META: Self = Self(1 << 5);
    pub fn insert(&mut self, other: Self) {
        self.0 |= other.0;
    }

    pub fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}

/// Represents the key code of a key event, such as a character, function key, or special key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyCodeDef {
    Backspace,
    Enter,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    Tab,
    BackTab,
    Delete,
    Insert,
    F(u8),
    Char(char),
    Null,
    Esc,
    CapsLock,
    ScrollLock,
    NumLock,
    PrintScreen,
    Pause,
    Menu,
    KeypadBegin,
}

/// Represents a mouse button (left, right, middle).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButtonDef {
    Left,
    Right,
    Middle,
}

/// Represents the kind of mouse event, such as button down, button up, drag, move, or scroll.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseEventKindDef {
    Down(MouseButtonDef),
    Up(MouseButtonDef),
    Drag(MouseButtonDef),
    Moved,
    ScrollDown,
    ScrollUp,
    ScrollLeft,
    ScrollRight,
}
