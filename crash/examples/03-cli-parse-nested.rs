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

    let width_s = args.next().ok_or(TooFew)?;
    let height_s = args.next().ok_or(TooFew)?;

    if args.next().is_some() {
        return Err(TooMany);
    };

    let width = width_s.parse().or(Err(InvalidInteger(width_s)))?;
    let height = height_s.parse().or(Err(InvalidInteger(height_s)))?;

    Ok(Frame { width, height })
}

fn main() {
    let frame_e = parse_args(std::env::args());
    println!("{:?}", frame_e);
}
