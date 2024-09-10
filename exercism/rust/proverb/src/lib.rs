use std::iter::once;

pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        String::new()
    } else {
        list.iter()
            .zip(list.iter().skip(1))
            .map(|words| {
                format!("For want of a {} the {} was lost.\n", words.0, words.1)
            })
            .chain(once(format!("And all for the want of a {}.", list[0])))
            .collect()
    }
}
