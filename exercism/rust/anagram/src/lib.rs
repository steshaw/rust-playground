use std::collections::HashSet;

pub fn anagrams_for<'a>(
    word: &str,
    possible_anagrams: &[&'a str],
) -> HashSet<&'a str> {
    // For the `word`, find anagrams among `possible_anagrams`.
    let word_lowered = word.to_lowercase();
    let word_sorted = sorted(&word_lowered);
    possible_anagrams
        .iter()
        .filter(|pa| {
            let pa_lowered = pa.to_lowercase();
            let pa_sorted = sorted(&pa_lowered);
            let anagram_of_self = word_lowered == pa_lowered;
            !anagram_of_self && word_sorted == pa_sorted
        })
        .copied()
        .collect()
}

fn sorted(word: &str) -> Vec<char> {
    let mut word_sorted = word.chars().collect::<Vec<char>>();
    word_sorted.sort();
    word_sorted
}
