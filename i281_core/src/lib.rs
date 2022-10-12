use std::iter::Peekable;

#[derive(Clone, Debug)]
pub struct TokenIter<I: Iterator<Item = char>> {
    iter: Peekable<I>,
    unique: fn(char) -> bool,
    skip: fn(char) -> bool,
    skip_after: char,

    peeked: Option<Option<String>>,

    pos: usize,
    lines_encountered: usize,
}

impl<I: Iterator<Item = char>> TokenIter<I> {
    pub fn new_with(
        iter: I,
        unique: fn(char) -> bool,
        skip_after: char,
        skip: fn(char) -> bool,
    ) -> Self {
        Self {
            iter: iter.peekable(),
            unique,
            skip,
            skip_after,
            peeked: None,

            pos: 0,
            lines_encountered: 0,
        }
    }

    pub fn new(iter: I) -> Self {
        Self::new_with(
            iter,
            |c| c.is_ascii_punctuation(),
            ';',
            |c| c.is_control() || c.is_whitespace(),
        )
    }

    pub fn peek(&mut self) -> Option<&str> {
        let peeked = match self.peeked {
            Some(ref p) => p.as_ref(),
            None => {
                self.peeked = Some(self.next());
                self.peeked.as_ref().unwrap().as_ref()
            }
        };
        peeked.map(String::as_str)
    }

    pub fn line_number(&self) -> usize {
        self.lines_encountered + 1
    }

    pub fn position(&self) -> usize {
        self.pos
    }

    fn iter_next(&mut self) -> Option<char> {
        self.pos += 1;
        let next = self.iter.next();
        if next == Some('\n') {
            self.lines_encountered += 1
        }
        next
    }
}

impl<I: Iterator<Item = char>> Iterator for TokenIter<I> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        // if we peeked into the iterator return the peeked token
        // otherwise we need to get the next token from the stream
        match self.peeked.take() {
            Some(v) => return v,
            None => {}
        }

        match self.iter.next() {
            Some(mut char) => {
                // skip specified skip chars
                while (self.skip)(char) {
                    char = self.iter_next()?;
                }
                if char == self.skip_after {
                    // if we hit a skip after char
                    // skip until we reach a new line
                    while char != '\n' {
                        char = self.iter_next()?;
                    }
                    return self.next();
                }
                // if we hit a unique character return it
                if (self.unique)(char) {
                    return Some(char.to_string());
                } else {
                    // collect chars until we hit a unique or skip char
                    let mut s = String::new();
                    while !((self.skip)(char) || (self.unique)(char)) {
                        s.push(char);
                        char = match self.iter.peek() {
                            Some(c) => {
                                if !((self.skip)(*c) || (self.unique)(*c)) {
                                    // safe because peek returned some
                                    self.iter_next().unwrap()
                                } else {
                                    *c
                                }
                            }
                            // if we reach the end while creating a token string we return it
                            None => return Some(s),
                        };
                    }
                    Some(s)
                }
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::TokenIter;

    #[test]
    fn test_peek() {
        let chars = "a b, c".chars();
        let mut iter = TokenIter::new(chars);

        assert!(iter.peek() == Some("a"));
        assert!(iter.next() == Some("a".to_owned()));
        assert!(iter.peek() == Some("b"));
        assert!(iter.peek() == Some("b"));
        assert!(iter.next() == Some("b".to_owned()));
    }
}
