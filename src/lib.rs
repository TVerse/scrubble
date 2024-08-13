use anyhow::Result;
use board::Board;

pub mod bitboard;
mod board;
pub struct TileMap {
    // TODO score
    letters: Vec<String>,
}

impl TileMap {
    pub fn new(letters: Vec<String>) -> Result<Self> {
        if letters.len() > u8::MAX as usize {
            anyhow::bail!("Max number of distinct tiles: {}", (u8::MAX as usize) + 1);
        }
        Ok(Self { letters })
    }

    pub fn get(&self, idx: TileMapIdx) -> Option<&String> {
        self.letters.get(idx.0 as usize)
    }

    pub fn find(&self, needle: &str) -> Option<TileMapIdx> {
        self.letters
            .iter()
            .position(|s| s == needle)
            .map(|idx| TileMapIdx(idx as u8))
    }

    fn len(&self) -> u8 {
        self.letters.len() as u8
    }

    pub fn english() -> Self {
        Self::new(('A'..='Z').into_iter().map(|c| c.to_string()).collect())
            .expect("The English alphabet has less than 256 letters")
    }
}

pub struct TileMapIdx(u8);

pub struct Game {
    board: Board,
    tiles: TileMap,
}

impl Game {
    pub fn new(tiles: TileMap) -> Self {
        Self {
            board: Board::new(tiles.len()),
            tiles,
        }
    }
}
