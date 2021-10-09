#[derive(Clone, Copy, Debug)]
enum Op {
    BRK,
    JSR,
}
use Op::*;

impl Op {
    fn info(self) -> Info {
        match self {
            BRK => Info::new(0x00, 1),
            JSR => Info::new(0x20, 3),
        }
    }
}

struct Info {
    code: u32,
    size: u8,
}

impl Info {
    fn new(code: u32, size: u8) -> Self {
        Info {
            code: code,
            size: size,
        }
    }
}

fn main() {
    for op in vec![BRK, JSR] {
        let i = op.info();
        println!("{:?}= {} (size {})", op, i.code, i.size)
    }
}
