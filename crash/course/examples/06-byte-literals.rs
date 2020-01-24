use std::io;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bytearray1: &[u8; 23] = b"Hello World in binary!\n";
    println!("{:?}", bytearray1);
    io::stdout().write_all(bytearray1)?;

    let bytearray2: &[u8] = b"Hello World in binary!\n";
    println!("{:?}", bytearray2);
    io::stdout().write_all(bytearray2)?;

    Ok(())
}
