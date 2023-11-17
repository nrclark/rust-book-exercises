#![warn(clippy::pedantic)]

#[allow(dead_code)]
mod common;

const LINE_PAIRS: [(&str, &str); 12] = [
    ("first", "a partridge in a pear tree"),
    ("second", "two turtle doves"),
    ("third", "three French hens"),
    ("fourth", "four calling birds"),
    ("fifth", "five gold rings"),
    ("sixth", "six geese a-laying"),
    ("seventh", "seven swans a-swimming"),
    ("eighth", "eight maids a-milking"),
    ("nineth", "nine ladies dancing"),
    ("tenth", "ten lords a-leaping"),
    ("eleventh", "eleven pipers piping"),
    ("twelfth", "twelve drummers drumming"),
];

fn capitalize(input: &str) -> String {
    let mut result = input.to_string();
    if let Some(first_char) = result.chars().next() {
        result.replace_range(
            0..first_char.len_utf8(),
            &first_char.to_uppercase().to_string(),
        );
    }
    result
}

fn lyrics(day: u32) -> Option<String> {
    let mut result = String::new();

    if !(0..=12).contains(&day) {
        return None;
    }

    result.push_str(
        format!(
            "On the {} day of Christmas my true love gave to me:\n",
            LINE_PAIRS[day as usize].0
        )
        .as_str(),
    );
    let mut day = day;

    Some(loop {
        result += capitalize(LINE_PAIRS[day as usize].1).as_str();
        match day {
            0 => {
                result.push_str(".\n");
                break result;
            }
            1 => result.push_str(", and\n"),
            _ => result.push_str(",\n"),
        }
        day -= 1;
    })
}

fn main() {
    for x in 0..12 {
        println!("{}", lyrics(x).unwrap());
    }
}
