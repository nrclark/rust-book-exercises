#![warn(clippy::pedantic)]
mod common;

fn main() {
    let value: String = common::get_value();
    let value: f32 = value.parse().expect("Failed to parse value");
    let celcius = common::fahrenheit_to_celsius(value);

    println!("Value: {value:.2}F is {celcius:.2}C");
}
