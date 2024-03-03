use std::{
    collections::BinaryHeap,
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
        let (n, m) = (input.usize(), input.usize());
        let c = input.vec::<i64>(n);
        let mut a = vec![vec![]; n];
        for i in 0..n {
            a[i] = input.vec::<i64>(m);
        }

        let mut dist = vec![vec![10i64.pow(18); m]; 2 * n];
        let mut adj = vec![vec![vec![]; m]; 2 * n];

        for k in 0..m {
            let mut ord = (0..n).collect::<Vec<_>>();
            ord.sort_by_key(|&x| a[x][k]);
            let mut distinct = vec![];
            let mut i = 0;
            while i < n {
                let j = i;
                while i < n && a[ord[i]][k] == a[ord[j]][k] {
                    i += 1;
                }
                distinct.push(a[ord[j]][k]);
            }

            for j in 0..(distinct.len() - 1) {
                adj[n + j][k].push((n + j + 1, k, distinct[j + 1] - distinct[j]));
                adj[n + j + 1][k].push((n + j, k, 0));
            }

            i = 0;
            for idx in ord {
                while distinct[i] != a[idx][k] {
                    i += 1;
                }
                adj[idx][k].push((n + i, k, c[idx]));
                adj[n + i][k].push((idx, k, 0));
                if k + 1 < m {
                    adj[idx][k].push((idx, k + 1, 0));
                    adj[idx][k + 1].push((idx, k, 0));
                }
            }
        }

        let mut heap = BinaryHeap::new();
        dist[n - 1][0] = 0;
        heap.push((0, n - 1, 0));

        while let Some((d, u, k)) = heap.pop() {
            if d != dist[u][k] {
                continue;
            }
            for &(v, nk, w) in &adj[u][k] {
                if dist[v][nk] > dist[u][k] + w {
                    dist[v][nk] = dist[u][k] + w;
                    heap.push((dist[v][nk], v, nk));
                }
            }
        }

        let ans = dist[0].iter().min().unwrap();
        writeln!(output, "{ans}").unwrap();
    }

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

