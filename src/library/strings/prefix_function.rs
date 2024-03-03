
pub fn prefix_function(s: &[u8]) -> Vec<usize> {
    let n = s.len();
    let mut pi = vec![0; n];
    for i in 1..n {
        let mut j = pi[i - 1];
        while j > 0 && s[i] != s[j] {
            j = pi[j - 1];
        }
        if s[i] == s[j] {
            j += 1;
        }
        pi[i] = j;
    }
    pi
}

#[derive(Debug, Clone)]
pub struct Matcher {
    pi: Vec<usize>,
    pattern_len: usize,
}

impl Matcher {
    pub fn new(pattern: &[u8], text: &[u8]) -> Self {
        let pi = prefix_function(&[pattern, b"$", text].concat());
        Self {
            pi,
            pattern_len: pattern.len(),
        }
    }

    /// Returns the indices of the text where the pattern starts.
    pub fn matches(&self) -> impl Iterator<Item = usize> + '_ {
        self.pi[self.pattern_len + 1..].iter().enumerate().filter_map(move |(i, &x)| {
            if x == self.pattern_len {
                Some(i + 1 - self.pattern_len)
            } else {
                None
            }
        })
    }

    /// Returns the amount of characters matched for each prefix of the text.
    pub fn pi(&self) -> &[usize] {
        &self.pi[self.pattern_len + 1..]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prefix_function() {
        let s = b"abacaba";
        let pi = prefix_function(s);
        assert_eq!(pi, vec![0, 0, 1, 0, 1, 2, 3]);
    }

    #[test]
    fn string_matching() {
        let matcher = Matcher::new(b"uk", b"snukesu");
        assert_eq!(matcher.matches().collect::<Vec<_>>(), vec![2]);
    }
}
