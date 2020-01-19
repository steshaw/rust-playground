// https://www.snoyman.com/blog/2018/11/rust-crash-course-05-rule-of-three

fn call_fn<F>(f: F)
where
    F: Fn() -> (),
{
    f()
}
fn call_fn_mut<F>(mut f: F)
where
    F: FnMut() -> (),
{
    f()
}
fn call_fn_once<F>(f: F)
where
    F: FnOnce() -> (),
{
    f()
}

mod michael_fn_ref {
    use super::*;

    pub fn main() {
        println!("\nmichael_fn_ref");
        let name = "Fred".to_string();
        let say_hi = || println!("Hello, {}", name);
        call_fn(say_hi);
        call_fn_mut(say_hi);
        call_fn_once(say_hi);
        say_hi();
        say_hi();
    }
}

mod fn_ref {
    use super::*;
    pub fn main() {
        println!("\nfn_ref");
        let name = "Fred".to_string();

        let welcome = move || {
            let mut name = name.clone();
            name += " and Wilma";
            println!("Welcome, {}", name);
        };

        call_fn(&welcome);

        call_fn_mut(&welcome);

        call_fn_once(&welcome);

        welcome();
        //println!("name = {}", name); // error[E0382]: borrow of moved value: `name`
        welcome();
    }
}

mod michael_fn_mut {
    use super::*;
    pub fn main() {
        println!("\nmichael_fn_mut");
        let mut say_hi = {
            let mut name = String::from("Alice");
            move || {
                name += " and Bob";
                println!("Hello, {}", name);
            }
        };
        //call_fn(say_hi);
        call_fn_mut(&mut say_hi);
        call_fn_mut(&mut say_hi);
        call_fn_once(&mut say_hi);
        call_fn_once(&mut say_hi);

        say_hi();
        say_hi();
    }
}

mod fn_mut {
    use super::*;
    pub fn main() {
        println!("\nfn_mut");
        let mut name = String::from("Fred");

        let mut welcome = move || {
            name += " and Wilma";
            println!("Welcome, {}", name);
        };

        // error[E0525]: expected a closure that implements the `Fn` trait, but this closure only implements `FnMut`.
        //call_fn(welcome);

        call_fn_mut(&mut welcome);

        call_fn_once(&mut welcome);

        welcome();
        welcome();
    }
}

mod michael_fn_once {
    use super::*;
    pub fn main() {
        println!("\nmichael_fn_once");
        let say_hi = {
            let name = String::from("Alice");
            || {
                let name = name;
                println!("Hello, {}", name);
            }
        };

        //call_fn(say_hi);
        //call_fn_mut(say_hi);
        call_fn_once(say_hi);
    }
}

mod michael_fn_once_fixed {
    use super::*;
    pub fn main() {
        println!("\nmichael_fn_once_fixed");
        let say_hi = {
            let name = "Alice".to_string();
            move || println!("Hello, {}", name)
        };

        call_fn(&say_hi);
        call_fn_mut(&say_hi);
        call_fn_once(&say_hi);
    }
}

mod michael_fn_once2 {
    use super::*;
    pub fn main() {
        println!("\nmichael_fn_once2");
        let say_hi = {
            let name = String::from("Alice");
            || std::mem::drop(name)
        };
        //call_fn(say_hi);
        //call_fn_mut(say_hi);
        call_fn_once(say_hi);
    }
}

mod fn_once {
    use super::*;
    pub fn main() {
        println!("\nfn_once");
        let name = String::from("Fred");

        let welcome = || {
            let mut name = name;
            name += " and Wilma";
            println!("Welcome, {}", name);
        };

        // error[E0525]: expected a closure that implements the `Fn` trait, but this closure only implements `FnOnce`
        //call_fn(&welcome);

        // error[E0525]: expected a closure that implements the `FnMut` trait, but this closure only implements `FnOnce`
        //call_fn_mut(&welcome);

        call_fn_once(welcome);
    }
}

mod fn_mut_ref2 {
    use super::*;
    pub fn main() {
        let mut name = String::from("Alice");
        let mut say_hi = || {
            name += " and Bob";
            println!("Hello, {}", name);
        };
        //call_fn(say_hi);
        call_fn_mut(&mut say_hi);
        call_fn_once(&mut say_hi);

        println!("name = {}", name);
    }
}

fn main() {
    michael_fn_ref::main();
    fn_ref::main();
    michael_fn_mut::main();
    fn_mut::main();
    michael_fn_once::main();
    michael_fn_once_fixed::main();
    michael_fn_once2::main();
    fn_once::main();
    fn_mut_ref2::main();
}
