use pancurses::{curs_set, endwin, initscr, Window};
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
        w.draw_box('|', '-');

        let use_mv = false;
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

fn main() {
    // Yikes unchecked casts:
    if false {
        for i in -5i32..6i32 {
            println!("{} -> {}", i, i as u32);
        }
    }

    let w = initscr();
    let _prev_cursor = curs_set(0);

    let (max_y, max_x) = w.get_max_yx();

    let frame = Frame {
        width: max_x as u32 - 2,
        height: max_y as u32 - 2,
    };
    println!("{:?}", frame);

    let mut game = Game::new(frame);
    let sleep_duration = std::time::Duration::from_millis(if false { 16 } else { 33 });
    for _i in 0..300 {
        w.clear();
        game.draw(&w);
        w.refresh();

        game.step();
        std::thread::sleep(sleep_duration);
    }
    w.mv(max_y - 2, 1);

    w.printw("[Hit any key to exit]");
    w.getch();
    endwin();
}
