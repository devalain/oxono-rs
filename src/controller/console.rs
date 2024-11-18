use {
    crate::{
        model::{Game, Moves, MovesBuilderError, Symbol},
        view::View,
    },
    std::io::stdin,
};

pub struct Controller {
    game: Game,
    view: View,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            game: Game::default(),
            view: View,
        }
    }

    fn prompt_moves(&mut self) -> Result<Moves, MovesBuilderError> {
        let builder = self.game.moves_builder();

        self.view.display_prompt_symbol();
        let builder = match stdin().lines().next().unwrap().as_deref() {
            Ok("O" | "o") => builder.symbol(Symbol::O)?,
            Ok("X" | "x") => builder.symbol(Symbol::X)?,
            _ => panic!("Input/Output error"),
        };

        self.view.display_prompt_totem();
        let builder = match stdin().lines().next().unwrap().as_deref() {
            Ok(line) => {
                let (x, y) = line.split_once(',').unwrap();
                builder.totem_pos((x.parse().unwrap(), y.parse().unwrap()))?
            }
            _ => panic!("Input/Output error"),
        };

        self.view.display_prompt_piece();
        let moves = match stdin().lines().next().unwrap().as_deref() {
            Ok(line) => {
                let (x, y) = line.split_once(',').unwrap();
                builder.piece_pos((x.parse().unwrap(), y.parse().unwrap()))?
            }
            _ => panic!("Input/Output error"),
        };

        Ok(moves)
    }

    pub fn start(&mut self) {
        loop {
            self.view.display_game(&self.game);
            match self.prompt_moves() {
                Ok(moves) => match self.game.play(moves) {
                    Ok(()) => {
                        if self.game.state().is_over() {
                            break;
                        }
                    }
                    Err(err) => {
                        self.view.display_error(err);
                    }
                },
                Err(err) => {
                    self.view.display_error(err);
                }
            }
        }
        self.view.display_game(&self.game);
    }
}
