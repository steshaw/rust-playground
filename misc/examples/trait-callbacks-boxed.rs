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
        // FIXME: Found out how to use an iterator here.
        println!("Cannot broadcast click");
        let l = self.listeners.len();
        println!("num listeners = {}", l);
        if l == 1 {
            self.listeners[0].on_click(x, y);
        }else if l == 2 {
            self.listeners[0].on_click(x, y);
            self.listeners[1].on_click(x, y);
        }
        /*
        for l in self.listeners() {
            l.on_click(x, y);
        }
        */
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
        listeners: vec![Box::new(Argh{name: "First"})],
    };
    b.click(1, 1);
    b.click(1, 12);
    b.listen(Box::new(Argh{name: "Second"}));
    b.click(2, 2);
    b.click(3, 3);
    println!("b.listeners.len = {}", b.listeners.len());
}
