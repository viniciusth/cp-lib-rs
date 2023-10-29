use std::{
    error::Error,
    io::{BufRead, BufWriter, Write},
};

use crate::library::io::Scanner;

// checks if two segments have intersection
pub fn solve<I: BufRead, O: Write>(
    mut input: Scanner<I>,
    mut output: BufWriter<O>,
) -> Result<(), Box<dyn Error + 'static>> {
    let _t = input.token::<usize>();
    let n = input.token::<usize>();
    let mut cnt = 0;
    let mut arr = vec![];
    for _ in 0..n {
        let x = input.token::<i64>();
        arr.push(x);
    }
    // dbg!(n);
    for i in 0..n {
        for j in i + 1..n {
            let mut bad = false;
            for k in 0..n {
                bad |= (arr[i] % arr[k]) == 0 && (arr[j] % arr[k]) == 0;
                if bad {
                    // dbg!(i, j, k, arr[i], arr[j], arr[k]);
                    break;
                }
            }
            if !bad {
                cnt += 1;
            }
        }
    }
    writeln!(output, "{cnt}")?;
    Ok(())
}
