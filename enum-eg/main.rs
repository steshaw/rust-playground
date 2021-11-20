use derive_more::Display;
use std::mem::*;

#[derive(Copy, Clone, Display)]
enum Format {
    Json,
    Gzip,
    S3,
}
use Format::*;

fn to_type(f: Format) -> String {
    match f {
        Json => "json".to_string(),
        Gzip => "gzip".to_string(),
        S3 => "s3".to_string(),
    }
}

fn main() {
    println!("sizeof Format = {}", size_of::<Format>());
    println!("sizeof &Format = {}", size_of::<&Format>());

    println!("{}", Json);
    println!("{}", Gzip);
    println!("{}", S3);

    for f in [Json, Gzip, S3] {
        println!("Format {} = {}", f, to_type(f));
    }
}
