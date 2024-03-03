use std::{
    error::Error,
    io::{BufRead, BufWriter, Write},
    thread,
};

use crate::library::{
    io::{create_io, Scanner},
    strings::prefix_function::Matcher,
};

pub fn solve<I: BufRead, O: Write>(
    mut input: Scanner<I>,
    mut output: BufWriter<O>,
) -> Result<(), Box<dyn Error>> {
    let n = input.usize();
    let mut a = vec![vec![]; n];
    for i in 0..n {
        a[i] = input.bytes();
    }

    // how many characters of j can be skipped if we place i first
    let mut matchers = vec![vec![None; n]; n];
    let mut suffix_common = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            // a_j is pattern, a_i is text
            let m = Matcher::new(&a[j], &a[i]);
            if m.matches().next().is_some() {
                suffix_common[i][j] = a[j].len();
                matchers[i][j] = Some(m);
                continue;
            }

            suffix_common[i][j] = *m.pi().last().unwrap();
            matchers[i][j] = Some(m);
        }
    }

    let mut useful = vec![];
    for i in 0..n {
        let mut all = true;
        for j in 0..n {
            if i == j {
                continue;
            }
            if a[i] == a[j] && i < j {
                continue;
            }
            if let Some(m) = &matchers[j][i] {
                all &= m.matches().next().is_none();
            }
        }
        if all {
            useful.push(i);
        }
    }

    let m = useful.len();

    let mut dp = vec![vec![usize::MAX; m]; 1 << m];
    for i in 0..m {
        dp[1 << i][i] = a[useful[i]].len();
    }
    // dbg!(&useful);
    // dbg!(&suffix_common[1][2], &suffix_common[2][1]);
    for msk in 1..(1 << m) {
        for i in 0..m {
            if (msk & (1 << i)) == 0 {
                continue;
            }
            for j in 0..m {
                if (msk & (1 << j)) != 0 {
                    continue;
                }
                let nmsk = msk | (1 << j);

                dp[nmsk][j] = dp[nmsk][j]
                    .min(dp[msk][i] + a[useful[j]].len() - suffix_common[useful[i]][useful[j]]);
            }
        }
    }

    let ans = dp[(1 << m) - 1].iter().min().unwrap();
    writeln!(output, "{}", ans).unwrap();

    Ok(())
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
