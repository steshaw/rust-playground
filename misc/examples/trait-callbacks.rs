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

#[derive(Debug, Clone)]
struct Argh{name: &'static str}

impl ClickCallback for Argh {
    fn on_click(&self, x: i64, y: i64) {
        println!("{} has been called back! x={} y={}", self.name, x, y);
    }
}

fn main() {
    let mut b = Button {
        listeners: &mut vec![Argh{name: "First"}],
    };
    b.click(1, 1);
    b.click(1, 12);
    b.listen(Argh{name: "Second"});
    b.click(2, 2);
    b.click(3, 3);
    println!("b = {:?}", b);
}
