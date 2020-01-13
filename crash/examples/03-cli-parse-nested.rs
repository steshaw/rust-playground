#[derive(Debug)]
struct Frame {
    width: u32,
    height: u32,
}

#[derive(Debug)]
enum ArgsErr {
    TooFew,
    TooMany,
    InvalidInteger(String),
}

#[allow(clippy::or_fun_call)]
fn parse_args(args: std::env::Args) -> Result<Frame, ArgsErr> {
    use self::ArgsErr::*;

    let mut args = args.skip(1);

    let width_s = match args.next() {
        None => return Err(TooFew),
        Some(width_s) => width_s,
    };

    let height_s = match args.next() {
        None => return Err(TooFew),
        Some(height_s) => height_s,
    };

    if args.next().is_some() {
        return Err(TooMany);
    };

    match width_s.parse() {
            Err(_) => Err(InvalidInteger(width_s)),
            Ok(width) => match height_s.parse() {
                Err(_) => Err(InvalidInteger(height_s)),
                Ok(height) => Ok(Frame { width, height }),
            },
    }
}

fn main() {
    let frame_e = parse_args(std::env::args());
    println!("{:?}", frame_e);
}
