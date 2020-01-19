fn validate_fn<F>(_f: &F)
where
    F: Fn() -> (),
{
}
fn validate_fn_mut<F>(_f: &F)
where
    F: FnMut() -> (),
{
}
fn validate_fn_once<F>(_f: &F)
where
    F: FnOnce() -> (),
{
}

mod fn_ref {
    use super::*;
    pub fn main() {
        let name = String::from("Fred");

        let welcome = move || {
            let mut name = name.clone();
            name += " and Wilma";
            println!("Welcome, {}", name);
        };

        validate_fn(&welcome);

        validate_fn_mut(&welcome);

        validate_fn_once(&welcome);

        welcome();
        //println!("name = {}", name); Compile error: borrow of moved value: `name`.
        welcome();
    }
}

mod fn_mut {
    use super::*;
    pub fn main() {
        let mut name = String::from("Fred");

        let mut welcome = move || {
            name += " and Wilma";
            println!("Welcome, {}", name);
        };

        // error[E0525]: expected a closure that implements the `Fn` trait, but this closure only implements `FnMut`.
        //validate_fn(&welcome);

        validate_fn_mut(&welcome);

        validate_fn_once(&welcome);

        welcome();
        //println!("name = {}", name); // error[E0382]: borrow of moved value: `name`
        welcome();
    }
}

mod fn_once {
    use super::*;
    pub fn main() {
        let name = String::from("Fred");

        let welcome = || {
            let mut name = name;
            name += " and Wilma";
            println!("Welcome, {}", name);
        };

        // error[E0525]: expected a closure that implements the `Fn` trait, but this closure only implements `FnOnce`
        //validate_fn(&welcome);

        // error[E0525]: expected a closure that implements the `FnMut` trait, but this closure only implements `FnOnce`
        //validate_fn_mut(&welcome);

        validate_fn_once(&welcome);

        welcome();
        //println!("name = {}", name); // error[E0382]: borrow of moved value: `name`.
        //welcome(); // error[E0382]: use of moved value: `welcome`
    }
}

fn main() {
    fn_ref::main();
    fn_mut::main();
    fn_once::main();
}
