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

struct ParseArgs(std::env::Args);

impl ParseArgs {
    /*
    fn new() -> ParseArgs {
        unimplemented!()
    }
    */

    fn require_arg(&mut self) -> Result<String, ArgsErr> {
       self.0.next().ok_or(ArgsErr::TooFew)
    }
}

#[allow(clippy::or_fun_call)]
fn parse_args(args: std::env::Args) -> Result<Frame, ArgsErr> {
    use self::ArgsErr::*;

    //let mut args = args.skip(1);
    let mut args = ParseArgs(args);

    // Ignore command name.
    args.require_arg()?;

    let width_s = args.require_arg()?;
    let height_s = args.require_arg()?;

    // Require end of arguments here.
    let mut require_no_more_args = || match args.0.next() {
        None => Ok(()),
        Some(_) => Err(TooMany),
    };

    require_no_more_args()?;

    let parse_u32 = |s: String| s.parse().or(Err(InvalidInteger(s)));

    let width = parse_u32(width_s)?;
    let height = parse_u32(height_s)?;

    Ok(Frame { width, height })
}

fn main() {
    let frame_e = parse_args(std::env::args());
    println!("{:?}", frame_e);
}
