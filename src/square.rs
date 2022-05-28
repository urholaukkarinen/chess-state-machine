use std::{convert::TryFrom, fmt::Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Square {
    pub x: u8,
    pub y: u8,
}

impl From<(u8, u8)> for Square {
    fn from(tuple: (u8, u8)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

impl From<[u8; 2]> for Square {
    fn from(pair: [u8; 2]) -> Self {
        Self {
            x: pair[0],
            y: pair[1],
        }
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            char::from_u32('a' as u32 + self.x as u32).unwrap(),
            self.y + 1
        )
    }
}

impl TryFrom<&str> for Square {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != 2 {
            return Err(());
        }

        let mut square = Square::default();

        for c in value.chars() {
            match c.to_ascii_lowercase() {
                'a' => square.x = 0,
                'b' => square.x = 1,
                'c' => square.x = 2,
                'd' => square.x = 3,
                'e' => square.x = 4,
                'f' => square.x = 5,
                'g' => square.x = 6,
                'h' => square.x = 7,

                '1' => square.y = 0,
                '2' => square.y = 1,
                '3' => square.y = 2,
                '4' => square.y = 3,
                '5' => square.y = 4,
                '6' => square.y = 5,
                '7' => square.y = 6,
                '8' => square.y = 7,
                _ => return Err(()),
            };
        }

        Ok(square)
    }
}
