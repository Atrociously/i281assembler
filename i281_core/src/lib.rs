use std::{iter::Peekable, str::Chars};

#[derive(Clone, Debug)]
pub struct TokenIter<I: Iterator<Item = char>> {
    iter: Peekable<I>,
    unique: fn(char) -> bool,
    skip: fn(char) -> bool,
    skip_after: char,

    peeked: Option<Option<String>>,

    col: usize,
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

            col: 0,
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

    /// Will create a new iterator over the next token
    /// the returned iterator is guaranteed to only ever return Some once.
    /// It will return None if [`peek`] returns None
    pub fn peek_to_iter_one(&mut self) -> Option<TokenIter<Chars>> {
        let unique = self.unique.clone();
        let skip_after = self.skip_after.clone();
        let skip = self.skip.clone();
        
        self.peek().map(str::chars).map(|c| TokenIter::new_with(c.clone(), unique, skip_after, skip))
    }

    pub fn line_number(&self) -> usize {
        self.lines_encountered + 1
    }

    pub fn column(&self) -> usize {
        self.col
    }

    // use this to iterate over the chars on the internal iterator
    // it keeps track of some informational state
    fn iter_next(&mut self) -> Option<char> {
        self.col += 1;
        let next = self.iter.next();
        if next == Some('\n') {
            self.lines_encountered += 1;
            self.col = 0;
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

    #[test]
    fn test_peek_to_iter_one() {
        let chars = "a b, c".chars();
        let mut iter = TokenIter::new(chars);

        let peeked = iter.peek_to_iter_one();
        let mut peek_c = peeked.unwrap();
        assert!(peek_c.next() == Some("a".to_owned()));
        assert!(peek_c.next() == None);
        assert!(iter.peek() == Some("a"));
        assert!(iter.next() == Some("a".to_owned()));

    }
}
