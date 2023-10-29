use std::{
    error::Error,
    io::{BufRead, BufWriter, Write},
    thread,
};

use crate::library::io::{create_io, Scanner};

pub fn solve<I: BufRead, O: Write>(
    mut input: Scanner<I>,
    mut output: BufWriter<O>,
) -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    // 200MB
    const STACK_SIZE: usize = 200 * 1024 * 1024;
    thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(|| {
            let (input, output) = create_io();
            solve(input, output).unwrap()
        })
        .unwrap()
        .join()
        .unwrap();
    Ok(())
}
