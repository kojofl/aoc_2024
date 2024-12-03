use std::{collections::VecDeque, error::Error, fmt::Display, str};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Token {
    Char(u8),
    CharBlock(&'static [u8]),
    CharGroup(&'static [u8]),
    NegCharGroup(&'static [u8]),
    Number,
    AlphaNumeric,
    Or,
    Optional,
    Multiple,
    Exact(u8),
    Range(u8, u8),
    Any,
    Wildcard,
    Block(RegexBlock),
}

impl Token {
    fn is_special(&self) -> bool {
        match self {
            Token::Or | Token::Optional | Token::Any | Token::Multiple => true,
            _ => false,
        }
    }

    fn match_token(&self, s: &str) -> Option<usize> {
        match self {
            Token::Char(c) => {
                if s.as_bytes().first().unwrap() == c {
                    Some(1)
                } else {
                    None
                }
            }
            Token::Number => {
                if s.as_bytes().first().unwrap().is_ascii_digit() {
                    Some(1)
                } else {
                    None
                }
            }
            Token::AlphaNumeric => {
                if s.as_bytes().first().unwrap().is_ascii_alphanumeric() {
                    Some(1)
                } else {
                    None
                }
            }
            Token::CharBlock(cb) => {
                if s.as_bytes().starts_with(cb) {
                    Some(cb.len())
                } else {
                    None
                }
            }
            Token::Or => unimplemented!(),
            Token::Optional => unimplemented!(),
            Token::Any => unimplemented!(),
            Token::Multiple => unimplemented!(),
            Token::Range(_, _) => unimplemented!(),
            Token::Exact(_) => unimplemented!(),
            Token::CharGroup(cg) => {
                if cg.contains(s.as_bytes().first().unwrap()) {
                    return Some(1);
                }
                None
            }
            Token::NegCharGroup(cg) => {
                if !cg.contains(s.as_bytes().first().unwrap()) {
                    return Some(0);
                }
                None
            }
            Token::Wildcard => Some(1),
            Token::Block(ref b) => b.matches(s, false).map(|m| m.0.len()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum RegexError {
    InvalidSyntax(&'static str),
    InvalidCharacter(u8),
}

impl Display for RegexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegexError::InvalidCharacter(b) => {
                write!(f, "Invalid character {b}")
            }
            RegexError::InvalidSyntax(s) => write!(f, "{s}"),
        }
    }
}

impl Error for RegexError {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct RegexBlock {
    inner: VecDeque<Token>,
}

impl RegexBlock {
    pub fn from_bytes(mut s: &'static [u8]) -> Result<(RegexBlock, &[u8]), RegexError> {
        // This can now be a really large over estimation.
        // TODO: smarter allocation
        let mut buffer = VecDeque::with_capacity(s.len());
        let mut is_escaped = false;
        let mut i = 0;
        loop {
            match s[i] {
                b'*' => {
                    // If escaped use literal
                    if is_escaped {
                        buffer.push_back(Token::Char(b'*'));
                        is_escaped = !is_escaped;
                        continue;
                    }
                    let Some(prev) = buffer.pop_back() else {
                        return Err(RegexError::InvalidSyntax("A start operator doesn't work without a predecessor if you wish to literally match it try \\*"));
                    };
                    if buffer.back().map(|t| t.is_special()).unwrap_or(false) {
                        return Err(RegexError::InvalidSyntax("A special operator can not be use to describe another special operator"));
                    }
                    buffer.push_back(Token::Any);
                    buffer.push_back(prev);
                }
                b'+' => {
                    // If escaped use literal
                    if is_escaped {
                        buffer.push_back(Token::Char(b'+'));
                        is_escaped = !is_escaped;
                        continue;
                    }
                    let Some(prev) = buffer.pop_back() else {
                        return Err(RegexError::InvalidSyntax("A start operator doesn't work without a predecessor if you wish to literally match it try \\+"));
                    };
                    if buffer.back().map(|t| t.is_special()).unwrap_or(false) {
                        return Err(RegexError::InvalidSyntax("A special operator can not be use to describe another special operator"));
                    }
                    buffer.push_back(Token::Multiple);
                    buffer.push_back(prev);
                }
                b'|' => {
                    // If escaped use literal
                    if is_escaped {
                        buffer.push_back(Token::Char(b'|'));
                        is_escaped = !is_escaped;
                        continue;
                    }
                    let Some(prev) = buffer.pop_back() else {
                        return Err(RegexError::InvalidSyntax("A pipe operator doesn't work without a predecessor if you wish to literally match it try \\|"));
                    };
                    if buffer.back().map(|t| t.is_special()).unwrap_or(false) {
                        return Err(RegexError::InvalidSyntax("A special operator can not be use to describe another special operator"));
                    }
                    buffer.push_back(Token::Or);
                    buffer.push_back(prev);
                }
                b'?' => {
                    // If escaped use literal
                    if is_escaped {
                        buffer.push_back(Token::Char(b'?'));
                        is_escaped = !is_escaped;
                        continue;
                    }
                    let Some(prev) = buffer.pop_back() else {
                        return Err(RegexError::InvalidSyntax("A question mark operator doesn't work without a predecessor if you wish to literally match it try \\?"));
                    };
                    if buffer.back().map(|t| t.is_special()).unwrap_or(false) {
                        return Err(RegexError::InvalidSyntax("A special operator can not be use to describe another special operator"));
                    }
                    buffer.push_back(Token::Optional);
                    buffer.push_back(prev);
                }
                b'.' => {
                    // If escaped use literal
                    if is_escaped {
                        buffer.push_back(Token::Char(b'.'));
                        is_escaped = !is_escaped;
                        continue;
                    }
                    buffer.push_back(Token::Wildcard);
                }
                b'(' => {
                    if is_escaped {
                        buffer.push_back(Token::Char(b'('));
                        is_escaped = !is_escaped;
                        continue;
                    }
                    let (block, rest) = RegexBlock::from_bytes(&s[i + 1..])?;
                    s = rest;
                    buffer.push_back(Token::Block(block));
                    i = 0;
                }
                b')' => {
                    if is_escaped {
                        buffer.push_back(Token::Char(b')'));
                        is_escaped = !is_escaped;
                        continue;
                    }
                    return Ok((Self { inner: buffer }, &s[i..]));
                }
                b'[' => {
                    let mut start = i + 1;
                    let neg = {
                        if s[start] == b'^' {
                            start += 1;
                            true
                        } else {
                            false
                        }
                    };
                    let end = start
                        + s[start..]
                            .iter()
                            .position(|b| *b == b']')
                            .expect("End to a capture group");
                    if neg {
                        buffer.push_back(Token::NegCharGroup(&s[start..end]));
                    } else {
                        buffer.push_back(Token::CharGroup(&s[start..end]));
                    }
                    i = end;
                }
                b'{' => {
                    // If escaped use literal
                    if is_escaped {
                        buffer.push_back(Token::Char(b'{'));
                        is_escaped = !is_escaped;
                        continue;
                    }
                    let start = i + 1;
                    let end = start + s[start..].iter().position(|b| *b == b'}').expect("range");
                    let Some(prev) = buffer.pop_back() else {
                        return Err(RegexError::InvalidSyntax("A Range doesn't work without a predecessor if you wish to literally match it try \\?"));
                    };
                    if buffer.back().map(|t| t.is_special()).unwrap_or(false) {
                        return Err(RegexError::InvalidSyntax("A special operator can not be use to describe another special operator"));
                    }
                    let Some(index) = s[start..end].iter().position(|b| *b == b',') else {
                        buffer.push_back(Token::Exact(unsafe {
                            str::from_utf8_unchecked(&s[start..end])
                                .parse::<u8>()
                                .map_err(|_| RegexError::InvalidSyntax("Exact must be a number"))?
                        }));
                        buffer.push_back(prev);
                        i = end;
                        continue;
                    };
                    let (r_start, r_end) = &s[start..end].split_at(index);

                    buffer.push_back(Token::Range(
                        unsafe {
                            str::from_utf8_unchecked(r_start)
                                .parse::<u8>()
                                .map_err(|_| RegexError::InvalidSyntax("Range must be a number"))?
                        },
                        unsafe {
                            str::from_utf8_unchecked(&r_end[1..])
                                .parse::<u8>()
                                .map_err(|_| RegexError::InvalidSyntax("Range must be a number"))
                        }?,
                    ));
                    buffer.push_back(prev);
                    i = end;
                }
                b'\\' => {
                    if is_escaped {
                        buffer.push_back(Token::Char(b'\\'));
                        is_escaped = !is_escaped;
                    } else {
                        is_escaped = !is_escaped
                    }
                }
                c if c.is_ascii() && is_escaped => {
                    match c {
                        b'd' => buffer.push_back(Token::Number),
                        b'w' => buffer.push_back(Token::AlphaNumeric),
                        _ => {
                            return Err(RegexError::InvalidSyntax("Invalid escape"));
                        }
                    }
                    is_escaped = !is_escaped;
                }
                c if c.is_ascii() => {
                    let start = i;
                    i += 1;
                    let mut trailing_c = false;
                    while let Some(n) = s.get(i) {
                        if n.is_ascii() && !b"\\|)(*.?+".contains(n) {
                            i += 1;
                        } else {
                            if b"*?+".contains(n) {
                                trailing_c = true;
                                i -= 1;
                            }
                            break;
                        }
                    }
                    if i - start > 1 {
                        buffer.push_back(Token::CharBlock(&s[start..i]));
                        if trailing_c {
                            buffer.push_back(Token::Char(s[i]));
                            i += 1;
                        }
                    } else {
                        buffer.push_back(Token::Char(s[start]));
                        if trailing_c {
                            buffer.push_back(Token::Char(s[i]));
                            i += 1;
                        }
                    }
                    i -= 1;
                }
                c => return Err(RegexError::InvalidCharacter(c)),
            }
            i += 1;
            if i >= s.len() {
                return Ok((Self { inner: buffer }, &s[..]));
            }
        }
    }

    fn move_foreward(s: &str, step: usize) -> &str {
        &s[step..]
    }

    pub(crate) fn matches<'a>(&self, s: &'a str, anchor_end: bool) -> Option<(&'a str, &'a str)> {
        if let Some(rest) = matches_inner(self.inner.as_slices().0, s) {
            if anchor_end && !rest.is_empty() {
                return None;
            }
            let len = s.len() - rest.len();
            return Some((&s[..len], rest));
        }
        None
    }

    pub(crate) fn matches_with_rest<'a>(&self, s: &'a str) -> Option<(&'a str, &'a str)> {
        if let Some(rest) = matches_inner(self.inner.as_slices().0, s) {
            let len = s.len() - rest.len();
            return Some((&s[..len], rest));
        }
        None
    }
}

/// Matches the regex block against a string slice recursively the return value if present is the
/// not consumed part of the input string.
fn matches_inner<'a>(block: &[Token], mut s: &'a str) -> Option<&'a str> {
    let mut state = 0;
    while state < block.len() {
        if s.is_empty() && block[state] != Token::Optional {
            return None;
        }
        match block[state] {
            Token::Char(c) => {
                if s.as_bytes().first().unwrap() == &c {
                    s = &s[1..]
                } else {
                    return None;
                }
            }
            Token::Number => {
                if s.as_bytes().first().unwrap().is_ascii_digit() {
                    s = &s[1..]
                } else {
                    return None;
                }
            }
            Token::AlphaNumeric => {
                if s.as_bytes().first().unwrap().is_ascii_alphanumeric() {
                    s = &s[1..]
                } else {
                    return None;
                }
            }
            Token::CharBlock(cb) => {
                if s.as_bytes().starts_with(cb) {
                    s = RegexBlock::move_foreward(s, cb.len());
                } else {
                    return None;
                }
            }
            Token::CharGroup(cg) => {
                if cg.contains(s.as_bytes().first().unwrap()) {
                    s = &s[1..]
                } else {
                    return None;
                }
            }
            Token::NegCharGroup(cg) => {
                if cg.contains(s.as_bytes().first().unwrap()) {
                    return None;
                }
            }
            Token::Or => {
                if let Some(l) = block[state + 1].match_token(s) {
                    s = RegexBlock::move_foreward(s, l);
                } else if let Some(l) = block[state + 2].match_token(s) {
                    s = RegexBlock::move_foreward(s, l);
                } else {
                    return None;
                }
                state += 2;
            }
            Token::Optional => {
                if let Some(r) = matches_inner(&block[state + 1..], s) {
                    return Some(r);
                } else {
                    return matches_inner(&block[state + 2..], s);
                }
            }
            Token::Multiple => {
                if let Some(l) = block[state + 1].match_token(s) {
                    s = RegexBlock::move_foreward(s, l);
                } else {
                    return None;
                }
                loop {
                    if let Some(r) = matches_inner(&block[state + 2..], s) {
                        return Some(r);
                    }
                    if let Some(l) = block[state + 1].match_token(s) {
                        s = RegexBlock::move_foreward(s, l);
                    } else {
                        return None;
                    }
                }
            }
            Token::Exact(n) => {
                for _ in 0..n {
                    if let Some(l) = block[state + 1].match_token(s) {
                        s = RegexBlock::move_foreward(s, l);
                    } else {
                        return None;
                    }
                }
                state += 2;
            }
            Token::Range(start, end) => {
                for _ in 0..start {
                    if let Some(l) = block[state + 1].match_token(s) {
                        s = RegexBlock::move_foreward(s, l);
                    } else {
                        return None;
                    }
                }
                for _ in 0..end {
                    if let Some(r) = matches_inner(&block[state + 2..], s) {
                        return Some(r);
                    }
                    if let Some(l) = block[state + 1].match_token(s) {
                        s = RegexBlock::move_foreward(s, l);
                    } else {
                        return None;
                    }
                }
            }
            Token::Any => loop {
                if let Some(r) = matches_inner(&block[state + 2..], s) {
                    return Some(r);
                }
                if let Some(l) = block[state + 1].match_token(s) {
                    s = RegexBlock::move_foreward(s, l);
                } else {
                    return None;
                }
            },
            Token::Wildcard => {
                s = &s[1..];
            }
            Token::Block(ref b) => {
                if let Some(rest) = matches_inner(b.inner.as_slices().0, s) {
                    s = rest;
                } else {
                    return None;
                }
            }
        }
        state += 1;
    }
    Some(s)
}
