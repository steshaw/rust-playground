fn output_arg(arg: &str) {
    let cs = arg.chars();
    println!(
        "arg: {}, characters: {}, bytes: {}",
        arg,
        cs.collect::<Vec<_>>().len(),
        arg.len()
    );
}
fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() == 1 {
        let peace = "שלום";
        output_arg(peace);
    } else {
        for a in args {
            output_arg(&a)
        }
    }
}
