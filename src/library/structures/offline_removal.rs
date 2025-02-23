use std::{collections::{hash_map, HashMap}, hash::Hash};

/// Example usages:
/// https://codeforces.com/edu/course/2/lesson/7/3/practice/contest/289392/submission/307348927 (personal)
/// https://codeforces.com/contest/2069/submission/307350861
///
/// Complexity proof outline and explanation in my study repo.

/// Solves the offline removal problem in O(q log q log n)
/// All queries will be solved in order.
pub struct OfflineRemoval<S, So, T, U, O> {
    queries: Vec<DCQuery<T, U>>,
    ranges: Vec<(usize, usize)>,
    ans: Vec<O>,
    structure: S,
    solver: So,
}

#[derive(Debug)]
pub enum DCQuery<T, U> {
    /// Toggles the the type T in the structure: adds if it doesn't exist, removes if it does.
    /// This value will need to implement Hash, be mindful to add consistent values in relation to
    /// hash as it will define if we add or remove something. (u, v) != (v, u).
    Toggle(T),
    Query(U),
}

pub trait DCQuerySolver<S, T, U, O> {
    /// Solves a query where the structure has the correct operations toggled.
    /// Can only use read functions from structure, but solver state can be mutated if needed.
    fn solve(&mut self, structure: &S, query: &DCQuery<T, U>) -> O;
}

pub trait AddRollbackStructure<T> {
    fn add(&mut self, value: T);
    fn rollback(&mut self, quantity: usize);
}

impl<T: Hash + Eq + Clone, U, O, S: AddRollbackStructure<T>, So: DCQuerySolver<S, T, U, O>> OfflineRemoval<S, So, T, U, O> {
    pub fn new(solver: So, structure: S, queries: Vec<DCQuery<T, U>>) -> Self {
        let ranges = Self::preprocess(&queries);
        Self {
            ans: Vec::with_capacity(queries.len()),
            queries,
            ranges,
            structure,
            solver,
        }
    }

    /// Computes the range that each operation is going to remain active for.
    fn preprocess(queries: &[DCQuery<T, U>]) -> Vec<(usize, usize)> {
        let mut ranges = Vec::with_capacity(queries.len()/2);
        let mut open = HashMap::<T, usize>::with_capacity(queries.len()/2);
        for (r, q) in queries.iter().enumerate() {
            match q {
                DCQuery::Toggle(t) => {
                    match open.entry(t.clone()) {
                        hash_map::Entry::Occupied(e) => {
                            let l = e.remove();
                            ranges.push((l, r - 1));
                        },
                        hash_map::Entry::Vacant(e) => {
                            e.insert(r);
                        },
                    }
                }
                _ => (),
            };
        }

        ranges.extend(open.into_values().map(|x| (x, queries.len())));

        ranges
    }

    pub fn solve(mut self) -> Vec<O> {
        if self.queries.len() == 0 {
            return self.ans;
        }
        let idxs: Vec<_> = (0..self.ranges.len()).collect();
        self.f(0, self.queries.len() - 1, &idxs);
        self.ans
    }

    fn f(&mut self, l: usize, r: usize, ranges: &[usize]) {
        let mut new_ranges = Vec::new();
        let mut adds = 0;
        for &i in ranges {
            let (ql, qr) = self.ranges[i];
            if ql <= l && r <= qr {
                let DCQuery::Toggle(q) = &self.queries[ql] else {
                    panic!("ql should always be a toggle query");
                };
                self.structure.add(q.clone());
                adds += 1;
            } else if !(qr < l || r < ql) {
                new_ranges.push(i);
            }
        }

        if l == r {
            let output = self.solver.solve(&self.structure, &self.queries[l]);
            self.ans.push(output);
            self.structure.rollback(adds);
            return;
        }

        let mid = (l + r) / 2;
        self.f(l, mid, &new_ranges);
        self.f(mid+1, r, &new_ranges);
        self.structure.rollback(adds);
    }
}
