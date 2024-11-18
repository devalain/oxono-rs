use super::Color;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Player {
    color: Color,
}

impl Player {
    fn pink() -> Self {
        Self { color: Color::Pink }
    }
    fn black() -> Self {
        Self {
            color: Color::Black,
        }
    }

    pub fn color(&self) -> Color {
        self.color
    }
}

#[derive(Debug)]
pub struct Players {
    players: [Player; 2],
}
impl Default for Players {
    fn default() -> Self {
        Self {
            players: [Player::pink(), Player::black()],
        }
    }
}
impl Players {
    pub fn current(&self) -> &Player {
        &self.players[0]
    }
    pub fn turn(&mut self) {
        let [a, b] = &mut self.players;
        core::mem::swap(a, b);
    }
}
