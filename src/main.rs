use std::io;
use std::io::prelude::*;
use std::fmt;

struct Rectangle {
    width: i32,
    height: i32
}

impl Rectangle {
    fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height
        }
    }
    
    fn area(&self) -> i32 {
        self.width * self.height
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
"Rectangle {{
    width: {},
    height: {}
}}",
            self.width,
            self.height
        )
    }
}

// Unlike println!(), print!() does not automatically flush stdout.
fn print_and_flush(msg: &str) {
    print!("{}", msg);
    io::stdout().flush().unwrap();
}

fn geti32() -> i32 {
    let stdin = io::stdin();
    loop {
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();

        match buf.trim().parse::<i32>() {
            Ok(val) => break val,
            Err(_)  => {
                print_and_flush("Sorry, that didn't work. Try again: ");
                continue;
            }
        };
    }
}

fn main() {
    
    let rect = Rectangle::new(
        {
            print_and_flush("Enter the rectange width: ");
            geti32()
        }, {
            print_and_flush("Enter the rectangle height: ");
            geti32()
        }
    );

    println!("Result: {}", rect);
    println!("Area: {}", rect.area());
}