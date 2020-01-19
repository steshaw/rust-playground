fn pass_by_value(_x: String) {}

fn pass_by_ref(_x: &String) {}

fn pass_by_mut_ref(x: &mut String) {
    if false {
        pass_by_mut_ref(x);
        pass_by_mut_ref(x);
    }
    pass_by_ref(x); // that's fine
    pass_by_ref(x); // that's fine
    //pass_by_value(*x); // Compile error: cannot move out of `*x` which is behind a mutable reference.
}

fn main() {
    pass_by_mut_ref(&mut "fred".into());
}
