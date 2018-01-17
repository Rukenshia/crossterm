//! With this module you can perform actions that are color related.
//! Like styling the font, foreground color and background color.

use std::fmt;
use std::convert::From;
use std::str::FromStr;

use Construct;
use crossterm_style::{ObjectStyle, StyledObject};
use super::base_color::ITerminalColor;

#[cfg(unix)]
use super::ANSIColor;
#[cfg(windows)]
use super::WinApiColor;

/// Colors that are available for coloring the termainal font.
#[derive(Debug, Copy, Clone)]
pub enum Color {
    Black,

    Red,
    DarkRed,

    Green,
    DarkGreen,

    Yellow,
    DarkYellow,

    Blue,
    DarkBlue,

    Magenta,
    DarkMagenta,

    Cyan,
    DarkCyan,

    Grey,
    White,
}

/// Color types that can be used to determine if the Color enum is an Fore- or Background Color
#[derive(Debug, Copy, Clone)]
pub enum ColorType {
    Background,
    Foreground,
}

impl<'a> From<&'a str> for Color {
    fn from(src: &str) -> Self {
        src.parse().unwrap_or(Color::White)
    }
}

impl From<String> for Color {
    fn from(src: String) -> Self {
        src.parse().unwrap_or(Color::White)
    }
}

impl FromStr for Color {
    type Err = ();

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        let src = src.to_lowercase();

        match src.as_ref() {
            "black" => Ok(Color::Black),
            "red" => Ok(Color::Red),
            "dark_red" => Ok(Color::DarkRed),
            "green" => Ok(Color::Green),
            "dark_green" => Ok(Color::DarkGreen),
            "yellow" => Ok(Color::Yellow),
            "dark_yellow" => Ok(Color::DarkYellow),
            "blue" => Ok(Color::Blue),
            "dark_blue" => Ok(Color::DarkBlue),
            "magenta" => Ok(Color::Magenta),
            "dark_magenta" => Ok(Color::DarkMagenta),
            "cyan" => Ok(Color::Cyan),
            "dark_cyan" => Ok(Color::DarkCyan),
            "grey" => Ok(Color::Grey),
            "white" => Ok(Color::White),
            _ => Ok(Color::White),
        }
    }
}

/// Struct that stores an specific platform implementation for color related actions. 
pub struct TerminalColor {
    terminal_color: Option<Box<ITerminalColor>>,
}

impl TerminalColor {
    /// Instantiate an color implementation whereon color related actions can be performed.
    pub fn init(&mut self) {
        if let None = self.terminal_color {
            self.terminal_color = get_color_options();
        }
    }

    /// Set the forground color to the given color.
    ///
    /// #Example
    ///
    /// ```rust
    /// extern crate crossterm;
    ///
    /// use self::crossterm::crossterm_style::{ get, Color};
    ///
    /// // Get colored terminal instance
    /// let mut colored_terminal = get();
    /// 
    /// // Set foreground color of the font
    /// colored_terminal.set_fg(Color::Red);
    /// // crossterm provides to set the background from &str or String
    /// colored_terminal.set_fg(Color::from("Red"));
    ///
    /// ```
    pub fn set_fg(&mut self, color: Color) {
        &self.init();
        if let Some(ref terminal_color) = self.terminal_color {
            terminal_color.set_fg(color);
        }
    }

    /// Set the background color to the given color.
    ///
    /// #Example
    ///
    /// ```rust
    ///
    /// extern crate crossterm;
    ///
    /// use self::crossterm::crossterm_style::{ get, Color};
    ///
    /// // Get colored terminal instance
    /// let mut colored_terminal = get();
    /// 
    /// // Set background color of the font
    /// colored_terminal.set_bg(Color::Red);
    /// // crossterm provides to set the background from &str or String
    /// colored_terminal.set_bg(Color::from("Red"));
    ///
    /// ```
    pub fn set_bg(&mut self, color: Color) {
        &self.init();
        if let Some(ref terminal_color) = self.terminal_color {
            terminal_color.set_bg(color);
        }
    }

    /// Reset the terminal colors and attributes to default.
    /// # Example
    ///
    /// ```rust
    /// extern crate crossterm;
    ///
    /// use self::crossterm::crossterm_style::get;
    ///
    /// // Get colored terminal instance
    /// let mut colored_terminal = get();
    /// 
    /// colored_terminal.reset();
    ///
    /// ```
    pub fn reset(&mut self) {
        &self.init();
        if let Some(ref terminal_color) = self.terminal_color {
            terminal_color.reset();
        }
    }
}

/// Get an concrete ITerminalColor implementation based on the current operating system.
fn get_color_options() -> Option<Box<ITerminalColor>> {
    #[cfg(unix)]
    return Some(ANSIColor::new());
    #[cfg(windows)]
    return Some(WinApiColor::new());
}

/// Get an TerminalColor implementation whereon color related actions can be performed.
///
/// # Example
///
/// ```rust
/// extern crate crossterm;
///
/// use self::crossterm::crossterm_style::{get, Color};
/// 
/// // Get colored terminal instance
/// let mut colored_terminal = get();
///
/// // preform some actions on the colored terminal
/// colored_terminal.set_fg(Color::Red);
/// colored_terminal.set_bg(Color::Blue);
/// colored_terminal.reset();
/// ```
pub fn get() -> Box<TerminalColor> {
    Box::from(TerminalColor {
        terminal_color: get_color_options(),
    })
}

/// Wraps an displayable object so it can be formatted with colors and attributes.
///
/// Check `/examples/color` in the libary for more spesific examples.
/// 
/// #Example
///
/// ```rust
/// extern crate crossterm;
///
/// use self::crossterm::crossterm_style::{paint,Color};
///
/// fn main()
/// {
///     // Create an styledobject object from the text 'Unstyled font' 
///     // Currently it has the default foregroundcolor and backgroundcolor.
///     println!("{}",paint("Unstyled font"));
///
///     // Create an displayable object from the text 'Colored font', 
///     // Paint this with the `Red` foreground color and `Blue` backgroundcolor.
///     // Print the result.
///     let styledobject = paint("Colored font").with(Color::Red).on(Color::Blue);
///     println!("{}", styledobject);
///    
///     // Or all in one line
///     println!("{}", paint("Colored font").with(Color::Red).on(Color::Blue));
/// }
/// ```
pub fn paint<D>(val: D) -> StyledObject<D>
where
    D: fmt::Display,
{
    ObjectStyle::new().apply_to(val)
}