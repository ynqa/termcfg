#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ColorDef {
    /// A named color, e.g. "red", "blue", "green".
    Named(String),
    /// An RGB color defined by its red, green, and blue components.
    Rgb { r: u8, g: u8, b: u8 },
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ContentStyleDef {
    pub fg: Option<ColorDef>,
    pub bg: Option<ColorDef>,
    pub ul: Option<ColorDef>,
    pub attributes: Vec<String>,
}
