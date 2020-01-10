#[derive(Debug)]
enum VDirection {
    Up,
    Down,
}

#[derive(Debug)]
enum HDirection {
    Left,
    Right,
}

#[derive(Debug)]
struct Ball {
    x: u32,
    y: u32,
    v_direction: VDirection,
    h_direction: HDirection,
}

#[derive(Debug)]
struct Frame {
    width: u32,
    height: u32,
}

#[derive(Debug)]
struct Game {
    frame: Frame,
    ball: Ball,
}

impl Game {
    fn new() -> Game {
        let frame = Frame {
            width: 60,
            height: 30,
        };
        let ball = Ball {
            x: 2,
            y: 4,
            v_direction: VDirection::Up,
            h_direction: HDirection::Left,
        };
        Game { frame, ball }
    }
}

fn main() {
    println!("I am bouncy! => {:#?}", Game::new());
}
