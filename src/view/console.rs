use crate::model::{Game, GameState};

pub struct View;
impl View {
    pub fn display_game(&self, game: &Game) {
        let game_view = GameView::new(game);
        println!("{}", game_view)
    }

    pub fn display_prompt_symbol(&self) {
        println!("Enter symbol (o/x) ");
    }
    pub fn display_prompt_totem(&self) {
        println!("Enter new totem pos (x,y) ");
    }
    pub fn display_prompt_piece(&self) {
        println!("Enter piece pos (x,y) ");
    }

    pub fn display_error<E: core::fmt::Debug>(&self, err: E) {
        println!("Error: {err:?}");
    }
}

pub struct GameView<'g> {
    game: &'g Game,
}

impl<'g> core::fmt::Display for GameView<'g> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "======= Oxono game =======")?;
        write!(f, "State: ")?;
        match self.game.state() {
            GameState::Started => writeln!(f, "Started")?,
            GameState::PinkWins => writeln!(f, "Over. Pink wins.")?,
            GameState::BlackWins => writeln!(f, "Over. Black wins.")?,
            GameState::Draw => writeln!(f, "Over. Nobody wins.")?,
        }
        writeln!(
            f,
            "Current player: {:?}",
            self.game.current_player().color()
        )?;
        writeln!(f, "\n{}", self.game.board())?;
        writeln!(f, "==========================")?;

        Ok(())
    }
}

impl<'g> GameView<'g> {
    pub fn new(game: &'g Game) -> Self {
        Self { game }
    }
}
