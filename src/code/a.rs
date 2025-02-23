use std::{
    error::Error,
    io::{BufRead, BufWriter, Write},
    thread,
};

use crate::library::{
    io::{create_io, Scanner},
    structures::{
        dsu::RollbackDSU,
        offline_removal::{AddRollbackStructure, DCQuery, DCQuerySolver, OfflineRemoval},
    },
};

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
enum Graph {
    A,
    B,
}

type Tg = (Graph, usize, usize);
type Dcq = DCQuery<Tg, ()>;

struct DoubleGraphDSU {
    dsu_a: RollbackDSU,
    dsu_ab: RollbackDSU,
    ops: Vec<(Graph, i32)>,
    contradictions: usize,
}

impl AddRollbackStructure<Tg> for DoubleGraphDSU {
    fn add(&mut self, (g, x, y): Tg) {
        assert!(x <= y, "edge should be consistent");
        // println!("Adding {x} {y} to {g:?}");
        let mut delta = 0;
        match g {
            Graph::A => {
                let merged_a = self.dsu_a.merge(x, y);
                let merged_ab = self.dsu_ab.merge(x, y);
                if merged_a && !merged_ab {
                    self.contradictions -= 1;
                    delta += 1;
                }
            }
            Graph::B => {
                if self.dsu_ab.merge(x, y) {
                    self.contradictions += 1;
                    delta -= 1;
                }
            }
        };
        self.ops.push((g, delta));
    }

    fn rollback(&mut self, quantity: usize) {
        // println!("Rolling back {quantity}");
        assert!(quantity <= self.ops.len(), "too many rollbacks");
        let mut a = 0;
        let mut ab = 0;
        let mut total_delta = 0;
        for _ in 0..quantity {
            let op = self.ops.pop().unwrap();
            match op.0 {
                Graph::A => { a += 1; ab += 1; }
                Graph::B => ab += 1,
            }
            total_delta += op.1;
        }
        self.dsu_a.rollback(a);
        self.dsu_ab.rollback(ab);
        self.contradictions = (self.contradictions as i32 + total_delta) as usize;
    }
}

impl DoubleGraphDSU {
    fn new(n: usize) -> Self {
        Self {
            dsu_a: RollbackDSU::new(n),
            dsu_ab: RollbackDSU::new(n),
            ops: Vec::new(),
            contradictions: 0,
        }
    }
}

pub fn solve<I: BufRead, O: Write>(
    mut input: Scanner<I>,
    mut output: BufWriter<O>,
) -> Result<(), Box<dyn Error>> {
    // let t = input.usize();

    let (n, q) = (input.usize(), input.usize());
    let mut queries = Vec::with_capacity(q);
    for _ in 0..q {
        let op = input.string();
        let (mut x, mut y) = (input.usize(), input.usize());
        if x > y {
            std::mem::swap(&mut x, &mut y);
        }
        let qry = match op.as_str() {
            "A" => Dcq::Toggle((Graph::A, x - 1, y - 1)),
            "B" => Dcq::Toggle((Graph::B, x - 1, y - 1)),
            _ => unreachable!("invalid input"),
        };
        queries.push(qry);
    }

    struct Solver {}

    impl DCQuerySolver<DoubleGraphDSU, Tg, (), usize> for Solver {
        fn solve(&mut self, dsu: &DoubleGraphDSU, query: &Dcq) -> usize {
            // println!("Solving");
            match query {
                Dcq::Toggle(_) => dsu.contradictions,
                _ => unreachable!("impossible"),
            }
        }
    }

    let runner = OfflineRemoval::new(Solver {}, DoubleGraphDSU::new(n), queries);
    let ans = runner.solve();

    ans.into_iter().for_each(|x| {
        writeln!(output, "{x}").unwrap();
    });

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
