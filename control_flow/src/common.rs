use std::env;
use std::io;

pub fn fahrenheit_to_celsius(temperature: f32) -> f32 {
    (temperature - 32.0) / 180.0 * 100.0
}

pub fn get_value() -> String {
    if let Some(n) = env::args().nth(1) {
        n.trim().to_string()
    } else {
        let mut value = String::new();
        io::stdin()
            .read_line(&mut value)
            .expect("Failed to read value");
        value.trim().to_owned()
    }
}
