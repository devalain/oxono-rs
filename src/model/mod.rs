mod board;
mod color;
mod moves;
mod player;
mod position;
mod symbol;

pub use {
    board::{Board, MoveApplyError, Pieces, Square},
    color::Color,
    moves::{Moves, MovesBuilderError, MovesBuilderInit},
    player::{Player, Players},
    position::Position,
    symbol::Symbol,
};

#[derive(Debug, Default, Eq, PartialEq)]
pub enum GameState {
    #[default]
    Started,
    PinkWins,
    BlackWins,
    Draw,
}
impl GameState {
    pub fn is_over(&self) -> bool {
        !matches!(self, Self::Started)
    }
}

#[derive(Debug, Default)]
pub struct Game {
    state: GameState,
    board: Board,
    players: Players,
}
impl Game {
    fn update_state(&mut self) {
        let state_if_wins = match self.current_player().color() {
            Color::Pink => GameState::PinkWins,
            Color::Black => GameState::BlackWins,
        };
        for four in Position::four_latteral_groups() {
            if self.board.wins(four) {
                self.state = state_if_wins;
                break;
            }
        }
        if self.board.no_more_pieces() {
            self.state = GameState::Draw;
        }
    }
    pub fn state(&self) -> &GameState {
        &self.state
    }
    pub fn current_player(&self) -> &Player {
        self.players.current()
    }
    pub fn board(&self) -> &Board {
        &self.board
    }
    pub fn moves_builder(&self) -> MovesBuilderInit<'_, '_> {
        Moves::builder(&self.board, self.current_player())
    }
    pub fn play(&mut self, moves: Moves) -> Result<(), MoveApplyError> {
        if self.state.is_over() {
            panic!("Game is over")
        }
        let player = self.players.current();
        self.board.apply(moves, player)?;
        self.update_state();
        if !self.state.is_over() {
            self.players.turn();
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! valid_play {
        ($game:expr => $symbol:ident; $totem:expr; $piece:expr) => {
            $game
                .play(
                    $game
                        .moves_builder()
                        .symbol(Symbol::$symbol)
                        .unwrap()
                        .totem_pos($totem)
                        .unwrap()
                        .piece_pos($piece)
                        .unwrap(),
                )
                .unwrap()
        };
    }

    #[test]
    fn game_works() {
        let mut game = Game::default();
        assert_eq!(game.state(), &GameState::Started);
        assert_eq!(game.current_player().color(), Color::Pink);

        valid_play!(game => O; (2,3); (1,3));
        assert_eq!(game.current_player().color(), Color::Black);

        assert!(matches!(
            game.moves_builder()
                .symbol(Symbol::O)
                .unwrap()
                .totem_pos((2, 3)),
            Err(MovesBuilderError::InvalidTotemMove)
        ));
        assert_eq!(game.current_player().color(), Color::Black);

        valid_play!(game => O; (3,3); (3,2));
        assert_eq!(game.current_player().color(), Color::Pink);

        valid_play!(game => O; (2,3); (2,4));
        assert_eq!(game.current_player().color(), Color::Black);

        valid_play!(game => O; (3,3); (3,4));
        assert_eq!(game.current_player().color(), Color::Pink);

        valid_play!(game => O; (2,3); (3,3));
        assert_eq!(game.current_player().color(), Color::Black);

        valid_play!(game => O; (2,1); (3,1));
        assert_eq!(game.state(), &GameState::BlackWins);
    }
}
