use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ShortcutError {
    #[error("shortcut is empty")]
    EmptyShortcut,

    #[error("invalid shortcut: {shortcut}")]
    InvalidShortcut { shortcut: String },

    #[error("modifier must come before key code: {shortcut}")]
    ModifierAfterKeyCode { shortcut: String },

    #[error("multiple key codes in shortcut: {shortcut}")]
    MultipleKeyCodes { shortcut: String },

    #[error("unsupported key token: {token}")]
    UnsupportedKeyToken { token: String },

    #[error("missing key code in shortcut: {shortcut}")]
    MissingKeyCode { shortcut: String },

    #[error("event is not representable as a shortcut")]
    InvalidEventForShortcut,

    #[error("key code is not supported for shortcut serialization")]
    UnsupportedKeyCodeForShortcut,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum StyleError {
    #[error("invalid style entry: {entry}")]
    InvalidStyleEntry { entry: String },

    #[error("unknown style key: {key}")]
    UnknownStyleKey { key: String },

    #[error("duplicate style key: {key}")]
    DuplicateStyleKey { key: String },

    #[error("empty style value for key: {key}")]
    EmptyStyleValue { key: String },

    #[error("unsupported color token: {token}")]
    UnsupportedColorToken { token: String },

    #[error("unsupported attribute token: {token}")]
    UnsupportedAttributeToken { token: String },
}
