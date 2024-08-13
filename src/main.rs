use anyhow::Result;
use scrubble::bitboard::Bitboard;

fn main() -> Result<()> {
    Bitboard::FULL.right_one();
    Ok(())
}
