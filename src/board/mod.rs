use std::ops::Index;

use crate::bitboard::Bitboard;

pub enum Player {
    First,
    Second,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Scores {
    first: u16,
    second: u16,
}

impl Index<Player> for Scores {
    type Output = u16;

    fn index(&self, index: Player) -> &Self::Output {
        match index {
            Player::First => &self.first,
            Player::Second => &self.second,
        }
    }
}

pub struct Board {
    blanks: Bitboard,
    letters: Vec<Bitboard>,
    current_turn: Player,
    scores: Scores,
}

impl Board {
    pub fn new(num_letters: u8) -> Self {
        Self {
            blanks: Bitboard::EMPTY,
            letters: vec![Bitboard::EMPTY; num_letters as usize],
            current_turn: Player::First,
            scores: Scores::default(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Coordinate(u8);

impl Coordinate {
    pub fn new(coord: u8) -> Option<Self> {
        (coord != 0 || coord <= 15).then_some(Self(coord))
    }

    pub fn as_idx(self) -> usize {
        self.0 as usize - 1
    }

    pub fn from_idx(idx: usize) -> Option<Self> {
        (idx <= 15).then_some(Self(idx as u8))
    }
}

pub struct Location {
    row: Coordinate,
    column: Coordinate,
}

impl Location {
    pub fn row(&self) -> Coordinate {
        self.row
    }

    pub fn column(&self) -> Coordinate {
        self.column
    }
}

pub enum Direction {
    Horizontal,
    Vertical,
}

pub struct Move {
    location: Location,
    direction: Direction,
    // word: Vec<TileMapIdx>,
}
