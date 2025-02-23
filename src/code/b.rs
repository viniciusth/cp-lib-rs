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
    let t = input.usize();
    for _ in 0..t {
        let n = input.i64();
        if has(n) {
            writeln!(output, "0").unwrap();
            continue;
        }

        let mut cur = 9;
        let mut ans = 10;
        for _ in 1..=9 {
            for j in 1..=10 {
                if has(cur * j + n) {
                    ans = ans.min(j);
                }
            }
            cur = cur * 10 + 9;
        }
        writeln!(output, "{}", ans).unwrap();
    }

    Ok(())
}

fn has(mut n: i64) -> bool {
    while n > 0 {
        if n % 10 == 7 {
            return true;
        }
        n /= 10;
    }
    return false;
}

pub fn main() -> Result<(), Box<dyn Error>> {
    // 1GB
    const STACK_SIZE: usize = 1024 * 1024 * 1024;
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

