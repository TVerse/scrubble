use std::{
    arch::x86_64::__m256i,
    fmt::Debug,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
};

use crate::board::Location;

use super::Bitboard;

use safe_arch::{
    m128i, m256i, set_splat_i16_m256i, shl_all_u16_m256i, shr_all_i16_m256i, zeroed_m256i,
};

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
pub struct BitboardImpl {
    rows: m256i,
}

impl BitboardImpl {
    const ROW_MAX: u16 = 0x7FFF;

    const EMPTY: [u16; 16] = [0; 16];
    const FULL: [u16; 16] = [
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
    ];
}

impl Bitboard for BitboardImpl {
    fn empty() -> Self {
        Self {
            rows: zeroed_m256i(),
        }
    }

    fn full() -> Self {
        Self {
            rows: Self::FULL.into(),
        }
    }

    fn for_location(l: Location) -> Self {
        let mut rows = [0; 16];
        rows[l.row().as_idx()] = 1 << l.column().as_idx();
        Self::new_raw(rows)
    }

    fn new_raw(mut rows: [u16; 16]) -> Self {
        rows[15] = 0;
        Self { rows: rows.into() }
    }

    fn count_ones(self) -> u32 {
        let rows: [u16; 16] = self.rows.into();
        rows.iter().fold(0, |acc, r| acc + r.count_ones())
    }

    fn right(self, by: usize) -> Self {
        let tmp = shl_all_u16_m256i(self.rows, m128i::from(by as u128));
        let rows = tmp & Self::full().rows;
        Self { rows }
    }

    fn left(self, by: usize) -> Self {
        Self {
            rows: shr_all_i16_m256i(self.rows, m128i::from(by as u128)),
        }
    }

    fn up(self, by: usize) -> Self {
        let by = by as usize;
        let mut rows: [u16; 16] = self.rows.into();
        rows.rotate_right(by);
        for i in 0..by {
            rows[i] = 0
        }
        rows[15] = 0;
        let rows = rows.into();
        Self { rows }
    }

    fn down(self, by: usize) -> Self {
        let by = by as usize;
        let mut rows: [u16; 16] = self.rows.into();
        rows.rotate_left(by);
        for i in (0..by).rev() {
            rows[14 - i] = 0
        }
        rows[15] = 0;
        let rows = rows.into();
        Self { rows }
    }
}

impl PartialEq for BitboardImpl {
    fn eq(&self, other: &Self) -> bool {
        (*self ^ *other).rows == Self::empty().rows
    }
}

impl Debug for BitboardImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut partial = f.debug_struct("Bitboard");
        let rows: [u16; 16] = self.rows.into();
        for (idx, r) in rows.iter().enumerate().rev() {
            partial.field(
                &format!("r{i:02}", i = idx + 1),
                &format!("{:016b}", r.reverse_bits() >> 1),
            );
        }
        partial.finish()
    }
}

impl BitAndAssign for BitboardImpl {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs
    }
}

impl BitAnd for BitboardImpl {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            rows: self.rows & rhs.rows,
        }
    }
}

impl BitOrAssign for BitboardImpl {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs
    }
}

impl BitOr for BitboardImpl {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            rows: self.rows | rhs.rows,
        }
    }
}

impl BitXorAssign for BitboardImpl {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs
    }
}

impl BitXor for BitboardImpl {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self {
            rows: self.rows ^ rhs.rows,
        }
    }
}

impl Not for BitboardImpl {
    type Output = Self;

    fn not(self) -> Self::Output {
        self ^ Self::full()
    }
}
