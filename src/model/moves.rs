use core::marker::PhantomData;

use super::{board::Square, Board, Player, Position, Symbol};

pub(crate) struct MovesData {
    pub symbol: Symbol,
    pub totem_old_pos: Position,
    pub totem_new_pos: Position,
    pub piece_pos: Position,
}

pub struct Moves {
    symbol: Symbol,
    totem_old_pos: Position,
    totem_new_pos: Position,
    piece_pos: Position,
}

impl Moves {
    /// Creates a builder that build a valid move on the given board for the given player.
    pub fn builder<'b, 'p>(
        board: &'b Board,
        player: &'p Player,
    ) -> MovesBuilder<'b, 'p, MovesBuilderStateInit> {
        MovesBuilder {
            board,
            player,
            data: MovesBuilderStateData::None,
            _marker: PhantomData,
        }
    }

    pub fn into_data(self) -> MovesData {
        let Self {
            symbol,
            totem_old_pos,
            totem_new_pos,
            piece_pos,
        } = self;
        MovesData {
            symbol,
            totem_old_pos,
            totem_new_pos,
            piece_pos,
        }
    }
}

#[derive(Debug)]
pub enum MovesBuilderError {
    TotemNotFound,
    OutOfBoard,
    NoPieceLeft,
    InvalidTotemMove,
    InvalidPiecePlacement,
}

pub enum MovesBuilderStateData {
    None,
    Symbol {
        symbol: Symbol,
        totem_old_pos: Position,
    },
    TotemPos {
        symbol: Symbol,
        totem_old_pos: Position,
        totem_new_pos: Position,
    },
}
pub struct MovesBuilderStateInit;
pub struct MovesBuilderStateSymbol;
pub struct MovesBuilderStateTotemPos;
mod sealed {
    pub trait MoveBuilderState {}
}
impl sealed::MoveBuilderState for MovesBuilderStateInit {}
impl sealed::MoveBuilderState for MovesBuilderStateSymbol {}
impl sealed::MoveBuilderState for MovesBuilderStateTotemPos {}
pub struct MovesBuilder<'b, 'p, T: sealed::MoveBuilderState + 'static> {
    board: &'b Board,
    player: &'p Player,
    data: MovesBuilderStateData,
    _marker: PhantomData<&'static mut T>,
}
pub type MovesBuilderInit<'b, 'p> = MovesBuilder<'b, 'p, MovesBuilderStateInit>;

impl<'b, 'p> MovesBuilder<'b, 'p, MovesBuilderStateInit> {
    pub fn symbol(
        self,
        symbol: Symbol,
    ) -> Result<MovesBuilder<'b, 'p, MovesBuilderStateSymbol>, MovesBuilderError> {
        let Self { board, player, .. } = self;
        if !board.has_left_piece(symbol, player.color()) {
            return Err(MovesBuilderError::NoPieceLeft);
        }
        let totem_old_pos = board
            .find(Square::Totem(symbol))
            .ok_or(MovesBuilderError::TotemNotFound)?;

        Ok(MovesBuilder {
            board,
            player,
            data: MovesBuilderStateData::Symbol {
                symbol,
                totem_old_pos,
            },
            _marker: PhantomData,
        })
    }
}

impl<'b, 'p> MovesBuilder<'b, 'p, MovesBuilderStateSymbol> {
    pub fn totem_pos(
        self,
        totem_new_pos: impl TryInto<Position>,
    ) -> Result<MovesBuilder<'b, 'p, MovesBuilderStateTotemPos>, MovesBuilderError> {
        let totem_new_pos = totem_new_pos
            .try_into()
            .map_err(|_| MovesBuilderError::OutOfBoard)?;
        let Self {
            board,
            player,
            data:
                MovesBuilderStateData::Symbol {
                    symbol,
                    totem_old_pos,
                },
            ..
        } = self
        else {
            unreachable!();
        };

        if board
            .totem_valid_moves(totem_old_pos)
            .all(|p| p != totem_new_pos)
        {
            return Err(MovesBuilderError::InvalidTotemMove);
        }

        Ok(MovesBuilder {
            board,
            player,
            data: MovesBuilderStateData::TotemPos {
                symbol,
                totem_old_pos,
                totem_new_pos,
            },
            _marker: PhantomData,
        })
    }
}

impl<'b, 'p> MovesBuilder<'b, 'p, MovesBuilderStateTotemPos> {
    pub fn piece_pos(self, piece_pos: impl TryInto<Position>) -> Result<Moves, MovesBuilderError> {
        let piece_pos = piece_pos
            .try_into()
            .map_err(|_| MovesBuilderError::OutOfBoard)?;
        let Self {
            board,
            data:
                MovesBuilderStateData::TotemPos {
                    symbol,
                    totem_old_pos,
                    totem_new_pos,
                },
            ..
        } = self
        else {
            unreachable!();
        };

        if board
            .piece_valid_moves(totem_new_pos, symbol)
            .all(|p| p != piece_pos)
        {
            return Err(MovesBuilderError::InvalidPiecePlacement);
        }

        Ok(Moves {
            symbol,
            totem_old_pos,
            totem_new_pos,
            piece_pos,
        })
    }
}
