mod types;

use self::types::{RegexBlock, RegexError};

#[derive(Debug, Clone)]
pub struct Regex {
    _ref: &'static [u8],
    inner: RegexBlock,
    anchor_start: bool,
    anchor_end: bool,
}

impl Regex {
    pub fn from_str(s: &str) -> Result<Regex, RegexError> {
        let _ref: &'static [u8] = s.as_bytes().to_vec().leak();
        let anchor_start = _ref.starts_with(b"^");
        let start = anchor_start as usize;
        let anchor_end = _ref.ends_with(b"$");
        let end = {
            if anchor_end {
                _ref.len() - 1
            } else {
                _ref.len()
            }
        };
        let inner = RegexBlock::from_bytes(&_ref[start..end])?.0;
        Ok(Self {
            _ref,
            inner,
            anchor_start,
            anchor_end,
        })
    }

    /// Returns the substring that matched the expression if there was one.
    pub fn match_str<'a>(&self, heystack: &'a str) -> Option<&'a str> {
        if !self.anchor_start {
            for s in 0..heystack.len() {
                if let Some(m) = self.inner.matches(&heystack[s..], self.anchor_end) {
                    return Some(m.0);
                }
            }
        } else {
            return self.inner.matches(&heystack, self.anchor_end).map(|s| s.0);
        }
        None
    }

    /// Returns the substring that matched the expression if there was and the rest.
    pub fn match_str_with_rest<'a>(&self, heystack: &'a str) -> Option<(&'a str, &'a str)> {
        if !self.anchor_start {
            for s in 0..heystack.len() {
                if let Some(m) = self.inner.matches(&heystack[s..], self.anchor_end) {
                    return Some(m);
                }
            }
        } else {
            return self.inner.matches(&heystack, self.anchor_end);
        }
        None
    }

    pub fn iter<'a>(&'a self, heystack: &'a str) -> RegexIterator {
        RegexIterator {
            regex: self,
            rest: heystack,
        }
    }
}

pub struct RegexIterator<'a> {
    regex: &'a Regex,
    rest: &'a str,
}

impl<'a> Iterator for RegexIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        match self.regex.match_str_with_rest(self.rest) {
            Some((m, r)) => {
                self.rest = r;
                Some(m)
            }
            None => None,
        }
    }
}
