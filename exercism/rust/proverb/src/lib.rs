#![feature(iter_intersperse)]

pub fn build_proverb(list: &[&str]) -> String {
    list.iter()
        .zip(list.iter().skip(1))
        .map(|(first, second)| {
            format!("For want of a {first} the {second} was lost.")
        })
        .chain(
            list.iter()
                .take(1)
                .map(|first| format!("And all for the want of a {}.", first)),
        )
        .intersperse("\n".to_string())
        .collect()
}
