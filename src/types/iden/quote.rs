//! The characters an identifier is quoted with.

/// The opening and closing characters that quote an identifier: `"` on both sides in
/// standard SQL, a backtick in MySQL.
///
/// Each is one byte, so a `char` given to a `From` conversion must be ASCII; anything
/// wider is cut to its low byte.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Quote(pub(crate) u8, pub(crate) u8);

impl Quote {
    /// The same character on both sides.
    pub fn new(c: u8) -> Self {
        Self(c, c)
    }

    /// The opening character.
    pub fn left(&self) -> char {
        char::from(self.0)
    }

    /// The closing character.
    pub fn right(&self) -> char {
        char::from(self.1)
    }
}

impl From<char> for Quote {
    fn from(c: char) -> Self {
        Self::new(c as u8)
    }
}

impl From<(char, char)> for Quote {
    fn from((l, r): (char, char)) -> Self {
        Self(l as u8, r as u8)
    }
}

impl From<u8> for Quote {
    fn from(c: u8) -> Self {
        Self::new(c)
    }
}

impl From<(u8, u8)> for Quote {
    fn from((l, r): (u8, u8)) -> Self {
        Self(l, r)
    }
}
