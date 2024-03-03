use std::{
    error::Error,
    io::{BufRead, BufWriter, Write},
};

use crate::library::io::Scanner;

pub fn solve<I: BufRead, O: Write>(
    mut input: Scanner<I>,
    mut output: BufWriter<O>,
) -> Result<(), Box<dyn Error + 'static>> {
    let t = input.usize();
    for _ in 0..t {
        let n = input.usize();
        let s: Vec<i64> = input
            .bytes()
            .into_iter()
            .map(|x| if x == b'>' { 1 } else { -1 })
            .collect();

        let mut ans = vec![0; n];
        for i in 0..n {
            let mut t = s.clone();
            let mut idx = i as i64;
            let mut r = 0;
            while 0 <= idx && idx < n as i64 {
                let k = t[idx as usize];
                t[idx as usize] = -t[idx as usize];
                idx += k;
                r += 1;
            }
            ans[i] = r;
        }
        for i in 0..n {
            write!(output, "{} ", ans[i])?;
        }
        writeln!(output)?;
    }
    Ok(())
}
