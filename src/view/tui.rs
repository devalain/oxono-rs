use {
    crate::{
        controller::UIState,
        model::{Color, Game, Position, Square, Symbol},
    },
    ratatui::{
        buffer::Buffer,
        layout::{Constraint, Direction, Flex, Layout, Rect},
        style::{self, Stylize},
        symbols::{self, border},
        text::Line,
        widgets::{
            canvas::{self, Canvas},
            Block, BorderType, Clear, Paragraph, Widget,
        },
        Frame,
    },
};

const PINK: style::Color = style::Color::Rgb(0xFF, 0x1F, 0x8F);

pub fn draw(game: &Game, ui: &UIState, frame: &mut Frame) {
    frame.render_widget(View(game, ui), frame.area());
    if let Some(msg) = ui.error_msg() {
        let block = Block::bordered().title("Error");
        let paragraph = Paragraph::new(msg).centered().block(block);
        let area = popup_area(frame.area(), 40, 20);
        frame.render_widget(Clear, area); //this clears out the background
        frame.render_widget(paragraph, area);
    }
}

fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}

pub struct View<'g, 'ui>(&'g Game, &'ui UIState);
impl<'g, 'ui> Widget for View<'g, 'ui> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let title = Line::from(" Oxono ".bold());
        let instructions = Line::from(vec![
            " Quit ".into(),
            "<Q> ".blue().bold(),
            " Move selection ".into(),
            "<arrows> ".blue().bold(),
            " Select ".into(),
            "<Enter> ".blue().bold(),
        ]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let side = area.width.min(area.height);
        let min_board_side = side;
        let min_player_height = min_board_side / 6;

        let h_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Fill(1),
                Constraint::Length(min_board_side),
                Constraint::Fill(1),
            ])
            .split(block.inner(area));
        let v_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Fill(1),
                Constraint::Max(min_player_height),
                Constraint::Length(min_board_side / 2),
                Constraint::Max(min_player_height),
                Constraint::Fill(1),
            ])
            .split(h_layout[1]);

        block.render(area, buf);
        Paragraph::new(Line::from(vec![match self.0.state() {
            crate::model::GameState::Started => "Game started".bold(),
            crate::model::GameState::PinkWins => {
                "Game is over: Pink wins.".bold().green().slow_blink()
            }
            crate::model::GameState::BlackWins => {
                "Game is over: Black wins.".bold().green().slow_blink()
            }
            crate::model::GameState::Draw => {
                "Game is over: Nobody wins.".bold().green().slow_blink()
            }
        }]))
        .centered()
        .render(v_layout[0], buf);
        PinkPiecesView(self.0).render(v_layout[1], buf);
        BoardView(self.0, self.1).render(v_layout[2], buf);
        BlackPiecesView(self.0).render(v_layout[3], buf);
    }
}

struct PinkPiecesView<'g>(&'g Game);
impl<'g> Widget for PinkPiecesView<'g> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let x_pieces = self.0.board().pieces().get(Symbol::X, Color::Pink);
        let o_pieces = self.0.board().pieces().get(Symbol::O, Color::Pink);
        let block = if self.0.current_player().color() == Color::Pink {
            Block::bordered().border_type(BorderType::Rounded)
        } else {
            Block::bordered().black()
        };
        Paragraph::new(Line::from(vec![
            "Pink ".fg(PINK).bold(),
            format!("[X {x_pieces}]").black().bg(PINK),
            " ".into(),
            format!("[O {o_pieces}]").black().bg(PINK),
        ]))
        .centered()
        .block(block)
        .render(area, buf);
    }
}
struct BoardView<'g, 'ui>(&'g Game, &'ui UIState);
impl<'g, 'ui> Widget for BoardView<'g, 'ui> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let board = self.0.board();
        let selected_symbol = self.1.selected_symbol();
        let selected_totem_pos = self.1.selected_totem_pos();
        Canvas::default()
            .marker(symbols::Marker::HalfBlock)
            .x_bounds([0.0, 600.0])
            .y_bounds([0.0, 600.0])
            .paint(|ctx| {
                for y in 0..6 {
                    for x in 0..6 {
                        let cx = x as f64 * 100.0;
                        let cy = (5 - y) as f64 * 100.0;
                        let pos = Position::new(x, y);

                        ctx.draw(&canvas::Rectangle {
                            x: cx,
                            y: cy,
                            width: 100.0,
                            height: 100.0,
                            color: style::Color::White,
                        });

                        if Some(pos) == selected_totem_pos {
                            let cx = pos.x() as f64 * 100.0;
                            let cy = (5 - pos.y()) as f64 * 100.0;
                            match selected_symbol {
                                Some(Symbol::O) => {
                                    ctx.draw(&canvas::Circle {
                                        x: cx + 50.0,
                                        y: cy + 50.0,
                                        radius: 40.0,
                                        color: style::Color::Blue,
                                    });
                                }
                                Some(Symbol::X) => {
                                    ctx.draw(&canvas::Line {
                                        x1: cx + 20.0,
                                        y1: cy + 20.0,
                                        x2: cx + 80.0,
                                        y2: cy + 80.0,
                                        color: style::Color::Blue,
                                    });
                                    ctx.draw(&canvas::Line {
                                        x1: cx + 20.0,
                                        y1: cy + 80.0,
                                        x2: cx + 80.0,
                                        y2: cy + 20.0,
                                        color: style::Color::Blue,
                                    });
                                }
                                None => {}
                            }
                        }
                        match board.get(pos) {
                            Square::Totem(symbol)
                                if Some(symbol) != selected_symbol.as_ref()
                                    || selected_symbol.is_none()
                                    || selected_totem_pos.is_none() =>
                            {
                                match symbol {
                                    Symbol::O => {
                                        ctx.draw(&canvas::Circle {
                                            x: cx + 50.0,
                                            y: cy + 50.0,
                                            radius: 40.0,
                                            color: style::Color::Blue,
                                        });
                                    }
                                    Symbol::X => {
                                        ctx.draw(&canvas::Line {
                                            x1: cx + 20.0,
                                            y1: cy + 20.0,
                                            x2: cx + 80.0,
                                            y2: cy + 80.0,
                                            color: style::Color::Blue,
                                        });
                                        ctx.draw(&canvas::Line {
                                            x1: cx + 20.0,
                                            y1: cy + 80.0,
                                            x2: cx + 80.0,
                                            y2: cy + 20.0,
                                            color: style::Color::Blue,
                                        });
                                    }
                                }
                            }
                            Square::Piece(symbol, color) => {
                                let color = match color {
                                    Color::Pink => PINK,
                                    Color::Black => style::Color::White,
                                };
                                match symbol {
                                    Symbol::O => {
                                        ctx.draw(&canvas::Circle {
                                            x: cx + 50.0,
                                            y: cy + 50.0,
                                            radius: 40.0,
                                            color,
                                        });
                                    }
                                    Symbol::X => {
                                        ctx.draw(&canvas::Line {
                                            x1: cx + 20.0,
                                            y1: cy + 20.0,
                                            x2: cx + 80.0,
                                            y2: cy + 80.0,
                                            color,
                                        });
                                        ctx.draw(&canvas::Line {
                                            x1: cx + 20.0,
                                            y1: cy + 80.0,
                                            x2: cx + 80.0,
                                            y2: cy + 20.0,
                                            color,
                                        });
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
                if let Some(s) = selected_symbol {
                    if let Some(p) = self.0.board().find(Square::Totem(s)) {
                        if selected_totem_pos.is_none() {
                            ctx.draw(&canvas::Rectangle {
                                x: p.x() as f64 * 100.0,
                                y: (5 - p.y()) as f64 * 100.0,
                                width: 100.0,
                                height: 100.0,
                                color: style::Color::Blue,
                            });
                            for p in self.0.board().totem_valid_moves(p) {
                                ctx.draw(&canvas::Rectangle {
                                    x: p.x() as f64 * 100.0,
                                    y: (5 - p.y()) as f64 * 100.0,
                                    width: 100.0,
                                    height: 100.0,
                                    color: style::Color::Green,
                                });
                            }
                        }
                    }
                }
                if let Some(p) = selected_totem_pos {
                    for p in self
                        .0
                        .board()
                        .piece_valid_moves(p, selected_symbol.unwrap())
                    {
                        ctx.draw(&canvas::Rectangle {
                            x: p.x() as f64 * 100.0,
                            y: (5 - p.y()) as f64 * 100.0,
                            width: 100.0,
                            height: 100.0,
                            color: style::Color::Green,
                        });
                    }
                }

                if let Some(p) = self.1.selected_pos() {
                    let color = if self.0.current_player().color() == Color::Pink {
                        PINK
                    } else {
                        style::Color::Black
                    };
                    ctx.draw(&canvas::Rectangle {
                        x: p.x() as f64 * 100.0,
                        y: (5 - p.y()) as f64 * 100.0,
                        width: 100.0,
                        height: 100.0,
                        color,
                    });
                }
            })
            .render(area, buf);
    }
}
struct BlackPiecesView<'g>(&'g Game);
impl<'g> Widget for BlackPiecesView<'g> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let x_pieces = self.0.board().pieces().get(Symbol::X, Color::Black);
        let o_pieces = self.0.board().pieces().get(Symbol::O, Color::Black);
        let block = if self.0.current_player().color() == Color::Black {
            Block::bordered().border_type(BorderType::Rounded)
        } else {
            Block::bordered().black()
        };
        Paragraph::new(vec![
            Line::from(vec![" ".into()]),
            Line::from(vec![
                "Black ".gray().bold(),
                format!("[X {x_pieces}]").black().on_white(),
                " ".into(),
                format!("[O {o_pieces}]").black().on_white(),
            ]),
        ])
        .centered()
        .block(block)
        .render(area, buf);
    }
}
