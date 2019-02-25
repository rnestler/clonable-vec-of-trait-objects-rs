use crate::*;

#[derive(Clone, Debug)]
pub enum Enum {
    A(A),
    B(B),
}

impl Trait for Enum {
    fn print(&self) {
        match self {
            Enum::A(a) => a.print(),
            Enum::B(b) => b.print(),
        }
    }
}

