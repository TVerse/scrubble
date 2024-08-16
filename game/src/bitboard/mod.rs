use std::{
    fmt::Debug,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not},
};

mod scalar;
#[cfg(target_feature = "avx2")]
mod avx2;

#[cfg(not(target_feature = "avx2"))]
pub use scalar::BitboardImpl;
#[cfg(target_feature = "avx2")]
pub use avx2::BitboardImpl;

use crate::board::Location;

pub trait Bitboard:
    Copy + Clone + Debug + Not + BitAnd + BitOr + BitAndAssign + BitOrAssign
{
    fn empty() -> Self;
    fn full() -> Self;

    fn for_location(l: Location) -> Self;
    fn new_raw(rows: [u16; 16]) -> Self;

    fn count_ones(self) -> u32;

    fn right(self, by: usize) -> Self;
    fn left(self, by: usize) -> Self;
    fn up(self, by: usize) -> Self;
    fn down(self, by: usize) -> Self;
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};
    use proptest::prelude::*;

    prop_compose! {
        fn arb_bitboard()(id in any::<[[bool; 15]; 15]>()) -> BitboardImpl {
            let mut raw: [u16; 16] = [0; 16];
            for i in 0..15 {
                for b in 0..15 {
                    raw[i] |= id[i][b] as u16 >> b
                }
            }
            BitboardImpl::new_raw(raw)
        }
    }

    #[test]
    fn count_ones() {
        let b = BitboardImpl::full();
        assert_eq!(b.count_ones(), 225);
    }

    #[test]
    fn right_one() {
        let b = BitboardImpl::full();
        let result = b.right(1);
        let expected = BitboardImpl::new_raw([0x7FFE; 16]);
        assert_eq!(result, expected);
    }
    #[test]
    fn fifteen_right_ones_is_empty() {
        let mut b = BitboardImpl::full();
        for _ in 0..14 {
            b = b.right(1);
        }
        assert_ne!(b, BitboardImpl::empty());
        assert_eq!(b.right(1), BitboardImpl::empty());
    }

    #[test]
    fn left_one() {
        let b = BitboardImpl::full();
        let result = b.left(1);
        let expected = BitboardImpl::new_raw([0x3FFF; 16]);
        assert_eq!(result, expected);
    }

    #[test]
    fn fifteen_left_ones_is_empty() {
        let mut b = BitboardImpl::full();
        for _ in 0..14 {
            b = b.left(1);
        }
        assert_ne!(b, BitboardImpl::empty());
        assert_eq!(b.left(1), BitboardImpl::empty());
    }

    #[test]
    fn up_one() {
        let b = BitboardImpl::new_raw([1, 0, 2, 0, 1, 0, 2, 0, 1, 0, 2, 0, 1, 0, 2, 0]);
        let expected = BitboardImpl::new_raw([0, 1, 0, 2, 0, 1, 0, 2, 0, 1, 0, 2, 0, 1, 0, 0]);
        assert_eq!(b.up(1), expected);
    }

    #[test]
    fn fifteen_up_ones_is_empty() {
        let mut b = BitboardImpl::full();
        for _ in 0..14 {
            b = b.up(1);
        }
        assert_ne!(b, BitboardImpl::empty());
        assert_eq!(b.up(1), BitboardImpl::empty());
    }

    #[test]
    fn down_one() {
        let b = BitboardImpl::new_raw([
            0xFFFF, 0x0000, 0xFFFF, 0x0000, 0xFFFF, 0x0000, 0xFFFF, 0x0000, 0xFFFF, 0x0000, 0xFFFF,
            0x0000, 0xFFFF, 0x0000, 0xFFFF, 0x0000,
        ]);
        let expected = BitboardImpl::new_raw([
            0x0000, 0xFFFF, 0x0000, 0xFFFF, 0x0000, 0xFFFF, 0x0000, 0xFFFF, 0x0000, 0xFFFF, 0x0000,
            0xFFFF, 0x0000, 0xFFFF, 0x0000, 0x0000,
        ]);
        assert_eq!(b.down(1), expected);
    }

    #[test]
    fn fifteen_down_ones_is_empty() {
        let mut b = BitboardImpl::full();
        for _ in 0..14 {
            b = b.down(1);
        }
        assert_ne!(b, BitboardImpl::empty());
        assert_eq!(b.down(1), BitboardImpl::empty());
    }

    #[test]
    fn not_empty_full() {
        assert_eq!(!BitboardImpl::empty(), BitboardImpl::full());
        assert_eq!(!BitboardImpl::full(), BitboardImpl::empty());
    }

    proptest! {
      #[test]
      fn up_consistency(by in 0..15usize, bb in arb_bitboard()) {
          let a = bb.up(by);
          let b = (0..by).fold(bb, |bb, _| bb.up(1));
          assert_eq!(a, b);
      }

      #[test]
      fn down_consistency(by in 0..15usize, bb in arb_bitboard()) {
          let a = bb.down(by);
          let b = (0..by).fold(bb, |bb, _| bb.down(1));
          assert_eq!(a, b);
      }

      #[test]
      fn left_consistency(by in 0..15usize, bb in arb_bitboard()) {
          let a = bb.left(by);
          let b = (0..by).fold(bb, |bb, _| bb.left(1));
          assert_eq!(a, b);
      }

      #[test]
      fn right_consistency(by in 0..15usize, bb in arb_bitboard()) {
          let a = bb.right(by);
          let b = (0..by).fold(bb, |bb, _| bb.right(1));
          assert_eq!(a, b);
      }

      #[test]
      fn partialeq_consistency(bb in arb_bitboard()) {
        assert_eq!(bb, bb);
      }

      #[test]
      fn bitand_identity(bb in arb_bitboard()) {
        assert_eq!(bb & BitboardImpl::full(), bb);
      }

      #[test]
      fn bitand_zero(bb in arb_bitboard()) {
        assert_eq!(bb & BitboardImpl::empty(), BitboardImpl::empty());
      }

      #[test]
      fn bitor_identity(bb in arb_bitboard()) {
        assert_eq!(bb | BitboardImpl::empty(), bb);
      }

      #[test]
      fn bitor_zero(bb in arb_bitboard()) {
        assert_eq!(bb | BitboardImpl::full(), BitboardImpl::full());
      }

      #[test]
      fn invert_involution(bb in arb_bitboard()) {
        assert_eq!(!!bb, bb);
      }
    }
}
