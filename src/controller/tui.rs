use {
    crate::{
        model::{Game, MoveApplyError, MovesBuilderError, Position, Square, Symbol},
        view,
    },
    crossterm::event::{self},
    ratatui::DefaultTerminal,
    std::io,
};

pub enum UserMessage {
    Quit,
    Select,
    Right,
    Up,
    Left,
    Down,
    None,
}

pub enum MoveError {
    MoveBuilderError(MovesBuilderError),
    MoveApplyError(MoveApplyError),
}

#[derive(Default)]
pub struct UIState {
    selected_pos: Option<Position>,
    selected_symbol: Option<Symbol>,
    selected_totem_pos: Option<Position>,
    error: Option<MoveError>,
}
impl UIState {
    pub fn reset_selection(&mut self) {
        let _ = self.selected_pos.take();
        let _ = self.selected_symbol.take();
        let _ = self.selected_totem_pos.take();
    }
    pub fn selected_pos(&self) -> Option<Position> {
        self.selected_pos
    }
    pub fn selected_symbol(&self) -> Option<Symbol> {
        self.selected_symbol
    }
    pub fn selected_totem_pos(&self) -> Option<Position> {
        self.selected_totem_pos
    }
    pub fn error_msg(&self) -> Option<String> {
        self.error.as_ref().map(|e| match e {
            MoveError::MoveBuilderError(error) => format!("{error:?}"),
            MoveError::MoveApplyError(error) => format!("{error:?}"),
        })
    }
}

#[derive(Default)]
pub struct Controller {
    game: Game,
    ui: UIState,
}
impl Controller {
    pub fn new() -> Self {
        Self::default()
    }
    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        loop {
            terminal.draw(|frame| view::draw(&self.game, &self.ui, frame))?;

            match self.handle_events()? {
                UserMessage::Quit => break,
                UserMessage::Select => {
                    if self.ui.error.take().is_some() {
                        continue;
                    }
                    let Some(pos) = self.ui.selected_pos else {
                        self.ui.selected_pos = Some(Position::new(0, 0));
                        continue;
                    };
                    let builder_1 = self.game.moves_builder();
                    if self.ui.selected_symbol.is_none() {
                        if let Square::Totem(symb) = self.game.board().get(pos) {
                            if let Err(err) = builder_1.symbol(*symb) {
                                self.ui.error = Some(MoveError::MoveBuilderError(err));
                            } else {
                                self.ui.selected_symbol = Some(*symb);
                            }
                        }
                        continue;
                    }
                    let builder_2 = builder_1.symbol(self.ui.selected_symbol.unwrap()).unwrap();
                    if self.ui.selected_totem_pos.is_none() {
                        if let Err(err) = builder_2.totem_pos(pos) {
                            self.ui.error = Some(MoveError::MoveBuilderError(err));
                        } else {
                            self.ui.selected_totem_pos = Some(pos);
                        }
                        continue;
                    }
                    let builder_3 = builder_2
                        .totem_pos(self.ui.selected_totem_pos.unwrap())
                        .unwrap();
                    match builder_3.piece_pos(pos) {
                        Err(err) => {
                            self.ui.error = Some(MoveError::MoveBuilderError(err));
                            continue;
                        }
                        Ok(moves) => {
                            if let Err(err) = self.game.play(moves) {
                                self.ui.error = Some(MoveError::MoveApplyError(err));
                            }
                            self.ui.reset_selection();
                        }
                    }
                }
                UserMessage::Right => {
                    if let Some(pos) = self.ui.selected_pos.as_mut() {
                        if let Some(new) = pos.right() {
                            *pos = new;
                        }
                    } else {
                        self.ui.selected_pos = Some(Position::new(0, 0));
                    }
                }
                UserMessage::Up => {
                    if let Some(pos) = self.ui.selected_pos.as_mut() {
                        if let Some(new) = pos.up() {
                            *pos = new;
                        }
                    } else {
                        self.ui.selected_pos = Some(Position::new(0, 0));
                    }
                }
                UserMessage::Left => {
                    if let Some(pos) = self.ui.selected_pos.as_mut() {
                        if let Some(new) = pos.left() {
                            *pos = new;
                        }
                    } else {
                        self.ui.selected_pos = Some(Position::new(0, 0));
                    }
                }
                UserMessage::Down => {
                    if let Some(pos) = self.ui.selected_pos.as_mut() {
                        if let Some(new) = pos.down() {
                            *pos = new;
                        }
                    } else {
                        self.ui.selected_pos = Some(Position::new(0, 0));
                    }
                }
                UserMessage::None => {}
            }
        }

        Ok(())
    }
    fn handle_events(&self) -> io::Result<UserMessage> {
        match event::read()? {
            event::Event::Key(key) if key.kind == event::KeyEventKind::Press => match key.code {
                event::KeyCode::Char('q') => Ok(UserMessage::Quit),
                event::KeyCode::Enter if !self.game.state().is_over() => Ok(UserMessage::Select),
                event::KeyCode::Right if !self.game.state().is_over() => Ok(UserMessage::Right),
                event::KeyCode::Up if !self.game.state().is_over() => Ok(UserMessage::Up),
                event::KeyCode::Left if !self.game.state().is_over() => Ok(UserMessage::Left),
                event::KeyCode::Down if !self.game.state().is_over() => Ok(UserMessage::Down),
                _ => Ok(UserMessage::None),
            },
            _ => Ok(UserMessage::None),
        }
    }
    pub fn start(&mut self) {
        let mut terminal = ratatui::init();

        if let Err(err) = self.run(&mut terminal) {
            eprintln!("{err:#?}");
        }

        ratatui::restore();
    }
}
