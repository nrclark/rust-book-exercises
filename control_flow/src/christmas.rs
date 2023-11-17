#![warn(clippy::pedantic)]

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
    if input.is_empty() {
        return String::new();
    }

    let mut chars = input.chars();
    let first_char = chars.next().unwrap().to_uppercase().to_string();
    first_char + chars.as_str()
}

fn lyrics(mut day: u32) -> Option<String> {
    let mut result = String::new();

    if day > 11 {
        return None;
    }

    result.push_str(
        format!(
            "On the {} day of Christmas my true love gave to me:\n",
            LINE_PAIRS[day as usize].0
        )
        .as_str(),
    );

    if day == 0 {
        result.push_str(capitalize(LINE_PAIRS[day as usize].1).as_str());
        result.push_str(".\n");
        return Some(result);
    }

    let mut capitalized: [String; 12] = Default::default();

    for (x, element) in capitalized.iter_mut().enumerate() {
        if x > (day as usize) {
            break;
        }
        *element = capitalize(LINE_PAIRS[x].1);
    }

    Some(loop {
        if day == 0 {
            result.push_str("And ");
            result.push_str(LINE_PAIRS[day as usize].1);
            result.push_str(".\n");
            break result;
        }

        result.push_str(capitalized[day as usize].as_str());
        result.push_str(",\n");
        day -= 1;
    })
}

fn main() {
    for x in 0..12 {
        println!("{}", lyrics(x).unwrap());
    }
}
