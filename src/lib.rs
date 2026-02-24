pub mod error;
pub mod event;
pub mod style;

#[cfg(all(feature = "crossterm_0_28_1", not(feature = "crossterm_0_29_0")))]
use crossterm_0_28_1 as crossterm;
#[cfg(feature = "crossterm_0_29_0")]
use crossterm_0_29_0 as crossterm;

#[cfg(any(feature = "crossterm_0_28_1", feature = "crossterm_0_29_0"))]
pub mod crossterm_config;

#[cfg(feature = "termion_4_0_6")]
use termion_4_0_6 as termion;

#[cfg(feature = "termion_4_0_6")]
pub mod termion_config;
