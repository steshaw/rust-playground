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
    fn k(width_s: &str, height_s: &str) -> Result<Frame, ArgsErr> {
        let width = width_s
            .parse::<u32>()
            .or(Err(ArgsErr::InvalidInteger(width_s.to_string())))?;
        let height = height_s
            .parse::<u32>()
            .or(Err(ArgsErr::InvalidInteger(height_s.to_string())))?;

        Ok(Frame { width, height })
    }
    // FIXME: Early returns and panics galore!
    let args = args.collect::<Vec<_>>();
    let a1_s = args.get(1);
    let a2_s = args.get(2);
    let a3_s = args.get(3);
    match (a1_s, a2_s, a3_s) {
        (None, None, None) => Err(ArgsErr::TooFew),
        (Some(_), None, None) => Err(ArgsErr::TooFew),
        (Some(width_s), Some(height_s), None) => k(width_s, height_s),
        (Some(_), Some(_), Some(_)) => Err(ArgsErr::TooMany),
        (_, _, _) => {
            // Other case are impossible!
            Err(ArgsErr::TooMany)
        }
    }

    /*
    if args.len() < 3 {
        return Err(ArgsErr::TooFew);
    }
    if args.len() > 3 {
        return Err(ArgsErr::TooMany);
    }
    if let Ok(width) = a1_s.parse::<u32>() {
        if let Ok(height) = a2_s.parse::<u32>() {
            Ok(Frame { width, height })
        } else {
            Err(ArgsErr::InvalidInteger(a2_s.to_string()))
        }
    } else {
        Err(ArgsErr::InvalidInteger(a1_s.to_string()))
    }
    */
}

fn main() {
    let frame_e = parse_args(std::env::args());
    println!("{:?}", frame_e);
}
