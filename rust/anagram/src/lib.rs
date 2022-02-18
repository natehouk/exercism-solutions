use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams: HashSet<&'a str> = HashSet::new();
    let mut sorted: Vec<char> = word.to_lowercase().chars().collect();
    sorted.sort_unstable();
    for possible in possible_anagrams {
        let mut anagram: Vec<char> = possible.to_lowercase().chars().collect();
        anagram.sort_unstable();
        if anagram == sorted && possible.to_lowercase().chars().collect::<Vec<char>>() != word.to_lowercase().chars().collect::<Vec<char>>() {
            anagrams.insert(possible);
        }
    }
    anagrams
}
