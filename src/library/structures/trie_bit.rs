/// Implements a bit trie with arbitrarily stored data.
/// For usage you need to implement TrieInserter to manipulate that data during insertion,
/// and TrieSearcher for iterating of the trie in meaningful ways.
/// Each node is initialized with the default value for the data.
/// Current existing implementations: [DefaultTrieInserter], [TrieFindMax], [TrieFindMin],
/// [TrieFindInequality].
pub struct TrieBit<const N: usize, T: Default> {
    trie: Vec<[usize; 2]>,
    cnt: Vec<usize>,
    data: Vec<T>,
}

/// If you need more data inside every node, implement this trait.
/// The default behavior is to do nothing. depth == 0 => leaf
pub trait TrieInserter<T> {
    fn insert(&mut self, _data: &mut T, _input: i64, _depth: usize) {}
    fn remove(&mut self, _data: &mut T, _input: i64, _depth: usize) {}
}

/// For finding meaningful information on the trie, many searches may be implemented.
/// This trait tries to guide the implementation of these searches into its smallest
/// operations. A search may modify the nodes internal state.
/// depth == 0 => leaf, bit_depth is the bit you will "insert" in the step you are about to take.
/// current_value => number that represents the path taken thus far.
pub trait TrieSearcher<T> {
    fn process(&mut self, data: &mut T, depth: usize, current_value: i64);
    fn go_left(&mut self, data: &mut T, bit_depth: usize, current_value: i64) -> bool;
    fn go_right(&mut self, data: &mut T, bit_depth: usize, current_value: i64) -> bool;
}

impl<const N: usize, T: Default> TrieBit<N, T> {
    pub fn new() -> Self {
        Self {
            trie: vec![[0; 2]; 1],
            cnt: vec![0; 1],
            // didn't use macro because rust-analyzer was saying it was unsafe lol
            data: Vec::from([T::default()]),
        }
    }

    fn can(&self, idx: usize, bit: usize) -> bool {
        self.trie[idx][bit] != 0 && self.cnt[self.trie[idx][bit]] > 0
    }

    pub fn insert(&mut self, x: i64, inserter: &mut impl TrieInserter<T>) {
        let mut v = 0;
        self.cnt[v] += 1;
        inserter.insert(&mut self.data[v], x, N);
        for i in (0..N).rev() {
            let bit = (x >> i) & 1;
            if !self.can(v, bit as usize) {
                self.trie.push([0; 2]);
                self.cnt.push(0);
                self.data.push(T::default());
                self.trie[v][bit as usize] = self.trie.len() - 1;
            }
            v = self.trie[v][bit as usize] as usize;
            self.cnt[v] += 1;
            inserter.insert(&mut self.data[v], x, i);
        }
    }

    pub fn remove(&mut self, x: i64, inserter: &mut impl TrieInserter<T>) {
        let mut v = 0;
        self.cnt[v] -= 1;
        inserter.remove(&mut self.data[v], x, N);
        for i in (0..N).rev() {
            let bit = (x >> i) & 1;
            v = self.trie[v][bit as usize] as usize;
            self.cnt[v] -= 1;
            inserter.remove(&mut self.data[v], x, i);
        }
    }

    pub fn search(&mut self, searcher: &mut impl TrieSearcher<T>) {
        self.search_impl(0, N, 0, searcher);
    }

    fn search_impl(&mut self, v: usize, depth: usize, value: i64, searcher: &mut impl TrieSearcher<T>) {
        searcher.process(&mut self.data[v], depth, value);
        if depth == 0 {
            assert!(!self.can(v, 0) && !self.can(v, 1), "leaves should have no children.");
            return;
        }

        if self.can(v, 0) && searcher.go_left(&mut self.data[v], depth - 1, value) {
            self.search_impl(self.trie[v][0], depth - 1, value << 1, searcher);
        }

        if self.can(v, 1) && searcher.go_right(&mut self.data[v], depth - 1, value) {
            self.search_impl(self.trie[v][1], depth - 1, value << 1 | 1, searcher);
        }
    }
}

pub struct DefaultTrieInserter {}
impl<T> TrieInserter<T> for DefaultTrieInserter {}

/// Searches for the minimum value of (ans ^ x) where ans is a value present in the trie.
pub struct TrieFindMin {
    x: i64,
    ans: Option<i64>,
}

impl TrieFindMin {
    pub fn new(x: i64) -> Self {
        Self { x, ans: None }
    }

    /// Consumes the searcher and returns the value without checking.
    pub fn get_unchecked(self) -> i64 {
        self.ans.unwrap()
    }
}

// impl<T> TrieSearcher<T> for TrieFindMin {
//     fn process(&mut self, _: &mut T, depth: usize, value: i64) {
//         if depth == 0 {
//             assert!(self.ans.is_none(), "we should visit exactly one leaf in N+1 steps.");
//             self.ans = Some(value);
//         }
//     }
//
//     /// If we're wanting the min, going left means leaving the bit at bit_depth of x as it is.
//     /// TODO: these go_left / right arent enough to map the problem correctly,
//     /// the issue here is that this searcher need to go left or right, but if we say no to going
//     /// left and then there is no right node we just get stuck with a invalid answer.
//     fn go_left(&mut self, _: &mut T, bit_depth: usize, value: i64) -> bool {
//
//     }
//
//     fn go_right(&mut self, data: &mut T, bit_depth: usize, value: i64) -> bool {
//         todo!()
//     }
// }
