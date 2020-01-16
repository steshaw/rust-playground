#![feature(bool_to_option)]

use pancurses::{curs_set, endwin, initscr, Input, Window};
use std::fmt::{Display, Formatter};

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
    fn new(frame: Frame) -> Game {
        let ball = Ball {
            x: 2,
            y: 4,
            vert_dir: VertDir::Up,
            horiz_dir: HorizDir::Left,
        };
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

#[derive(Debug)]
enum Err {
    BadWindowDimensions,
}

fn sensible(dim: u32, initial_pos: u32) -> Result<(), Err> {
    (dim >= 20 && dim >= initial_pos)
        .then(|| ())
        .ok_or(Err::BadWindowDimensions)
}

fn validate(game: &Game, max_x: u32, max_y: u32) -> Result<(), Err> {
    sensible(max_x as u32, game.ball.x).and_then(|_| sensible(max_y as u32, game.ball.y))
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

    let (max_y, max_x) = w.get_max_yx();
    let max_y = max_y as u32;
    let max_x = max_x as u32;
    println!("max_y = {}, max_x = {}", max_y, max_x);

    let frame = Frame {
        width: max_x - 2,
        height: max_y - 2,
    };
    println!("{:?}", frame);

    let mut game = Game::new(frame);

    let timeout_duration = std::time::Duration::from_millis(if false { 16 } else { 33 });
    w.timeout(timeout_duration.as_millis() as i32);

    let result = validate(&game, max_x, max_y).and_then(|_| {
        for _i in 0..300 {
            w.clear();
            game.draw(&w);
            w.refresh();
            if let Some(Input::Character('q')) = w.getch() {
                break;
            }
            game.step();
        }
        w.mv(max_y as i32 - 2, 1);
        w.printw("[Hit any key to exit]");
        w.getch();
        Ok(())
    });

    endwin();
    result
}
