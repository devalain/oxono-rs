#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Symbol {
    X,
    O,
}

impl Symbol {
    pub fn opposite(self) -> Self {
        match self {
            Symbol::X => Symbol::O,
            Symbol::O => Symbol::X,
        }
    }
}
