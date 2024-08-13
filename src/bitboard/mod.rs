use std::{
    fmt::Debug,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
};

use crate::board::Location;

/// Bitboard for the scrabble board.
///
/// Convention:
///
/// * Row-major storage. Bitboard\[0\] is row 1. Bitboard\[14\] is row 15. Bitboard\[16\] is unused and always zero.
/// * Bit 0 is column 1, bit 14 is column 15. Bit 15 is unused and always zero. Example: if Bitboard\[0\] == 0x0001, the leftmost column is set.
/// * Keeping index 16 around preps for a hypothetical AVX2 implementation if it's not optimized to that already.
///
/// So, a (smaller) bitboard like this:
/// ```text
/// [
///     0b0001,
///     0b1000,
///     0b0000,
///     0b0100,
/// ]
/// ```
///
/// represents Scrabble board
/// ```text
/// ..X.
/// ....
/// ...X
/// ..X.
///
/// ```
///
/// A bitboard with column 1 set is represented by \[0x0001; 15\] plus a zero byte.
#[derive(Clone, Copy)]
pub struct Bitboard {
    rows: [u16; 16],
}

impl Bitboard {
    #[cfg(target_endian = "big")]
    compile_error!("Only little-endian targets are supported");

    const ROW_MAX: u16 = 0x7FFF;

    pub const EMPTY: Self = Self { rows: [0; 16] };
    pub const FULL: Self = Self {
        rows: [
            Self::ROW_MAX,
            Self::ROW_MAX,
            Self::ROW_MAX,
            Self::ROW_MAX,
            Self::ROW_MAX,
            Self::ROW_MAX,
            Self::ROW_MAX,
            Self::ROW_MAX,
            Self::ROW_MAX,
            Self::ROW_MAX,
            Self::ROW_MAX,
            Self::ROW_MAX,
            Self::ROW_MAX,
            Self::ROW_MAX,
            Self::ROW_MAX,
            0,
        ],
    };

    pub fn from_location(l: Location) -> Self {
        let mut rows = [0; 16];
        rows[l.row().as_idx()] = 1 << l.column().as_idx();
        Self::new_raw(rows)
    }

    fn new_raw(rows: [u16; 16]) -> Self {
        let mut s = Self { rows };
        s &= Self::FULL;
        assert_eq!(s.rows[15], 0);
        s
    }

    pub fn count_ones(self) -> u32 {
        self.row_iter().fold(0, |acc, r| acc + r.count_ones())
    }

    fn row_iter(&self) -> impl DoubleEndedIterator<Item = &u16> {
        // TODO stop at 15 or 16?
        // 16 auto-vectorizes at least sometimes, 15 is a loop, and row 16 is always zero!
        self.rows.iter().take(16)
    }

    fn row_iter_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut u16> {
        // TODO is just using all 16 faster or slower?
        self.rows.iter_mut().take(15)
    }

    fn invert(&mut self) {
        self.row_iter_mut().for_each(|r| *r = (!*r) & Self::ROW_MAX)
    }

    pub fn right_one(self) -> Self {
        let mut out = self;
        out.row_iter_mut().for_each(|r| *r <<= 1);
        // Normalize: could have shifted a 1 into the empty bits.
        out & Self::FULL
    }

    pub fn left_one(self) -> Self {
        let mut out = self;
        out.row_iter_mut().for_each(|r| *r >>= 1);
        // Don't need to normalize: our empty space can never contain a 1.
        out
    }

    pub fn up_one(self) -> Self {
        let mut out = self;
        for i in (0..15).rev() {
            out.rows[i + 1] = out.rows[i]
        }
        out.rows[0] = 0;
        out.rows[15] = 0;
        out
    }

    pub fn down_one(self) -> Self {
        let mut out = self;
        for i in 0..15 {
            out.rows[i] = out.rows[i + 1]
        }
        // No need to normalize: we don't write to the empty row, and the extra bits are already zero.
        out
    }
}

impl PartialEq for Bitboard {
    fn eq(&self, other: &Self) -> bool {
        self.row_iter()
            .zip(other.row_iter())
            .take(15)
            .all(|(l, r)| l & Self::ROW_MAX == r & Self::ROW_MAX)
    }
}

impl Debug for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut partial = f.debug_struct("Bitboard");
        for (idx, r) in self.row_iter().rev().enumerate() {
            partial.field(
                &format!("r{i:02}", i = idx + 1),
                &format!("{:015b}", r.reverse_bits() >> 1),
            );
        }
        partial.finish()
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.row_iter_mut()
            .zip(rhs.row_iter())
            .for_each(|(l, r)| *l &= r)
    }
}

impl BitAnd for Bitboard {
    type Output = Self;

    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.row_iter_mut()
            .zip(rhs.row_iter())
            .for_each(|(l, r)| *l |= r)
    }
}

impl BitOr for Bitboard {
    type Output = Self;

    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.row_iter_mut()
            .zip(rhs.row_iter())
            .for_each(|(l, r)| *l ^= r)
    }
}

impl BitXor for Bitboard {
    type Output = Self;

    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}

impl Not for Bitboard {
    type Output = Self;

    fn not(mut self) -> Self::Output {
        self.invert();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_ones() {
        let b = Bitboard::FULL;
        assert_eq!(b.count_ones(), 225);
    }

    #[test]
    fn right_one() {
        let b = Bitboard::FULL;
        let result = b.right_one();
        let expected = Bitboard::new_raw([0x7FFE; 16]);
        assert_eq!(result, expected);
    }
    #[test]
    fn fifteen_right_ones_is_empty() {
        let mut b = Bitboard::FULL;
        for _ in 0..14 {
            b = b.right_one();
        }
        assert_ne!(b, Bitboard::EMPTY);
        assert_eq!(b.right_one(), Bitboard::EMPTY);
    }

    #[test]
    fn left_one() {
        let b = Bitboard::FULL;
        let result = b.left_one();
        let expected = Bitboard::new_raw([0x3FFF; 16]);
        assert_eq!(result, expected);
    }

    #[test]
    fn fifteen_left_ones_is_empty() {
        let mut b = Bitboard::FULL;
        for _ in 0..14 {
            b = b.left_one();
        }
        assert_ne!(b, Bitboard::EMPTY);
        assert_eq!(b.left_one(), Bitboard::EMPTY);
    }

    #[test]
    fn up_one() {
        let b = Bitboard::new_raw([
            1,
            0,
            2,
            0,
            1,
            0,
            2,
            0,
            1,
            0,
            2,
            0,
            1,
            0,
            2,
            0,
        ]);
        let expected = Bitboard::new_raw([
            0,
            1,
            0,
            2,
            0,
            1,
            0,
            2,
            0,
            1,
            0,
            2,
            0,
            1,
            0,
            0,
                    ]);
        assert_eq!(b.up_one(), expected);
    }

    #[test]
    fn fifteen_up_ones_is_empty() {
        let mut b = Bitboard::FULL;
        for _ in 0..14 {
            b = b.up_one();
        }
        assert_ne!(b, Bitboard::EMPTY);
        assert_eq!(b.up_one(), Bitboard::EMPTY);
    }

    #[test]
    fn down_one() {
        let b = Bitboard::new_raw([
            0xFFFF, 0x0000, 0xFFFF, 0x0000, 0xFFFF, 0x0000, 0xFFFF, 0x0000, 0xFFFF, 0x0000, 0xFFFF,
            0x0000, 0xFFFF, 0x0000, 0xFFFF, 0x0000,
        ]);
        let expected = Bitboard::new_raw([
            0x0000, 0xFFFF, 0x0000, 0xFFFF, 0x0000, 0xFFFF, 0x0000, 0xFFFF, 0x0000, 0xFFFF, 0x0000,
            0xFFFF, 0x0000, 0xFFFF, 0x0000, 0x0000,
        ]);
        assert_eq!(b.down_one(), expected);
    }

    #[test]
    fn fifteen_down_ones_is_empty() {
        let mut b = Bitboard::FULL;
        for _ in 0..14 {
            b = b.down_one();
        }
        assert_ne!(b, Bitboard::EMPTY);
        assert_eq!(b.down_one(), Bitboard::EMPTY);
    }

    #[test]
    fn equality_ignores_extra_byte_and_bits() {
        let a = Bitboard::FULL;
        let b = Bitboard { rows: [0xFFFF; 16] };
        assert_eq!(a, b);
    }
}
