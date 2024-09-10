#![feature(iter_intersperse)]

pub fn build_proverb(list: &[&str]) -> String {
    list.iter()
        .zip(list.iter().skip(1))
        .map(|words| {
            format!("For want of a {} the {} was lost.", words.0, words.1)
        })
        .chain(
            list.iter()
                .take(1)
                .map(|first| format!("And all for the want of a {}.", first)),
        )
        .intersperse("\n".to_string())
        .collect()
}
