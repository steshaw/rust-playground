use std::fmt::*;

trait ClickCallback {
    fn on_click(&self, x: i64, y: i64);
}

//#[derive(Debug)]
struct Button {
    listeners: Vec<Box<dyn ClickCallback>>,
}

impl Button {
    fn listen(&mut self, cc: Box<dyn ClickCallback>) {
        self.listeners.push(cc)
    }
    fn click(self: &Self, x: i64, y: i64) {
        for l in self.listeners.iter() {
            l.on_click(x, y);
        }
    }
}

#[derive(Debug, Clone)]
struct Argh {
    name: &'static str,
}

impl ClickCallback for Argh {
    fn on_click(&self, x: i64, y: i64) {
        println!("{} has been called back! x={} y={}", self.name, x, y);
    }
}

fn main() {
    let mut b = Button {
        listeners: vec![Box::new(Argh { name: "First" })],
    };
    b.click(1, 1);
    b.click(1, 12);
    b.listen(Box::new(Argh { name: "Second" }));
    b.click(2, 2);
    b.click(3, 3);
    println!("b.listeners.len = {}", b.listeners.len());
}
