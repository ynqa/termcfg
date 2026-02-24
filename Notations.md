# Notation matrix for `crossterm` and `termion`

## Event Notation

### Format

e.g., `Ctrl+C`, `Ctrl+Shift+F1`, `Alt+Left`, `LeftDown`, `Ctrl+ScrollUp`

| Key | crossterm | termion | Notes |
|---|---|---|---|
| `<key>` | [`Event::Key`](https://docs.rs/crossterm/latest/crossterm/event/enum.Event.html#variant.Key) + [`KeyCode`](https://docs.rs/crossterm/latest/crossterm/event/enum.KeyCode.html) | [`Key`](https://docs.rs/termion/latest/termion/event/enum.Key.html) | e.g., `Enter`, `F1`, `A` |
| `<modifier>(+<modifier>)*+<key>` | [`KeyEvent::modifiers`](https://docs.rs/crossterm/latest/crossterm/event/struct.KeyEvent.html#structfield.modifiers) + [`KeyCode`](https://docs.rs/crossterm/latest/crossterm/event/enum.KeyCode.html) | Modifier variants in [`Key`](https://docs.rs/termion/latest/termion/event/enum.Key.html) | e.g., `Ctrl+Shift+F1`; termion supports only specific combinations |
| `<mouse>` | [`Event::Mouse`](https://docs.rs/crossterm/latest/crossterm/event/enum.Event.html#variant.Mouse) + [`MouseEventKind`](https://docs.rs/crossterm/latest/crossterm/event/enum.MouseEventKind.html) | [`MouseEvent`](https://docs.rs/termion/latest/termion/event/enum.MouseEvent.html) / [`MouseButton`](https://docs.rs/termion/latest/termion/event/enum.MouseButton.html) | e.g., `LeftDown`, `ScrollUp` |
| `<modifier>(+<modifier>)*+<mouse>` | [`MouseEvent::modifiers`](https://docs.rs/crossterm/latest/crossterm/event/struct.MouseEvent.html#structfield.modifiers) + [`MouseEventKind`](https://docs.rs/crossterm/latest/crossterm/event/enum.MouseEventKind.html) | No | Not supported in termion |

### Modifiers

| Key | crossterm | termion | Notes |
|---|---|---|---|
| `Ctrl` | [`KeyModifiers::CONTROL`](https://docs.rs/crossterm/latest/crossterm/event/struct.KeyModifiers.html#associatedconstant.CONTROL) | [`Key::Ctrl`](https://docs.rs/termion/latest/termion/event/enum.Key.html#variant.Ctrl) | |
| `Alt` | [`KeyModifiers::ALT`](https://docs.rs/crossterm/latest/crossterm/event/struct.KeyModifiers.html#associatedconstant.ALT) | [`Key::Alt`](https://docs.rs/termion/latest/termion/event/enum.Key.html#variant.Alt) | |
| `Shift` | [`KeyModifiers::SHIFT`](https://docs.rs/crossterm/latest/crossterm/event/struct.KeyModifiers.html#associatedconstant.SHIFT) | [`Key::ShiftLeft`](https://docs.rs/termion/latest/termion/event/enum.Key.html#variant.ShiftLeft) | |
| `Super` | [`KeyModifiers::SUPER`](https://docs.rs/crossterm/latest/crossterm/event/struct.KeyModifiers.html#associatedconstant.SUPER) | No | Rejected by termion conversion |
| `Hyper` | [`KeyModifiers::HYPER`](https://docs.rs/crossterm/latest/crossterm/event/struct.KeyModifiers.html#associatedconstant.HYPER) | No | Rejected by termion conversion |
| `Meta` | [`KeyModifiers::META`](https://docs.rs/crossterm/latest/crossterm/event/struct.KeyModifiers.html#associatedconstant.META) | No | Rejected by termion conversion |

Supported `key + modifier` combinations in termion:

- `Shift+Left`, `Shift+Right`, `Shift+Up`, `Shift+Down`
- `Alt+Left`, `Alt+Right`, `Alt+Up`, `Alt+Down`
- `Ctrl+Left`, `Ctrl+Right`, `Ctrl+Up`, `Ctrl+Down`
- `Ctrl+Home`, `Ctrl+End`
- `Shift+Tab` (normalized as `BackTab`)
- `Shift+<char>`, `Alt+<char>`, `Ctrl+<char>` (single character)

### Key Tokens

| Key | crossterm | termion | Notes |
|---|---|---|---|
| `Backspace` | [`KeyCode::Backspace`](https://docs.rs/crossterm/latest/crossterm/event/enum.KeyCode.html#variant.Backspace) | [`Key::Backspace`](https://docs.rs/termion/latest/termion/event/enum.Key.html#variant.Backspace) | |
| `Enter` | [`KeyCode::Enter`](https://docs.rs/crossterm/latest/crossterm/event/enum.KeyCode.html#variant.Enter) | [`Key::Char`](https://docs.rs/termion/latest/termion/event/enum.Key.html#variant.Char) | termion uses `\n` |
| `Left` | [`KeyCode::Left`](https://docs.rs/crossterm/latest/crossterm/event/enum.KeyCode.html#variant.Left) | [`Key::Left`](https://docs.rs/termion/latest/termion/event/enum.Key.html#variant.Left) | |
| `Right` | [`KeyCode::Right`](https://docs.rs/crossterm/latest/crossterm/event/enum.KeyCode.html#variant.Right) | [`Key::Right`](https://docs.rs/termion/latest/termion/event/enum.Key.html#variant.Right) | |
| `Up` | [`KeyCode::Up`](https://docs.rs/crossterm/latest/crossterm/event/enum.KeyCode.html#variant.Up) | [`Key::Up`](https://docs.rs/termion/latest/termion/event/enum.Key.html#variant.Up) | |
| `Down` | [`KeyCode::Down`](https://docs.rs/crossterm/latest/crossterm/event/enum.KeyCode.html#variant.Down) | [`Key::Down`](https://docs.rs/termion/latest/termion/event/enum.Key.html#variant.Down) | |
| `Home` | [`KeyCode::Home`](https://docs.rs/crossterm/latest/crossterm/event/enum.KeyCode.html#variant.Home) | [`Key::Home`](https://docs.rs/termion/latest/termion/event/enum.Key.html#variant.Home) | |
| `End` | [`KeyCode::End`](https://docs.rs/crossterm/latest/crossterm/event/enum.KeyCode.html#variant.End) | [`Key::End`](https://docs.rs/termion/latest/termion/event/enum.Key.html#variant.End) | |
| `PageUp` | [`KeyCode::PageUp`](https://docs.rs/crossterm/latest/crossterm/event/enum.KeyCode.html#variant.PageUp) | [`Key::PageUp`](https://docs.rs/termion/latest/termion/event/enum.Key.html#variant.PageUp) | |
| `PageDown` | [`KeyCode::PageDown`](https://docs.rs/crossterm/latest/crossterm/event/enum.KeyCode.html#variant.PageDown) | [`Key::PageDown`](https://docs.rs/termion/latest/termion/event/enum.Key.html#variant.PageDown) | |
| `Tab` | [`KeyCode::Tab`](https://docs.rs/crossterm/latest/crossterm/event/enum.KeyCode.html#variant.Tab) | [`Key::Char`](https://docs.rs/termion/latest/termion/event/enum.Key.html#variant.Char) | termion uses `\t` |
| `BackTab` | [`KeyCode::BackTab`](https://docs.rs/crossterm/latest/crossterm/event/enum.KeyCode.html#variant.BackTab) | [`Key::BackTab`](https://docs.rs/termion/latest/termion/event/enum.Key.html#variant.BackTab) | |
| `Delete` | [`KeyCode::Delete`](https://docs.rs/crossterm/latest/crossterm/event/enum.KeyCode.html#variant.Delete) | [`Key::Delete`](https://docs.rs/termion/latest/termion/event/enum.Key.html#variant.Delete) | |
| `Insert` | [`KeyCode::Insert`](https://docs.rs/crossterm/latest/crossterm/event/enum.KeyCode.html#variant.Insert) | [`Key::Insert`](https://docs.rs/termion/latest/termion/event/enum.Key.html#variant.Insert) | |
| `F1` | [`KeyCode::F`](https://docs.rs/crossterm/latest/crossterm/event/enum.KeyCode.html#variant.F) | [`Key::F`](https://docs.rs/termion/latest/termion/event/enum.Key.html#variant.F) | Same for `F2` and above |
| `A..Z` | [`KeyCode::Char`](https://docs.rs/crossterm/latest/crossterm/event/enum.KeyCode.html#variant.Char) | [`Key::Char`](https://docs.rs/termion/latest/termion/event/enum.Key.html#variant.Char) | Any single character |
| `Space` | [`KeyCode::Char`](https://docs.rs/crossterm/latest/crossterm/event/enum.KeyCode.html#variant.Char) | [`Key::Char`](https://docs.rs/termion/latest/termion/event/enum.Key.html#variant.Char) | Represents `' '` |
| `Minus` | [`KeyCode::Char`](https://docs.rs/crossterm/latest/crossterm/event/enum.KeyCode.html#variant.Char) | [`Key::Char`](https://docs.rs/termion/latest/termion/event/enum.Key.html#variant.Char) | Represents `'-'` |
| `Plus` | [`KeyCode::Char`](https://docs.rs/crossterm/latest/crossterm/event/enum.KeyCode.html#variant.Char) | [`Key::Char`](https://docs.rs/termion/latest/termion/event/enum.Key.html#variant.Char) | Represents `'+'` |
| `Null` | [`KeyCode::Null`](https://docs.rs/crossterm/latest/crossterm/event/enum.KeyCode.html#variant.Null) | [`Key::Null`](https://docs.rs/termion/latest/termion/event/enum.Key.html#variant.Null) | |
| `Esc` | [`KeyCode::Esc`](https://docs.rs/crossterm/latest/crossterm/event/enum.KeyCode.html#variant.Esc) | [`Key::Esc`](https://docs.rs/termion/latest/termion/event/enum.Key.html#variant.Esc) | |
| `CapsLock` | [`KeyCode::CapsLock`](https://docs.rs/crossterm/latest/crossterm/event/enum.KeyCode.html#variant.CapsLock) | No | Not supported in termion |
| `ScrollLock` | [`KeyCode::ScrollLock`](https://docs.rs/crossterm/latest/crossterm/event/enum.KeyCode.html#variant.ScrollLock) | No | Not supported in termion |
| `NumLock` | [`KeyCode::NumLock`](https://docs.rs/crossterm/latest/crossterm/event/enum.KeyCode.html#variant.NumLock) | No | Not supported in termion |
| `PrintScreen` | [`KeyCode::PrintScreen`](https://docs.rs/crossterm/latest/crossterm/event/enum.KeyCode.html#variant.PrintScreen) | No | Not supported in termion |
| `Pause` | [`KeyCode::Pause`](https://docs.rs/crossterm/latest/crossterm/event/enum.KeyCode.html#variant.Pause) | No | Not supported in termion |
| `Menu` | [`KeyCode::Menu`](https://docs.rs/crossterm/latest/crossterm/event/enum.KeyCode.html#variant.Menu) | No | Not supported in termion |
| `KeypadBegin` | [`KeyCode::KeypadBegin`](https://docs.rs/crossterm/latest/crossterm/event/enum.KeyCode.html#variant.KeypadBegin) | No | Not supported in termion |

### Mouse Tokens

| Key | crossterm | termion | Notes |
|---|---|---|---|
| `LeftDown` | [`MouseEventKind::Down`](https://docs.rs/crossterm/latest/crossterm/event/enum.MouseEventKind.html#variant.Down) | [`MouseEvent::Press`](https://docs.rs/termion/latest/termion/event/enum.MouseEvent.html#variant.Press) | |
| `RightDown` | [`MouseEventKind::Down`](https://docs.rs/crossterm/latest/crossterm/event/enum.MouseEventKind.html#variant.Down) | [`MouseEvent::Press`](https://docs.rs/termion/latest/termion/event/enum.MouseEvent.html#variant.Press) | |
| `MiddleDown` | [`MouseEventKind::Down`](https://docs.rs/crossterm/latest/crossterm/event/enum.MouseEventKind.html#variant.Down) | [`MouseEvent::Press`](https://docs.rs/termion/latest/termion/event/enum.MouseEvent.html#variant.Press) | |
| `LeftUp` | [`MouseEventKind::Up`](https://docs.rs/crossterm/latest/crossterm/event/enum.MouseEventKind.html#variant.Up) | No | Not supported in termion |
| `RightUp` | [`MouseEventKind::Up`](https://docs.rs/crossterm/latest/crossterm/event/enum.MouseEventKind.html#variant.Up) | No | Not supported in termion |
| `MiddleUp` | [`MouseEventKind::Up`](https://docs.rs/crossterm/latest/crossterm/event/enum.MouseEventKind.html#variant.Up) | No | Not supported in termion |
| `LeftDrag` | [`MouseEventKind::Drag`](https://docs.rs/crossterm/latest/crossterm/event/enum.MouseEventKind.html#variant.Drag) | No | Not supported in termion |
| `RightDrag` | [`MouseEventKind::Drag`](https://docs.rs/crossterm/latest/crossterm/event/enum.MouseEventKind.html#variant.Drag) | No | Not supported in termion |
| `MiddleDrag` | [`MouseEventKind::Drag`](https://docs.rs/crossterm/latest/crossterm/event/enum.MouseEventKind.html#variant.Drag) | No | Not supported in termion |
| `Moved` | [`MouseEventKind::Moved`](https://docs.rs/crossterm/latest/crossterm/event/enum.MouseEventKind.html#variant.Moved) | No | Not supported in termion |
| `ScrollDown` | [`MouseEventKind::ScrollDown`](https://docs.rs/crossterm/latest/crossterm/event/enum.MouseEventKind.html#variant.ScrollDown) | [`MouseButton::WheelDown`](https://docs.rs/termion/latest/termion/event/enum.MouseButton.html#variant.WheelDown) | represented as `Press(WheelDown, ..)` in termion |
| `ScrollUp` | [`MouseEventKind::ScrollUp`](https://docs.rs/crossterm/latest/crossterm/event/enum.MouseEventKind.html#variant.ScrollUp) | [`MouseButton::WheelUp`](https://docs.rs/termion/latest/termion/event/enum.MouseButton.html#variant.WheelUp) | represented as `Press(WheelUp, ..)` in termion |
| `ScrollLeft` | [`MouseEventKind::ScrollLeft`](https://docs.rs/crossterm/latest/crossterm/event/enum.MouseEventKind.html#variant.ScrollLeft) | [`MouseButton::WheelLeft`](https://docs.rs/termion/latest/termion/event/enum.MouseButton.html#variant.WheelLeft) | represented as `Press(WheelLeft, ..)` in termion |
| `ScrollRight` | [`MouseEventKind::ScrollRight`](https://docs.rs/crossterm/latest/crossterm/event/enum.MouseEventKind.html#variant.ScrollRight) | [`MouseButton::WheelRight`](https://docs.rs/termion/latest/termion/event/enum.MouseButton.html#variant.WheelRight) | represented as `Press(WheelRight, ..)` in termion |

#### Additional backend constraints

- Mouse inputs with modifiers like `Ctrl+LeftDown` are handled by crossterm via [`MouseEvent::modifiers`](https://docs.rs/crossterm/latest/crossterm/event/struct.MouseEvent.html#structfield.modifiers), but are not supported in termion.
- crossterm does not serialize [`KeyCode::Media(_)`](https://docs.rs/crossterm/latest/crossterm/event/enum.KeyCode.html#variant.Media) / [`KeyCode::Modifier(_)`](https://docs.rs/crossterm/latest/crossterm/event/enum.KeyCode.html#variant.Modifier) into notation.
- crossterm notation supports only [`Event::Key`](https://docs.rs/crossterm/latest/crossterm/event/enum.Event.html#variant.Key) / [`Event::Mouse`](https://docs.rs/crossterm/latest/crossterm/event/enum.Event.html#variant.Mouse).

## Style Notation

### Format

e.g., `fg=red,bg=#00FF00,ul=blue,attr=bold|italic`

| Key | crossterm | termion | Notes |
|---|---|---|---|
| `fg=<color>` | [`ContentStyle::foreground_color`](https://docs.rs/crossterm/latest/crossterm/style/struct.ContentStyle.html#structfield.foreground_color) | [`color::Fg`](https://docs.rs/termion/latest/termion/color/struct.Fg.html) | |
| `bg=<color>` | [`ContentStyle::background_color`](https://docs.rs/crossterm/latest/crossterm/style/struct.ContentStyle.html#structfield.background_color) | [`color::Bg`](https://docs.rs/termion/latest/termion/color/struct.Bg.html) | |
| `ul=<color>` | [`ContentStyle::underline_color`](https://docs.rs/crossterm/latest/crossterm/style/struct.ContentStyle.html#structfield.underline_color) | No | Underline color is not supported in termion |
| `attr=<token\|token...>` | [`Attribute`](https://docs.rs/crossterm/latest/crossterm/style/enum.Attribute.html) | [`style` module](https://docs.rs/termion/latest/termion/style/index.html) | |

### Color Tokens

| Key | crossterm | termion | Notes |
|---|---|---|---|
| `reset`, `black`, `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, `white` | [`Color` enum](https://docs.rs/crossterm/latest/crossterm/style/enum.Color.html) | [`color` module](https://docs.rs/termion/latest/termion/color/index.html) | |
| `darkgrey`, `darkred`, `darkgreen`, `darkyellow`, `darkblue`, `darkmagenta`, `darkcyan`, `grey` | [`Color` enum](https://docs.rs/crossterm/latest/crossterm/style/enum.Color.html) | No | Not supported in termion |
| `lightblack`, `lightred`, `lightgreen`, `lightyellow`, `lightblue`, `lightmagenta`, `lightcyan`, `lightwhite` | No | [`color` module](https://docs.rs/termion/latest/termion/color/index.html) | Not supported in crossterm |
| `#RRGGBB` | [`Color::Rgb`](https://docs.rs/crossterm/latest/crossterm/style/enum.Color.html#variant.Rgb) | [`color::Rgb`](https://docs.rs/termion/latest/termion/color/struct.Rgb.html) | |

#### Unsupported tokens

- `0..255` (e.g., `200`): [`Color::AnsiValue`](https://docs.rs/crossterm/latest/crossterm/style/enum.Color.html#variant.AnsiValue) / [`color::AnsiValue`](https://docs.rs/termion/latest/termion/color/struct.AnsiValue.html) exist in backend types, but are currently out of this crate's notation input/output contract. Support for ANSI 256-color tokens may be added in a future release.

### Attribute Tokens

| Key | crossterm | termion | Notes |
|---|---|---|---|
| `reset` | [`Attribute::Reset`](https://docs.rs/crossterm/latest/crossterm/style/enum.Attribute.html#variant.Reset) | [`style::Reset`](https://docs.rs/termion/latest/termion/style/struct.Reset.html) | |
| `bold` | [`Attribute::Bold`](https://docs.rs/crossterm/latest/crossterm/style/enum.Attribute.html#variant.Bold) | [`style::Bold`](https://docs.rs/termion/latest/termion/style/struct.Bold.html) | |
| `italic` | [`Attribute::Italic`](https://docs.rs/crossterm/latest/crossterm/style/enum.Attribute.html#variant.Italic) | [`style::Italic`](https://docs.rs/termion/latest/termion/style/struct.Italic.html) | |
| `crossedout` | [`Attribute::CrossedOut`](https://docs.rs/crossterm/latest/crossterm/style/enum.Attribute.html#variant.CrossedOut) | [`style::CrossedOut`](https://docs.rs/termion/latest/termion/style/struct.CrossedOut.html) | |
| `nobold` | [`Attribute::NoBold`](https://docs.rs/crossterm/latest/crossterm/style/enum.Attribute.html#variant.NoBold) | [`style::NoBold`](https://docs.rs/termion/latest/termion/style/struct.NoBold.html) | |
| `noitalic` | [`Attribute::NoItalic`](https://docs.rs/crossterm/latest/crossterm/style/enum.Attribute.html#variant.NoItalic) | [`style::NoItalic`](https://docs.rs/termion/latest/termion/style/struct.NoItalic.html) | |
| `nounderline` | [`Attribute::NoUnderline`](https://docs.rs/crossterm/latest/crossterm/style/enum.Attribute.html#variant.NoUnderline) | [`style::NoUnderline`](https://docs.rs/termion/latest/termion/style/struct.NoUnderline.html) | |
| `noblink` | [`Attribute::NoBlink`](https://docs.rs/crossterm/latest/crossterm/style/enum.Attribute.html#variant.NoBlink) | [`style::NoBlink`](https://docs.rs/termion/latest/termion/style/struct.NoBlink.html) | |
| `framed` | [`Attribute::Framed`](https://docs.rs/crossterm/latest/crossterm/style/enum.Attribute.html#variant.Framed) | [`style::Framed`](https://docs.rs/termion/latest/termion/style/struct.Framed.html) | |

#### For crossterm

| Key | crossterm | Notes |
|---|---|---|
| `underlined` | [`Attribute::Underlined`](https://docs.rs/crossterm/latest/crossterm/style/enum.Attribute.html#variant.Underlined) | |
| `dim` | [`Attribute::Dim`](https://docs.rs/crossterm/latest/crossterm/style/enum.Attribute.html#variant.Dim) | |
| `slowblink` | [`Attribute::SlowBlink`](https://docs.rs/crossterm/latest/crossterm/style/enum.Attribute.html#variant.SlowBlink) | |
| `rapidblink` | [`Attribute::RapidBlink`](https://docs.rs/crossterm/latest/crossterm/style/enum.Attribute.html#variant.RapidBlink) | |
| `reverse` | [`Attribute::Reverse`](https://docs.rs/crossterm/latest/crossterm/style/enum.Attribute.html#variant.Reverse) | |
| `noreverse` | [`Attribute::NoReverse`](https://docs.rs/crossterm/latest/crossterm/style/enum.Attribute.html#variant.NoReverse) | |
| `notcrossedout` | [`Attribute::NotCrossedOut`](https://docs.rs/crossterm/latest/crossterm/style/enum.Attribute.html#variant.NotCrossedOut) | |
| `normalintensity` | [`Attribute::NormalIntensity`](https://docs.rs/crossterm/latest/crossterm/style/enum.Attribute.html#variant.NormalIntensity) | |
| `doubleunderlined` | [`Attribute::DoubleUnderlined`](https://docs.rs/crossterm/latest/crossterm/style/enum.Attribute.html#variant.DoubleUnderlined) | |
| `undercurled` | [`Attribute::Undercurled`](https://docs.rs/crossterm/latest/crossterm/style/enum.Attribute.html#variant.Undercurled) | |
| `underdotted` | [`Attribute::Underdotted`](https://docs.rs/crossterm/latest/crossterm/style/enum.Attribute.html#variant.Underdotted) | |
| `underdashed` | [`Attribute::Underdashed`](https://docs.rs/crossterm/latest/crossterm/style/enum.Attribute.html#variant.Underdashed) | |
| `hidden` | [`Attribute::Hidden`](https://docs.rs/crossterm/latest/crossterm/style/enum.Attribute.html#variant.Hidden) | |
| `fraktur` | [`Attribute::Fraktur`](https://docs.rs/crossterm/latest/crossterm/style/enum.Attribute.html#variant.Fraktur) | |
| `nohidden` | [`Attribute::NoHidden`](https://docs.rs/crossterm/latest/crossterm/style/enum.Attribute.html#variant.NoHidden) | |
| `encircled` | [`Attribute::Encircled`](https://docs.rs/crossterm/latest/crossterm/style/enum.Attribute.html#variant.Encircled) | |
| `overlined` | [`Attribute::OverLined`](https://docs.rs/crossterm/latest/crossterm/style/enum.Attribute.html#variant.OverLined) | |
| `notframedorencircled` | [`Attribute::NotFramedOrEncircled`](https://docs.rs/crossterm/latest/crossterm/style/enum.Attribute.html#variant.NotFramedOrEncircled) | |
| `notoverlined` | [`Attribute::NotOverLined`](https://docs.rs/crossterm/latest/crossterm/style/enum.Attribute.html#variant.NotOverLined) | |

#### For termion

| Key | termion | Notes |
|---|---|---|
| `underline` | [`style::Underline`](https://docs.rs/termion/latest/termion/style/struct.Underline.html) | |
| `faint` | [`style::Faint`](https://docs.rs/termion/latest/termion/style/struct.Faint.html) | |
| `blink` | [`style::Blink`](https://docs.rs/termion/latest/termion/style/struct.Blink.html) | |
| `invert` | [`style::Invert`](https://docs.rs/termion/latest/termion/style/struct.Invert.html) | |
| `noinvert` | [`style::NoInvert`](https://docs.rs/termion/latest/termion/style/struct.NoInvert.html) | |
| `nocrossedout` | [`style::NoCrossedOut`](https://docs.rs/termion/latest/termion/style/struct.NoCrossedOut.html) | |
| `nofaint` | [`style::NoFaint`](https://docs.rs/termion/latest/termion/style/struct.NoFaint.html) | |
