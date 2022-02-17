// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut words = HashMap::new();
    for word in magazine {
        let count = words.entry(word).or_insert(0);
        *count += 1;
    }
    for word in note {
        let count = words.entry(word).or_insert(0);
        *count -= 1;
    }
    for (word, count) in words {
        if count < 0 {
            return false;
        }
    }
    return true;
}
