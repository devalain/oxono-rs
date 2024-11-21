use super::{moves::MovesData, Color, Moves, Player, Position, Symbol};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Square {
    Totem(Symbol),
    Piece(Symbol, Color),
    Empty,
}
impl core::fmt::Display for Square {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Square::Totem(Symbol::O) => write!(f, "TO"),
            Square::Totem(Symbol::X) => write!(f, "TX"),
            Square::Piece(Symbol::O, Color::Pink) => write!(f, "PO"),
            Square::Piece(Symbol::O, Color::Black) => write!(f, "BO"),
            Square::Piece(Symbol::X, Color::Pink) => write!(f, "PX"),
            Square::Piece(Symbol::X, Color::Black) => write!(f, "BX"),
            Square::Empty => write!(f, "  "),
        }
    }
}
impl Square {
    pub fn is_empty(&self) -> bool {
        matches!(self, &Square::Empty)
    }
}

#[derive(Clone, Copy, Debug)]
struct PiecesCount(i32);
impl Default for PiecesCount {
    fn default() -> Self {
        Self(8)
    }
}
impl PiecesCount {
    pub fn get(&self) -> i32 {
        self.0
    }
    pub fn take(&mut self, symbol: Symbol, color: Color) -> Option<Square> {
        if let Self(n @ 1..=8) = self {
            *n -= 1;
            Some(Square::Piece(symbol, color))
        } else {
            None
        }
    }
}

#[derive(Debug, Default)]
pub struct Pieces {
    x_pink: PiecesCount,
    x_black: PiecesCount,
    o_pink: PiecesCount,
    o_black: PiecesCount,
}
impl Pieces {
    pub fn take(&mut self, symbol: Symbol, color: Color) -> Option<Square> {
        match (symbol, color) {
            (Symbol::X, Color::Pink) => self.x_pink.take(symbol, color),
            (Symbol::X, Color::Black) => self.x_black.take(symbol, color),
            (Symbol::O, Color::Pink) => self.o_pink.take(symbol, color),
            (Symbol::O, Color::Black) => self.o_black.take(symbol, color),
        }
    }
    pub fn has_left(&self, symbol: Symbol, color: Color) -> bool {
        !match (symbol, color) {
            (Symbol::X, Color::Pink) => matches!(self.x_pink, PiecesCount(0)),
            (Symbol::X, Color::Black) => matches!(self.x_black, PiecesCount(0)),
            (Symbol::O, Color::Pink) => matches!(self.o_pink, PiecesCount(0)),
            (Symbol::O, Color::Black) => matches!(self.o_black, PiecesCount(0)),
        }
    }
    pub fn get(&self, symbol: Symbol, color: Color) -> i32 {
        match (symbol, color) {
            (Symbol::X, Color::Pink) => self.x_pink.get(),
            (Symbol::X, Color::Black) => self.x_black.get(),
            (Symbol::O, Color::Pink) => self.o_pink.get(),
            (Symbol::O, Color::Black) => self.o_black.get(),
        }
    }
}

#[derive(Debug)]
pub enum MoveApplyError {
    WrongTotemPosition,
    NotEmpty,
    NoPieceLeft,
}

#[derive(Debug)]
pub enum TotemStatus {
    FullyEnclave,
    Enclave,
    Free,
}

#[derive(Debug)]
pub struct Board {
    squares: [Square; 36],
    pieces: Pieces,
}
impl Default for Board {
    fn default() -> Self {
        let mut new = Self {
            squares: [Square::Empty; 36],
            pieces: Pieces::default(),
        };
        new.squares[14] = Square::Totem(Symbol::X);
        new.squares[21] = Square::Totem(Symbol::O);
        new
    }
}
impl core::fmt::Display for Board {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(
            f,
            "Pink's pieces: X: {}, O: {}\n\n",
            self.pieces.x_pink.get(),
            self.pieces.o_pink.get()
        )?;
        writeln!(f, "    0  1  2  3  4  5")?;
        writeln!(f, "  +--+--+--+--+--+--+")?;
        for y in 0..6 {
            write!(f, "{} |", y)?;
            for x in 0..6 {
                write!(f, "{}|", self.get(Position::new(x, y)))?;
            }
            writeln!(f, "\n  +--+--+--+--+--+--+")?;
        }
        write!(
            f,
            "\n\nBlack's pieces: X: {}, O: {}",
            self.pieces.x_black.get(),
            self.pieces.o_black.get()
        )?;

        Ok(())
    }
}

impl Board {
    fn pos_to_inner(pos: Position) -> usize {
        pos.x() + pos.y() * 6
    }
    fn inner_to_pos(inner: usize) -> Option<Position> {
        let x = (inner % 6) as u8;
        let y = (inner / 6) as u8;
        (x, y).try_into().ok()
    }
    fn swap(&mut self, pos1: Position, pos2: Position) {
        let i1 = Self::pos_to_inner(pos1);
        let i2 = Self::pos_to_inner(pos2);
        let (i_min, i_max) = if i1 < i2 { (i1, i2) } else { (i2, i1) };
        if let [x, .., y] = &mut self.squares[i_min..=i_max] {
            core::mem::swap(x, y);
        }
    }
    fn place(&mut self, pos: Position, symbol: Symbol, color: Color) -> Result<(), MoveApplyError> {
        let i = Self::pos_to_inner(pos);
        if let Square::Empty = self.squares[i] {
            if let Some(piece) = self.pieces.take(symbol, color) {
                self.squares[i] = piece;
                Ok(())
            } else {
                Err(MoveApplyError::NoPieceLeft)
            }
        } else {
            Err(MoveApplyError::NotEmpty)
        }
    }

    pub fn pieces(&self) -> &Pieces {
        &self.pieces
    }
    pub fn has_left_piece(&self, symbol: Symbol, color: Color) -> bool {
        self.pieces.has_left(symbol, color)
    }
    pub fn get(&self, pos: Position) -> &Square {
        &self.squares[Self::pos_to_inner(pos)]
    }
    pub fn find(&self, square: Square) -> Option<Position> {
        let inner = self
            .squares
            .iter()
            .enumerate()
            .find_map(|(i, s)| (s == &square).then_some(i))?;
        Self::inner_to_pos(inner)
    }
    pub fn totem_status(&self, pos: Position) -> TotemStatus {
        let right = pos.right();
        let up = pos.up();
        let left = pos.left();
        let down = pos.down();
        let is_enclave = [right, up, left, down]
            .iter()
            .filter_map(|p| p.filter(|pp| self.get(*pp).is_empty()))
            .count()
            == 0;

        if !is_enclave {
            TotemStatus::Free
        } else {
            let next_empty_right = pos.iter_right().skip(1).find(|p| self.get(*p).is_empty());
            let next_empty_up = pos.iter_up().skip(1).find(|p| self.get(*p).is_empty());
            let next_empty_left = pos.iter_left().skip(1).find(|p| self.get(*p).is_empty());
            let next_empty_down = pos.iter_down().skip(1).find(|p| self.get(*p).is_empty());
            let is_fully_enclave = [
                next_empty_right,
                next_empty_up,
                next_empty_left,
                next_empty_down,
            ]
            .iter()
            .all(Option::is_none);
            if is_fully_enclave {
                TotemStatus::FullyEnclave
            } else {
                TotemStatus::Enclave
            }
        }
    }
    pub fn totem_valid_moves(&self, pos: Position) -> impl Iterator<Item = Position> + '_ {
        struct TotemValidMoves<'b>(&'b Board, Position);
        enum TotemValidMovesIter<'b> {
            EmptySquares(&'b Board, usize),
            LatteralFirstEmptySquares(&'b Board, Position, usize),
            EmptyNeighbours(&'b Board, Position, Position, usize),
        }
        impl<'b> Iterator for TotemValidMovesIter<'b> {
            type Item = Position;
            fn next(&mut self) -> Option<Self::Item> {
                match self {
                    TotemValidMovesIter::EmptySquares(board, i) => loop {
                        let pos = Board::inner_to_pos(*i)?;
                        *i += 1;
                        if board.get(pos).is_empty() {
                            break Some(pos);
                        }
                    },
                    TotemValidMovesIter::LatteralFirstEmptySquares(board, pos, i) => loop {
                        let next = match *i {
                            0 => pos.iter_right().find(|s| board.get(*s).is_empty()),
                            1 => pos.iter_up().find(|s| board.get(*s).is_empty()),
                            2 => pos.iter_left().find(|s| board.get(*s).is_empty()),
                            3 => pos.iter_down().find(|s| board.get(*s).is_empty()),
                            _ => break None,
                        };
                        *i += 1;
                        if next.is_some() {
                            break next;
                        }
                    },
                    TotemValidMovesIter::EmptyNeighbours(board, pos, curr, i) => loop {
                        let next = {
                            let next = match *i {
                                0 => curr.right(),
                                1 => curr.up(),
                                2 => curr.left(),
                                3 => curr.down(),
                                _ => break None,
                            };
                            if next.map(|p| board.get(p).is_empty()).unwrap_or_default() {
                                *curr = next.unwrap();
                            } else {
                                *i += 1;
                                *curr = *pos;
                                continue;
                            }
                            Some(*curr)
                        };
                        if next.is_some() {
                            break next;
                        }
                    },
                }
            }
        }
        impl<'b> IntoIterator for TotemValidMoves<'b> {
            type IntoIter = TotemValidMovesIter<'b>;
            type Item = Position;
            fn into_iter(self) -> Self::IntoIter {
                let Self(board, pos) = self;
                match board.totem_status(pos) {
                    TotemStatus::FullyEnclave => TotemValidMovesIter::EmptySquares(board, 0),
                    TotemStatus::Enclave => {
                        TotemValidMovesIter::LatteralFirstEmptySquares(board, pos, 0)
                    }
                    TotemStatus::Free => TotemValidMovesIter::EmptyNeighbours(board, pos, pos, 0),
                }
            }
        }
        TotemValidMoves(self, pos).into_iter()
    }
    pub fn piece_valid_moves(
        &self,
        totem_pos: Position,
        symbol: Symbol,
    ) -> impl Iterator<Item = Position> + '_ {
        enum Eiter<A, B> {
            A(A),
            B(B),
        }
        impl<A: Iterator<Item = Position>, B: Iterator<Item = Position>> Iterator for Eiter<A, B> {
            type Item = Position;
            fn next(&mut self) -> Option<Self::Item> {
                match self {
                    Eiter::A(a) => a.next(),
                    Eiter::B(b) => b.next(),
                }
            }
        }
        let right = totem_pos.right();
        let up = totem_pos.up();
        let left = totem_pos.left();
        let down = totem_pos.down();
        let actual_totem_pos = self.find(Square::Totem(symbol)).unwrap();
        let usual_case = [right, up, left, down].into_iter().filter_map(move |p| {
            p.filter(|pp| {
                let sq = self.get(*pp);
                sq.is_empty() || pp == &actual_totem_pos
            })
        });
        let empty = self
            .squares
            .iter()
            .copied()
            .enumerate()
            .filter_map(|(i, s)| s.is_empty().then_some(i).and_then(Self::inner_to_pos));
        if usual_case.clone().count() == 0 {
            Eiter::A(
                empty
                    .chain(core::iter::once(actual_totem_pos))
                    .filter(move |p| *p != totem_pos),
            )
        } else {
            Eiter::B(usual_case)
        }
    }
    pub fn wins(&self, four: [Position; 4]) -> bool {
        let squares = four.iter().map(|p| self.get(*p));
        if !squares.clone().all(|s| matches!(s, &Square::Piece(_, _))) {
            return false;
        }
        let Square::Piece(symbol, color) = squares.clone().next().unwrap() else {
            unreachable!()
        };
        let mut same_symbol = true;
        let mut same_color = true;
        for sq in squares {
            let Square::Piece(s, c) = sq else {
                unreachable!()
            };
            if s != symbol {
                same_symbol = false;
            }
            if c != color {
                same_color = false;
            }
            if !same_color && !same_symbol {
                return false;
            }
        }
        true
    }
    pub fn no_more_pieces(&self) -> bool {
        !self.pieces.has_left(Symbol::O, Color::Pink)
            && !self.pieces.has_left(Symbol::X, Color::Pink)
            && !self.pieces.has_left(Symbol::O, Color::Black)
            && !self.pieces.has_left(Symbol::X, Color::Black)
    }

    pub fn apply(&mut self, moves: Moves, player: &Player) -> Result<(), MoveApplyError> {
        let MovesData {
            symbol,
            totem_old_pos,
            totem_new_pos,
            piece_pos,
        } = moves.into_data();

        // Try to move the totem
        if self.get(totem_old_pos) != &Square::Totem(symbol) {
            return Err(MoveApplyError::WrongTotemPosition);
        }
        self.swap(totem_old_pos, totem_new_pos);

        // Try to place the piece
        self.place(piece_pos, symbol, player.color())?;

        Ok(())
    }
}
