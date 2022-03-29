use std::char;
use std::collections::HashMap;
fn main() {
    let letters_map = get_char_values();
    let word = "";
    println!("{}", calculate_value_from_str(word, letters_map))
}

fn get_char_values() -> HashMap<char, u32> {
    let mut values: HashMap<char, u32> = HashMap::new();

    let mut i = 97;
    while i < 123 {
        values.insert(char::from_u32(i).unwrap(), i);
        i += 1;
    }
    return values;
}

fn calculate_value_from_str(word: &str, dict: HashMap<char, u32>) -> u32 {
    let char_list: Vec<char> = word.to_lowercase().chars().collect();
    let mut final_value: u32 = 0;
    for letter in char_list {
        final_value += dict.get(&letter).unwrap();
    }
    return final_value;
}
