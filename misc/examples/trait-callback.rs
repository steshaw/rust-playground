use std::fmt::*;

trait ClickCallback {
    fn on_click(&self, x: i64, y: i64);
}

#[derive(Debug)]
struct Button<'a, T: ClickCallback + Clone> {
    listeners: &'a mut Vec<T>,
}

impl<T: ClickCallback + Clone> Button<'_, T> {
    fn listen(&mut self, c: T) {
        self.listeners.push(c)
    }
    fn click(self: &Self, x: i64, y: i64) {
        for l in self.listeners.clone() {
            l.on_click(x, y);
        }
    }
}

/*
fn call_with_one<F>(func: F) -> usize
where
    F: Fn(usize) -> usize,
{
    func(1)
}

let double = |x| x * 2;
assert_eq!(call_with_one(double), 2);
*/

type CB = dyn Fn(&'static str, i64, i64) -> ();

#[derive(Clone)]
struct FnCallback<'a>
//where F: Fn(FnCallback, i64, i64) -> () {
{
    name: &'static str,
    cb: &'a CB,
}

impl<'a> Debug for FnCallback<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        std::fmt::Debug::fmt(&self.name, f)
    }
}

impl<'a> ClickCallback for FnCallback<'a> {
    fn on_click(&self, x: i64, y: i64) {
        let f = self.cb;
        f(self.name, x, y);
    }
}

fn main() {
    let first = |name, x, y| {
        println!("1st: name={}, x={}, y={}", name, x, y);
    };
    let mut b = Button {
        listeners: &mut vec![FnCallback {
            name: "First",
            cb: &first,
        }],
    };
    b.click(1, 1);
    b.click(1, 12);
    let second = |name, x, y| {
        println!("2nd: name={}, x={}, y={}", name, x, y);
    };
    b.listen(FnCallback {
        name: "Second",
        cb: &second,
    });
    b.click(2, 2);
    b.click(3, 3);
    println!("b = {:?}", b);
}
