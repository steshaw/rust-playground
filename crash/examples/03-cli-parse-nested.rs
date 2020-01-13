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

    let mut get_arg = || args.next().ok_or(TooFew);

    let width_s = get_arg()?;
    let height_s = get_arg()?;

    // Require end of arguments here.
    match args.next() {
        None => Ok(()),
        Some(_) => Err(TooMany)
    }?;

    let p_i = |s: String| s.parse().or(Err(InvalidInteger(s)));

    let width = p_i(width_s)?;
    let height = p_i(height_s)?;

    Ok(Frame { width, height })
}

fn main() {
    let frame_e = parse_args(std::env::args());
    println!("{:?}", frame_e);
}
