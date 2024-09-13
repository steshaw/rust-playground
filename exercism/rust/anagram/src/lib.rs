use std::collections::HashSet;

pub fn anagrams_for<'a>(
    word: &str,
    possible_anagrams: &[&'a str],
) -> HashSet<&'a str> {
    // For the '{word}' word find anagrams among the following words: {possible_anagrams:?}.
    let word_lowered = word.to_lowercase().chars().collect::<Vec<_>>();
    let mut word_sorted = word_lowered.clone();
    word_sorted.sort();
    let mut result = HashSet::new();
    for a in possible_anagrams {
        let a_lowered = a.to_lowercase().chars().collect::<Vec<_>>();
        let mut a_sorted = a_lowered.clone();
        a_sorted.sort();
        if word_lowered != a_lowered && word_sorted == a_sorted {
            result.insert(*a);
        }
    }
    result
}
