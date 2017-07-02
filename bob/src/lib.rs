/*
Bob is a lackadaisical teenager. In conversation, his responses are very limited.

Bob answers 'Sure.' if you ask him a question.

He answers 'Whoa, chill out!' if you yell at him.

He says 'Fine. Be that way!' if you address him without actually saying
anything.

He answers 'Whatever.' to anything else.
*/
pub fn reply(s : &'static str) -> &'static str {
    let s = s.trim_right();
    if s.is_empty() {
        "Fine. Be that way!"
    } else if s.chars().any(|c| c.is_alphabetic()) && s.chars().all(|c| !c.is_alphabetic() || c.is_uppercase()) {
        "Whoa, chill out!"
    } else if s.chars().last() == Some('?') {
        "Sure."
    } else {
        "Whatever."
    }
}
