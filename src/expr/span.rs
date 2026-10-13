use std::ops::Add;

use derive_more::Display;

#[derive(Display, Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
#[display("{start}:{end}")]
pub struct Span {
    start: usize,
    end: usize,
}

impl Add for Span {
    type Output = Span;

    fn add(self, rhs: Self) -> Self::Output {
        Span {
            start: self.start.min(rhs.start),
            end: self.end.max(rhs.end),
        }
    }
}

impl Span {
    pub fn new(start: usize, end: usize) -> Span {
        Span { start, end }
    }

    pub fn slice<'a>(self, string: &'a String) -> &'a str {
        &string[self.start..self.end]
    }
}
