use std::collections::HashSet;
use std::collections::HashMap;

type Freq = HashMap<char, u32>;

fn update_count(c: char, letter_to_count: Freq) -> Freq {
    let mut cp = HashMap::clone(&letter_to_count);
    cp.entry(c).and_modify(|count| *count += 1).or_insert(1);
    cp
}

pub fn frequencies(text: &str) -> Freq {
    text.to_lowercase().chars().fold(HashMap::new(),
                                     |freq, c| update_count(c, freq))
}

fn is_anagram(target: &str, candidate: &str) -> bool {
    frequencies(target) == frequencies(candidate) && target.to_lowercase() != candidate.to_lowercase()
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    possible_anagrams.iter()
        .filter(|src| is_anagram(word, src))
        .copied()
        .collect()

}
