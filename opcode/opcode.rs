#[derive(Clone, Copy, Debug)]
enum Op {
    BRK,
    JSR,
}
use Op::*;

struct Info {
    code: u32,
    size: u8,
}

fn info(op: Op) -> Info {
    match op {
        BRK => Info {
            code: 0x00,
            size: 1,
        },
        JSR => Info {
            code: 0x20,
            size: 3,
        },
    }
}

fn main() {
    for op in vec![BRK, JSR] {
        let i = info(op);
        println!("{:?}= {} (size {})", op, i.code, i.size)
    }
}
