fn main() {
    let input_string = String::from("Brrrr the mail comes in three batches per day and the square peg will settle in the round hole");
    let mut output: Vec<String> = vec![];
    let vowels = ['a', 'e', 'i', 'o', 'u', 'y'];

    for word in input_string.split_whitespace() {
        let mut lower_word = word.to_lowercase();
        let starts_with_vowel = vowels.iter().any(|&v| lower_word.starts_with(v));
        if starts_with_vowel {
            output.push(append_suffix(&mut lower_word, "hay"));
        } else {
            output.push(pig_consonant(&mut lower_word, &vowels));
        }
    }

    println!("{}", output.join(" "));
}

fn append_suffix(word: &mut String, append: &str) -> String {
    word.push_str(append);
    word.to_string()
}

fn pig_consonant(word: &mut String, vowels: &[char; 6]) -> String {
    let mut new_word = word.clone();
    // Finds the position of the first vowel in the word
    let first_vowel = vowels
        .iter()
        .map(|&v| word.find(v))
        .into_iter()
        .flat_map(|v| v)
        .min();
    // If the word contains a vowel, we move all the consonants before it to the end of the word
    if let Some(pos) = first_vowel {
        let after_vowel = word.split_off(pos);
        new_word = format!("{}{}", after_vowel, word);
    }    
    append_suffix(&mut new_word, "ay")
}
