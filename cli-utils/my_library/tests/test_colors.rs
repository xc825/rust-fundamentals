extern crate my_library;

use my_library::colors::{ColorString, Color};

#[test]
fn test_red_coloring() {
    let mut color_string = ColorString {
        color: Color::Red,
        string: "Red".to_string(),
        colorized: "".to_string(),
    };
    color_string.paint();
    assert_eq!(color_string.colorized, "\x1b[31mRed\x1b[0m");
}

#[test]
fn test_green_coloring() {
    let mut color_string = ColorString {
        color: Color::Green,
        string: "Green".to_string(),
        colorized: "".to_string(),
    };
    color_string.paint();
    assert_eq!(color_string.colorized, "\x1b[32mGreen\x1b[0m");
}

#[test]
fn test_blue_coloring() {
    let mut color_string = ColorString {
        color: Color::Blue,
        string: "Blue".to_string(),
        colorized: "".to_string(),
    };
    color_string.paint();
    assert_eq!(color_string.colorized, "\x1b[34mBlue\x1b[0m");
}

#[test]
fn test_reset_coloring() {
    let mut color_string = ColorString {
        color: Color::Blue,
        string: "Reset".to_string(),
        colorized: "".to_string(),
    };
    color_string.paint();
    assert_eq!(color_string.colorized, "\x1b[34mReset\x1b[0m");
    color_string.reset();
    assert_eq!(color_string.colorized, "\x1b[0mReset\x1b[0m");
    color_string.bold();
    assert_eq!(color_string.colorized, "\x1b[1mReset\x1b[0m");
}
