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
    fn k(width_s: &str, height_s: &str) -> Result<Frame, ArgsErr> {
        fn p(u32_s: &str) -> Result<u32, ArgsErr> {
            u32_s
                .parse::<u32>()
                .or(Err(ArgsErr::InvalidInteger(u32_s.to_string())))
        }
        let width = p(width_s)?;
        let height = p(height_s)?;
        Ok(Frame { width, height })
    }
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
}

fn main() {
    let frame_e = parse_args(std::env::args());
    println!("{:?}", frame_e);
}
