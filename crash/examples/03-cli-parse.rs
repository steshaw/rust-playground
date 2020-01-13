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
    let a1_s = args.get(1).unwrap();
    let a2_s = args.get(2).unwrap();
    if let Ok(width) = a1_s.parse::<u32>() {
        if let Ok(height) = a2_s.parse::<u32>() {
            Ok(Frame { width, height })
        } else {
            Err(ArgsErr::InvalidInteger(a2_s.to_string()))
        }
    } else {
        Err(ArgsErr::InvalidInteger(a1_s.to_string()))
    }
}

fn main() {
    let frame_e = parse_args(std::env::args());
    println!("{:?}", frame_e);
}
