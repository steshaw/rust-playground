#!/usr/bin/env cargo-eval
//! ```cargo
//! [dependencies]
//! anyhow = "1.0.87"
//! encoding_rs = "0.8.34"
//! ```
//!

//
// Experimenting with Michael Snoyman's tweet about a quick Window's 1255 decoder.
// https://x.com/snoyberg/status/1829165444500259015/
//

use std::io::{BufRead, BufReader};

fn main() -> anyhow::Result<()> {
    let stdin = std::io::stdin().lock();
    let stdin = BufReader::new(stdin);
    for line in stdin.lines() {
        let line = line?;
        let line = line.chars()
            .map(u8::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        let (line,_ ,_) = encoding_rs::WINDOWS_1255.decode(&line);
        println!("{line}");
    }
    Ok(())
}
