#[derive(Clone, Copy, Debug)]
pub struct SkipWhitespace<I: Iterator> {
    iter: I,
}

impl<I: Iterator> SkipWhitespace<I> {
    pub fn new(iter: I) -> SkipWhitespace<I> {
        SkipWhitespace { iter }
    }

    pub fn with_whitespace(self) -> I {
        return self.iter;
    }
}

impl<I> Iterator for SkipWhitespace<I>
where
    I: Iterator<Item = char>,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        match self.iter.next() {
            Some(mut next) => {
                while next.is_whitespace() {
                    next = match self.iter.next() {
                        Some(n) => n,
                        None => return None,
                    }
                }
                return Some(next);
            },
            None => None,
        }
    }
}
