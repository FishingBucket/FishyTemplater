use std::{fmt::Display, ops::Add};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    pub start: usize,
    pub end: usize
}

impl Span {
    pub fn new(start: usize, end: usize) -> Span {
        Span {
            start, end
        }
    }
}

impl Add for Span {
    type Output = Span;

    fn add(self, rhs: Self) -> Self::Output {
        Span {
            start: self.start.min(rhs.start),
            end: self.end.max(rhs.end)
        }
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0}:{1}", self.start, self.end)
    }
}