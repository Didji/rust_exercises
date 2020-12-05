static VOWELS: [char; 6] = ['a', 'e', 'i', 'o', 'u', 'y'];

fn main() {
    let input_string = String::from("Brrrr the mail comes in three batches per day and the square peg will settle in the round hole");

    println!("{}", transform_input_string(input_string));
}

fn transform_input_string(input_string: String) -> String {
    let mut output: Vec<String> = vec![];

    for word in input_string.split_whitespace() {
        output.push(transform_word(&mut word.to_lowercase()));
    }
    output.join(" ")
}

fn transform_word(word: &mut String) -> String {
    if starts_with_vowel(word) {
        return append_suffix(word, "hay")
    } else {
        return pig_consonant(word)
    }
}

fn starts_with_vowel(word: &String) -> bool {
    VOWELS.iter().any(|&v| word.starts_with(v))
}

fn append_suffix(word: &mut String, append: &str) -> String {
    word.push_str(append);
    word.to_string()
}

fn pig_consonant(word: &mut String) -> String {
    let mut new_word = word.clone();
    let first_vowel = find_position_of_first_vowel(word);
    if let Some(position) = first_vowel {
        new_word = move_first_consonants_to_end(word, position);
    }
    append_suffix(&mut new_word, "ay")
}

fn find_position_of_first_vowel(word: &String) -> Option<usize> {
    VOWELS
        .iter()
        .map(|&v| word.find(v))
        .into_iter()
        .flat_map(|v| v)
        .min()
}

fn move_first_consonants_to_end(word: &mut String, position_of_first_vowel: usize) -> String {
    let after_vowel = word.split_off(position_of_first_vowel);
    format!("{}{}", after_vowel, word)
}
