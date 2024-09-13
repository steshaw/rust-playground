/// What should the type of _function be?
pub fn map<A, B>(input: Vec<A>, mut f: impl FnMut(A) -> B) -> Vec<B> {
    // Transform input vector, input, using passed function.
    let mut result: Vec<B> = Vec::new();
    for i in input {
        result.push(f(i));
    }
    result
}
