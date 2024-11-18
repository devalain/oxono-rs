#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Position {
    x: u8,
    y: u8,
}

impl TryFrom<(u8, u8)> for Position {
    type Error = (u8, u8);

    fn try_from((x, y): (u8, u8)) -> Result<Self, Self::Error> {
        if x > 5 || y > 5 {
            Err((x, y))
        } else {
            Ok(Position { x, y })
        }
    }
}

impl Position {
    pub fn new(x: u8, y: u8) -> Self {
        if x > 5 || y > 5 {
            panic!("Invalid position: ({x}, {y})");
        }
        Self { x, y }
    }
    pub fn x(&self) -> usize {
        self.x as usize
    }
    pub fn y(&self) -> usize {
        self.y as usize
    }

    pub fn right(&self) -> Option<Position> {
        if self.x == 5 {
            None
        } else {
            Some(Self {
                x: self.x + 1,
                y: self.y,
            })
        }
    }
    pub fn up(&self) -> Option<Position> {
        if self.y == 0 {
            None
        } else {
            Some(Self {
                x: self.x,
                y: self.y - 1,
            })
        }
    }
    pub fn left(&self) -> Option<Position> {
        if self.x == 0 {
            None
        } else {
            Some(Self {
                x: self.x - 1,
                y: self.y,
            })
        }
    }
    pub fn down(&self) -> Option<Position> {
        if self.y == 5 {
            None
        } else {
            Some(Self {
                x: self.x,
                y: self.y + 1,
            })
        }
    }

    pub fn iter_right(&self) -> impl Iterator<Item = Position> + 'static {
        let mut next = Some(*self);
        core::iter::from_fn(move || {
            next = next?.right();
            next
        })
    }
    pub fn iter_up(&self) -> impl Iterator<Item = Position> + 'static {
        let mut next = Some(*self);
        core::iter::from_fn(move || {
            next = next?.up();
            next
        })
    }
    pub fn iter_left(&self) -> impl Iterator<Item = Position> + 'static {
        let mut next = Some(*self);
        core::iter::from_fn(move || {
            next = next?.left();
            next
        })
    }
    pub fn iter_down(&self) -> impl Iterator<Item = Position> + 'static {
        let mut next = Some(*self);
        core::iter::from_fn(move || {
            next = next?.down();
            next
        })
    }

    pub fn four_latteral_groups() -> impl Iterator<Item = [Position; 4]> {
        let mut i = 0;
        core::iter::from_fn(move || {
            let n = i / 3; // row num
            let f = i % 3; // 3 fours in a row
            i += 1;
            let mut array = [Position::new(0, 0); 4];
            match n {
                0..6 => {
                    for (i, p) in Position::new(n, f).iter_down().take(4).enumerate() {
                        array[i] = p;
                    }
                    Some(array)
                }
                6..12 => {
                    for (i, p) in Position::new(f, n - 6).iter_right().take(4).enumerate() {
                        array[i] = p;
                    }
                    Some(array)
                }
                _ => None,
            }
        })
    }
}
