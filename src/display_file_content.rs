use std::{
    io::{self, BufRead, Write},
    path::Path,
};

use crate::read_file;
use anyhow::Result;

pub fn lines<P: AsRef<Path>>(path: P) -> Result<()> {
    let buf = read_file::read_file(path)?;

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for (idx, line) in buf.lines().enumerate() {
        let line = line?;
        writeln!(handle, "{:>6}  {}", idx + 1, line)?;
    }

    Ok(())
}
