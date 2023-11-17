#![warn(clippy::pedantic)]

#[allow(dead_code)]
mod common;

fn fibonacci(mut position: u32) -> u32 {
    match position {
        0 => 0,
        1 | 2 => 1,
        _ => {
            let mut buf: [u32; 2] = [0, 1];
            while position != 0 {
                position -= 1;
                buf = [buf[1], buf[0] + buf[1]];
            }
            buf[0]
        }
    }
}

fn suffix(value: u32) -> &'static str {
    if (11..=13).contains(&value) {
        "th"
    } else {
        match value % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        }
    }
}

fn main() {
    let position: String = common::get_value();
    let position: u32 = position.parse().expect("Failed to parse value");
    let value = fibonacci(position);
    println!(
        "The {position}{} fibonacci value is {value}",
        suffix(position)
    );
}
