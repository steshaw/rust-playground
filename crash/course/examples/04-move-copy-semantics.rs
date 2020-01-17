/// This has "move semantics".
#[derive(Debug)]
struct Mv;

fn toot_mv(mv: &Mv) {
    println!("{:#?}", mv);
}

// This has "copy semantics".
#[derive(Debug)]
struct Cp;

// Copy is a compiler-generated bit-wise copy.
// However, "moves" are also bit-wise copies.
// Either may be optimised away by the compiler (but there are no
// guarantees).
impl Copy for Cp {}

impl Clone for Cp {
    fn clone(&self) -> Self {
        unimplemented!() // NOTE: Never invoked during this example.
    }
}

fn toot_cp(cp: Cp) {
    println!("{:#?}", cp);
}

// This cannot be Copy because String is not Copy.
#[derive(Debug)]
struct CannotBeCopy(String);

/*
impl Copy for CannotBeCopy {} // Compile error: trait 'Copy' may not be
implemented for this type.
*/

fn move_semantics() {
    let mv1 = Mv;
    let mv2 = mv1; // mv1 is "moved" to mv2.

    // NOTE: "Under the hood, both a copy and a move can result in bits
    // being copied in memory, although this is sometimes optimized away."
    // https://doc.rust-lang.org/std/marker/trait.Copy.html

    // NOTE: mv1 cannot be accessed here.
    // 'value used here after move'.
    // move occurs because `mv1` has type `Mv`, which does not implement the
    // `Copy` trait
    /*
    toot_mv(mv1); // Compile error!
    */

    // We can access mv2.
    toot_mv(&mv2);
    // We can access mv2 again since we passes "by reference" to toot_mv.
    toot_mv(&mv2);
    toot_mv(&mv2);
}

fn copy_semantics() {
    let cp1 = Cp;
    let cp2 = cp1; // mv1 is "copyied" to mv2. Doesn't use Clone.

    // NOTE: "Under the hood, both a copy and a move can result in bits
    // being copied in memory, although this is sometimes optimized away."
    // https://doc.rust-lang.org/std/marker/trait.Copy.html

    // NOTE: cp1 and cp2 can be accessed here.
    toot_cp(cp1);
    toot_cp(cp2);
    toot_cp(cp1);
    toot_cp(cp2);
}

fn cannot_be_copy() {
    let cbc1 = CannotBeCopy("Hello".into());
    // Apparently, the following can result in "bits being copied in
    // memory", even though we don't implement Copy and String doesn't
    // implemnted Copy.
    let cbc2 = cbc1;
    /*
    println!("{:?}", cbc1); // Compile error here.
    */
    println!("{:?}", cbc2);
}

fn main() {
    move_semantics();
    copy_semantics();
    cannot_be_copy();
}
