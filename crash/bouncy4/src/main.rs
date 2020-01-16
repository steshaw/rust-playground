#![feature(bool_to_option)]

use pancurses::{curs_set, endwin, initscr, Input, Window};
use std::fmt::{Display, Formatter};

type XY = (u32, u32);

#[derive(Debug)]
enum VertDir {
    Up,
    Down,
}

#[derive(Debug)]
enum HorizDir {
    Left,
    Right,
}

#[derive(Debug)]
struct Ball {
    x: u32,
    y: u32,
    vert_dir: VertDir,
    horiz_dir: HorizDir,
}

impl Ball {
    fn new() -> Ball {
        Ball {
            x: 2,
            y: 4,
            vert_dir: VertDir::Up,
            horiz_dir: HorizDir::Left,
        }
    }
}

#[derive(Debug)]
enum Err {
    BadWindowDimensions(String),
}

#[derive(Debug)]
struct Frame {
    width: u32,
    height: u32,
}

fn sensible_dim(dim_s: &str, dim: u32, initial_pos: u32) -> Result<(), Err> {
    (dim >= 10)
        .then(|| ())
        .ok_or(Err::BadWindowDimensions(format!("{} must be >= 10", dim_s)))?;
    (dim >= initial_pos)
        .then(|| ())
        .ok_or(Err::BadWindowDimensions(format!(
            "{} must be >= position of the ball {}: {}",
            dim_s, initial_pos, dim
        )))?;
    Ok(())
}

impl Frame {
    fn new(x_y: XY, ball: &Ball) -> Result<Frame, Err> {
        sensible_dim("x", x_y.0, ball.x)?;
        sensible_dim("y", x_y.1 as u32, ball.y)?;
        // Shift to make room for the border.
        Ok(Frame {
            width: x_y.0 - 2,
            height: x_y.1 - 2,
        })
    }
}

#[derive(Debug)]
struct Game {
    frame: Frame,
    ball: Ball,
}

impl Game {
    fn new(frame: Frame, ball: Ball) -> Game {
        Game { frame, ball }
    }
    fn step(&mut self) {
        self.ball.bounce(&self.frame);
        self.ball.mv();
    }
    fn draw(&self, w: &Window) {
        // Draw border.
        w.mv(0, 0);
        // XXX: Box drawing characters do not display :-(.
        let use_box_drawing_chars = false;
        if use_box_drawing_chars {
            w.draw_box('│', '─');
        } else {
            w.draw_box('|', '-');
        }

        let use_mv = true;
        if use_mv {
            w.mvaddch(self.ball.y as i32 + 1, self.ball.x as i32 + 1, 'o');
        } else {
            // Similar to old way.
            for row in 0..self.frame.height {
                for column in 0..self.frame.width {
                    w.mv((row + 1) as i32, (column + 1) as i32);
                    let c = if row == self.ball.y && column == self.ball.x {
                        'o'
                    } else {
                        ' '
                    };
                    w.addch(c);
                }
            }
        }
    }
}

impl Ball {
    fn bounce(&mut self, frame: &Frame) {
        if self.x == 0 {
            self.horiz_dir = HorizDir::Right;
        } else if self.x == frame.width - 1 {
            self.horiz_dir = HorizDir::Left;
        }

        if self.y == 0 {
            self.vert_dir = VertDir::Down;
        } else if self.y == frame.height - 1 {
            self.vert_dir = VertDir::Up;
        }
    }

    fn mv(&mut self) {
        match self.horiz_dir {
            HorizDir::Left => self.x -= 1,
            HorizDir::Right => self.x += 1,
        }
        match self.vert_dir {
            VertDir::Up => self.y -= 1,
            VertDir::Down => self.y += 1,
        }
    }
}

impl Display for Game {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        write!(fmt, "{:?}", self) // Piggyback on Debug.
    }
}

fn get_x_y(w: &Window) -> XY {
    let (max_y, max_x) = w.get_max_yx();
    (max_x as u32, max_y as u32)
}

fn game_loop(w: &Window) -> Result<(), Err> {
    let mut x_y = get_x_y(w);
    let ball = Ball::new();
    let mut game = Game::new(Frame::new(x_y, &ball)?, ball);
    loop {
        let new_x_y = get_x_y(w);
        if new_x_y != x_y {
            x_y = new_x_y;
            game.frame = Frame::new(x_y, &game.ball)?;
        }

        w.clear();
        game.draw(&w);
        w.refresh();
        if let Some(Input::Character('q')) = w.getch() {
            break;
        }
        game.step();
    }
    Ok(())
}

fn main() -> Result<(), Err> {
    // Yikes unchecked casts:
    if false {
        for i in -5i32..6i32 {
            println!("{} -> {}", i, i as u32);
        }
    }

    let w = initscr();
    let _prev_cursor = curs_set(0);
    let timeout_duration = std::time::Duration::from_millis(if false { 16 } else { 33 });
    w.timeout(timeout_duration.as_millis() as i32);

    let result = game_loop(&w);

    endwin();
    result
}
