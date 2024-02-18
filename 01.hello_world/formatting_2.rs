/*
* Activity
* Add an implementation of the fmt::Display trait for the Color struct above so that the output displays as:
*
* RGB (128, 255, 90) 0x80FF5A
* RGB (0, 3, 254) 0x0003FE
* RGB (0, 0, 0) 0x000000
* Three hints if you get stuck:
*
* The formula for calculating a color in the RGB color space is: RGB = (R*65536)+(G*256)+B ,
* (when R is RED, G is GREEN and B is BLUE). For more see RGB color format & calculation.
* You may need to list each color more than once.
* You can pad with zeros to a width of 2 with :0>2.
*/

use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    // `f` is a buffer, and this method must write the formatted string into it.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let hex = (self.red as u32 * 65536) + (self.green as u32 * 256) + self.blue as u32;

        // `write!` is like `format!`, but it will write the formatted string
        // into a buffer (the first argument).
        write!(
            f,
            "RGB ({}, {}, {}) 0x{:0>6X}",
            self.red, self.green, self.blue, hex
        )
    }
}

fn main() {
    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ] {
        // Switch this to use {} once you've added an implementation
        // for fmt::Display.
        println!("{}", color);
    }
}
