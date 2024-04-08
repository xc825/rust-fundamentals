//! Colorized output utilities for the terminal using ANSI escape codes.
//! # Examples:
//! ```
//! use my_library::colors::*;
//! println!("{}{}{}", red("Red"), green("Green"), blue("Blue"));
//! ```

/// Returns a string with the ANSI escape code for red.
/// # Examples:
/// ```
/// use my_library::colors::*;
/// println!("{}", red("Red"));
/// ```
pub fn red(s: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", s)
}


/// Returns a string with the ANSI escape code for green.
/// # Examples:
/// ```
/// use my_library::colors::*;
/// println!("{}", green("Green"));
/// ```
pub fn green(s: &str) -> String {
    format!("\x1b[32m{}\x1b[0m", s)
}

/// Returns a string with the ANSI escape code for blue.
/// # Examples:
/// ```
/// use my_library::colors::*;
/// println!("{}", blue("Blue"));
/// ```
pub fn blue(s: &str) -> String {
    format!("\x1b[34m{}\x1b[0m", s)
}

/// Returns a string with the ANSI escape code for bold.
/// # Examples:
/// ```
/// use my_library::colors::*;
/// println!("{}", bold("Bold"));
/// ```
pub fn bold(s: &str) -> String {
    format!("\x1b[1m{}\x1b[0m", s)
}

/// Returns a string with the ANSI escape code for reset.
/// # Examples:
/// ```
/// use my_library::colors::*;
/// println!("{}", reset("Reset"));
/// ```
pub fn reset(s: &str) -> String {
    format!("\x1b[0m{}\x1b[0m", s)
}

/// An enum representing the color of the text.
pub enum Color{
    Red,
    Green,
    Blue,
    Bold,
}

/// A struct that contains a string and a color.
pub struct ColorString {
    pub color: Color,
    pub string: String,
    pub colorized: String
}

/// Implementation of the ColorString struct.
impl ColorString {
    // create a method that will use the string and color fields to create a colorized string and assign it to the colorized field
    /// Create a new ColorString with the given string and color.
    /// # Examples:
    /// ```
    /// use my_library::colors::*;
    ///     let mut cs = ColorString::new("Hello, world!".to_string(), Color::Red);
    ///    cs.paint();
    ///   println!("{}", cs.colorized);
    /// ```
    pub fn new(string: String, color: Color) -> Self {
        Self {
            color,
            string,
            colorized: "".to_string(),
        }
    }
    pub fn paint(&mut self) {
        match self.color {
            Color::Red => self.colorized = red(&self.string),
            Color::Green => self.colorized = green(&self.string),
            Color::Blue => self.colorized = blue(&self.string),
            Color::Bold => self.colorized = bold(&self.string),
        };
    }

    pub fn bold(&mut self) {
        self.colorized = bold(&self.string);
    }

    pub fn reset(&mut self) {
        self.colorized = reset(&self.string);
    }
}
