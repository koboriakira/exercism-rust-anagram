use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut result: HashSet<&str> = HashSet::new();

    let word = word.to_lowercase();
    let word_chars = sorted(&word);
    for &possible_anagram in possible_anagrams {
        if word.to_lowercase() == possible_anagram.to_lowercase() {
            continue;
        }
        let possible_anagram_chars = sorted(possible_anagram);
        if word_chars.eq(&possible_anagram_chars) {
            result.insert(possible_anagram);
        }
    }
    return result;
}

fn sorted(word: &str) -> Vec<char> {
    let mut word_chars = word.to_lowercase().chars().collect::<Vec<char>>();
    word_chars.sort();
    word_chars
}
