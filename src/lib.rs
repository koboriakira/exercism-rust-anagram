use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let word = word.to_lowercase();
    let word_chars = sorted(&word);
    possible_anagrams
        .iter()
        .filter(|predicate| {
            let predicate = predicate.to_lowercase();
            if predicate.to_lowercase() == word {
                return false;
            } else {
                let possible_anagram_chars = sorted(&predicate);
                word_chars.eq(&possible_anagram_chars)
            }
        })
        .cloned()
        .collect()
}

fn sorted(word: &str) -> Vec<char> {
    let mut word_chars = word.chars().collect::<Vec<char>>();
    word_chars.sort();
    word_chars
}
