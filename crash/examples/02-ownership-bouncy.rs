use std::fmt;
use std::fmt::{Display, Formatter};
use std::{thread, time};

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
            height: 15,
        };
        let ball = Ball {
            x: frame.width / 2,
            y: frame.height / 2,
            v_direction: VDirection::Up,
            h_direction: HDirection::Left,
        };
        Game { frame, ball }
    }

    fn step(&mut self) {
        self.ball.bounce(&self.frame);
        self.ball.mv();
    }
}

fn write_row(frame: &Frame, fmt: &mut Formatter) -> fmt::Result {
    write!(fmt, "+")?;
    for _ in 0..frame.width {
        write!(fmt, "-")?;
    }
    writeln!(fmt, "+")
}

impl Display for Game {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write_row(&self.frame, fmt)?;
        for y in (0..self.frame.height).rev() {
            write!(fmt, "|")?;
            for x in 0..self.frame.width {
                let c = if self.ball.x == x && self.ball.y == y {
                    "o"
                } else {
                    " "
                };
                write!(fmt, "{}", c)?;
            }
            writeln!(fmt, "|")?;
        }
        write_row(&self.frame, fmt)
    }
}

impl Ball {
    fn bounce(&mut self, frame: &Frame) {
        // Switch horizontal direction. i.e. x.
        if self.x == 0 {
            self.h_direction = HDirection::Right;
        } else if self.x == frame.width - 1 {
            self.h_direction = HDirection::Left;
        }
        // Switch veritical direction. i.e. y.
        if self.y == 0 {
            self.v_direction = VDirection::Up;
        } else if self.y == frame.height - 1 {
            self.v_direction = VDirection::Down;
        }
    }

    fn mv(&mut self) {
        // Move in horizontal direction.
        match self.h_direction {
            HDirection::Left => self.x -= 1,
            HDirection::Right => self.x += 1,
        }
        // Move in vertical direction.
        match self.v_direction {
            VDirection::Down => self.y -= 1,
            VDirection::Up => self.y += 1,
        }
    }
}

fn clear() {
    print!("\x1B[H\x1B[2J\x1B[3J");
}

fn main() {
    let mut game = Game::new();
    println!("Game initial => {:#?}", game);
    for _ in 1..100 {
        clear();
        println!("{}", game);
        game.step();
        thread::sleep(time::Duration::from_millis(33));
    }
    println!("Game final => {:#?}", game);
}
