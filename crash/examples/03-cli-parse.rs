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

fn parse_args(args: std::env::Args) -> Result<Frame, ArgsErr> {
    // FIXME: Early returns and panics galore!
    if args.len() < 3 {
        return Err(ArgsErr::TooFew);
    }
    if args.len() > 3 {
        return Err(ArgsErr::TooMany);
    }
    let args = args.collect::<Vec<_>>();
    if let Ok(width) = args.get(1).unwrap().parse::<u32>() {
        if let Ok(height) = args.get(2).unwrap().parse::<u32>() {
            Ok(Frame { width, height })
        } else {
            Err(ArgsErr::InvalidInteger(args.get(2).unwrap().to_string()))
        }
    } else {
        Err(ArgsErr::InvalidInteger(args.get(1).unwrap().to_string()))
    }
}

fn main() {
    let frame_e = parse_args(std::env::args());
    println!("{:?}", frame_e);
}
