// taken from https://codeforces.com/contest/947/submission/55268770
use std::{io, str};

pub struct Scanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: str::SplitAsciiWhitespace<'static>,
}
impl<R: io::BufRead> Scanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: Vec::new(),
            buf_iter: "".split_ascii_whitespace(),
        }
    }
    pub fn token<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse");
            }
            self.buf_str.clear();
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed read");
            self.buf_iter = unsafe {
                let slice = str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_ascii_whitespace())
            }
        }
    }
    pub fn i32(&mut self) -> i32 {
        self.token()
    }

    pub fn i64(&mut self) -> i64 {
        self.token()
    }

    pub fn i128(&mut self) -> i128 {
        self.token()
    }

    pub fn usize(&mut self) -> usize {
        self.token()
    }

    pub fn vec<T>(&mut self, n: usize) -> Vec<T>
    where
        T: str::FromStr,
    {
        (0..n).map(|_| self.token()).collect()
    }

    pub fn i64_pair_array(&mut self, n: usize) -> Vec<(i64, i64)> {
        (0..n).map(|_| (self.i64(), self.i64())).collect()
    }

    pub fn string(&mut self) -> String {
        self.token::<String>()
    }

    pub fn bytes(&mut self) -> Vec<u8> {
        self.token::<String>().into_bytes()
    }
}

pub fn create_io() -> (
    Scanner<io::StdinLock<'static>>,
    io::BufWriter<io::StdoutLock<'static>>,
) {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    (
        Scanner::new(stdin.lock()),
        io::BufWriter::new(stdout.lock()),
    )
}

pub fn create_fio(
    input_path: &str,
    output_path: &str,
) -> (
    Scanner<io::BufReader<std::fs::File>>,
    io::BufWriter<io::BufWriter<std::fs::File>>,
) {
    let (input, output) = (
        std::fs::File::open(input_path).expect("Failed open file"),
        std::fs::File::create(output_path).expect("Failed create file"),
    );
    (
        Scanner::new(io::BufReader::new(input)),
        io::BufWriter::new(io::BufWriter::new(output)),
    )
}
