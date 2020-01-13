use std::fmt;
use std::fmt::{Display, Formatter};
use std::{thread, time};

mod parse_args;
use parse_args::{parse_args, Frame};

const ESC: &str = "\x1B";

fn clear() {
    print!("\x1B[H\x1B[2J\x1B[3J");
}

#[allow(dead_code)]
fn cursor_move_up(fmt: &mut Formatter, n: u32) -> fmt::Result {
    write!(fmt, "{}[{}A", ESC, n)
}

#[allow(dead_code)]
fn cursor_move_down(fmt: &mut Formatter, n: u32) -> fmt::Result {
    write!(fmt, "{}[{}B", ESC, n)
}

#[allow(dead_code)]
fn cursor_move_right(fmt: &mut Formatter, n: u32) -> fmt::Result {
    write!(fmt, "{}[{}C", ESC, n)
}

#[allow(dead_code)]
fn cursor_move_left(fmt: &mut Formatter, n: u32) -> fmt::Result {
    write!(fmt, "{}[{}D", ESC, n)
}

fn cursor_move(fmt: &mut Formatter, x: u32, y: u32) -> fmt::Result {
    write!(fmt, "{}[{};{}f", ESC, y, x)
}

fn cursor_save(fmt: &mut Formatter) -> fmt::Result {
    write!(fmt, "{}7", ESC)
}

fn cursor_restore(fmt: &mut Formatter) -> fmt::Result {
    write!(fmt, "{}8", ESC)
}

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

    fn prev_xy(&self) -> (u32, u32) {
        let x = match self.h_direction {
            HDirection::Left => self.x + 1,
            HDirection::Right => self.x - 1,
        };
        let y = match self.v_direction {
            VDirection::Down => self.y + 1,
            VDirection::Up => self.y - 1,
        };
        (x, y)
    }
}

impl Display for Frame {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        let write_row = |fmt: &mut Formatter| {
            write!(fmt, "+")?;
            for _ in 0..self.width {
                write!(fmt, "-")?;
            }
            write!(fmt, "+")
        };

        write_row(fmt)?;
        for y in (0..self.height).rev() {
            cursor_move(fmt, 1, y + 2)?;
            write!(fmt, "|")?;
            cursor_move(fmt, self.width + 2, y + 2)?;
            write!(fmt, "|")?;
        }
        cursor_move(fmt, 1, self.height + 2)?;
        write_row(fmt)?;
        cursor_save(fmt)
    }
}

#[derive(Debug)]
struct Game {
    frame: Frame,
    ball: Ball,
}

impl Game {
    fn new(frame: Frame) -> Game {
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

impl Display for Game {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        // Erase current ball.
        let erase = true;
        if erase {
            let prev_pos = self.ball.prev_xy();
            cursor_move(fmt, prev_pos.0 + 2, prev_pos.1 + 2)?;
            write!(fmt, " ")?;
        }

        // Draw ball.
        cursor_move(fmt, self.ball.x + 2, self.ball.y + 2)?;
        write!(fmt, "o")?;

        // Restore the cursor to a position after the frame was drawn.
        cursor_restore(fmt)
    }
}

fn main() -> Result<(), parse_args::ArgsErr> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let frame = parse_args(args)?;
    let mut game = Game::new(frame);
    let log_enabled = true;

    if log_enabled {
        println!("Game initial => {:#?}", game)
    };

    clear();
    let num_frames = 300;
    for _ in 0..num_frames {
        println!("{}", game);
        game.step();
        thread::sleep(time::Duration::from_millis(16));
    }

    if log_enabled {
        println!("Game final => {:#?}", game)
    };

    Ok(())
}
