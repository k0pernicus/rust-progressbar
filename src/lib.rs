use std::io::prelude::*;
use std::io;

/// ProgressBar is a simple object to represent a basic time computation progression.
/// Currently, this object has not been published in "Cargo". Also, there's few risk of bugs..;
/// ## Example
/// ```
/// let current_value = 0;
/// let max_value = 100;
/// let current_character_to_display = '*';
/// let pb = ProgressBar::new(max_value, current_value, current_character_to_display);
/// loop {
///     if pb.is_done(){
///         break;
///     }
///     pb.next();
///     pb.print();
/// }
/// ```

pub struct ProgressBar {
    max_value: u32,
    current_value: u32,
    esc_char: char,
}

impl ProgressBar {

    pub fn new(max_value: u32, current_value: u32, esc_char: char) -> ProgressBar {
        ProgressBar {
            max_value: max_value,
            current_value: current_value,
            esc_char: esc_char,
        }
    }

    pub fn set_max_value(&mut self, new_max_value: u32) {
        self.max_value = new_max_value;
    }

    pub fn set_current_value(&mut self, new_current_value: u32) {
        self.current_value = new_current_value;
    }

    pub fn next(&mut self) {
        self.current_value += 1;
        if ! self.is_done() {
            print!("\r");
        }
        else {
            print!("\n");
        }
    }

    pub fn next_to(&mut self, up_to: u32) {
        self.current_value += up_to;
        if ! self.is_done() {
            print!("\r");
        }
        else {
            print!("\n");
        }
    }

    pub fn print(&self) {
        print!("> ");
        for _ in 0..self.current_value {
            print!("{0}", self.esc_char);
        }
        print!(" [{0}%]", self.current_value);
        io::stdout().flush().ok().expect("Could not flush stdout");
    }

    pub fn is_done(&self) -> bool {
        return self.current_value >= self.max_value;
    }

}
