use std::mem::transmute;
use std::fmt;
use std::ops::Not;

use crate::square::Square;

// Constants representing various positions and sections of the chessboard as `u64` bitboards.
// These constants use specific bit patterns to represent different files, ranks, and sides on the board.

const WHITE_SIDE: u64 = 0x0000_0000_FFFF_FFFF;
const BLACK_SIDE: u64 = 0xFFFF_FFFF_0000_0000;

// File bitboards (columns on the chessboard)
const FILE_A: u64 = 0x0101_0101_0101_0101;
const FILE_B: u64 = 0x0202_0202_0202_0202;
const FILE_C: u64 = 0x0404_0404_0404_0404;
const FILE_D: u64 = 0x0808_0808_0808_0808;
const FILE_E: u64 = 0x1010_1010_1010_1010;
const FILE_F: u64 = 0x2020_2020_2020_2020;
const FILE_G: u64 = 0x4040_4040_4040_4040;
const FILE_H: u64 = 0x8080_8080_8080_8080;

// Rank bitboards (rows on the chessboard)
const RANK_1: u64 = 0x0000_0000_0000_00FF;
const RANK_2: u64 = 0x0000_0000_0000_FF00;
const RANK_3: u64 = 0x0000_0000_00FF_0000;
const RANK_4: u64 = 0x0000_0000_FF00_0000;
const RANK_5: u64 = 0x0000_00FF_0000_0000;
const RANK_6: u64 = 0x0000_FF00_0000_0000;
const RANK_7: u64 = 0x00FF_0000_0000_0000;
const RANK_8: u64 = 0xFF00_0000_0000_0000;

// Bitboards representing dark and light squares
const DARK_SQUARES: u64 = 0xAA55_AA55_AA55_AA55;
const LIGHT_SQUARES: u64 = 0x55AA_55AA_55AA_55AA;

// Full bitboard (all squares occupied)
const FULL: u64 = 0xFFFF_FFFF_FFFF_FFFF;

/// A `BitBoard` represents a 64-bit chessboard where each bit corresponds to a square.
/// It is useful for efficiently representing and manipulating chess positions.
/// 
/// The bitboard follows the Little-Endian Rank-File (LERF) notation. 
/// In this system, the least significant bit (LSB) represents the bottom-left corner of the chessboard, 
/// while the most significant bit (MSB) represents the top-right corner.
/// 
///  ```md,ignore
/// 8 | 56 57 58 59 60 61 62 63
/// 7 | 48 49 50 51 52 53 54 55
/// 6 | 40 41 42 43 44 45 46 47
/// 5 | 32 33 34 35 36 37 38 39
/// 4 | 24 25 26 27 28 29 30 31
/// 3 | 16 17 18 19 20 21 22 23
/// 2 | 8  9  10 11 12 13 14 15
/// 1 | 0  1  2  3  4  5  6  7
///    -------------------------
///      A  B  C  D  E  F  G  H
/// ```
#[derive(PartialEq, Eq, PartialOrd, Clone, Copy, Debug, Default, Hash)]
pub struct BitBoard(pub u64);

/// Implements display formatting for the `BitBoard` struct.
/// This allows for the `BitBoard` to be printed in a human-readable format,
/// where filled squares are shown as '★' and empty squares as '·'.
impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s: String = format!("\n      Bitboard: {}\n", self.0);

        for rank in (0..8).rev() {
            s.push_str(format!("\n{}   ", rank + 1).as_str());
            for file in 0..8 {
                let square = rank * 8 + file;
                if self.get_square(Square::from_index(square)) {
                    s.push_str("★ ");
                } else {
                    s.push_str("· ");
                }
            }
        }
        s.push_str("\n\n    A B C D E F G H");
        write!(f, "{s}")
    }
}

/// Implements the `Not` trait for `BitBoard`, allowing the bitwise NOT operation `!`.
/// The bitwise NOT flips all bits in the `BitBoard`, effectively inverting the board state.
impl Not for BitBoard {
    type Output = Self;

    #[inline]
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

/// Implements `Iterator` for `BitBoard`, allowing iteration over the set squares.
/// Each call to `next` returns the next `Square` that is set (i.e., the next '1' bit)
impl Iterator for BitBoard {
    type Item = Square;

    #[inline]
    fn next(&mut self) -> Option<Square> {
        if *self == Self::EMPTY {
            None
        } else {
            let square: Square = self.to_square();
            self.0 &= self.0 - 1;

            Some(square)
        }
    }
}

/// Methods for the `BitBoard` struct, including utilities for manipulating bits and interacting with squares.
impl BitBoard {

    // Predefined `BitBoard` constants for sides, files, and ranks
    pub const WHITE_SIDE: BitBoard = BitBoard(WHITE_SIDE);
    pub const BLACK_SIDE: BitBoard = BitBoard(BLACK_SIDE);
    
    pub const FILE_A: BitBoard = BitBoard(FILE_A);
    pub const FILE_B: BitBoard = BitBoard(FILE_B);
    pub const FILE_C: BitBoard = BitBoard(FILE_C);
    pub const FILE_D: BitBoard = BitBoard(FILE_D);
    pub const FILE_E: BitBoard = BitBoard(FILE_E);
    pub const FILE_F: BitBoard = BitBoard(FILE_F);
    pub const FILE_G: BitBoard = BitBoard(FILE_G);
    pub const FILE_H: BitBoard = BitBoard(FILE_H);

    pub const RANK_1: BitBoard = BitBoard(RANK_1);
    pub const RANK_2: BitBoard = BitBoard(RANK_2);
    pub const RANK_3: BitBoard = BitBoard(RANK_3);
    pub const RANK_4: BitBoard = BitBoard(RANK_4);
    pub const RANK_5: BitBoard = BitBoard(RANK_5);
    pub const RANK_6: BitBoard = BitBoard(RANK_6);
    pub const RANK_7: BitBoard = BitBoard(RANK_7);
    pub const RANK_8: BitBoard = BitBoard(RANK_8);

    pub const DARK_SQUARES: BitBoard = BitBoard(DARK_SQUARES);
    pub const LIGHT_SQUARES: BitBoard = BitBoard(LIGHT_SQUARES);

    pub const EMPTY: BitBoard = BitBoard(0);
    pub const FULL: BitBoard = BitBoard(FULL);

    /// Converts the `BitBoard` to a `Square`, returning the square that corresponds to the least significant '1' bit.
    #[inline]
    pub const fn to_square(self) -> Square {
        unsafe { transmute((self.0.trailing_zeros() as u8) & 63) }
    }

    /// Sets a given `Square` on the `BitBoard`, turning the bit at the square's position to '1'.
    #[inline]
    pub const fn set_square(self, square: Square) -> Self {
        Self(self.0 | 1u64 << square.to_index())
    }

    /// Checks if a specific `Square` is set on the `BitBoard`.
    /// 
    /// `true` if the square is set, otherwise `false`.
    #[inline]
    pub const fn get_square(self, square: Square) -> bool {
        self.0 & (1u64 << square.to_index()) != 0
    }

    /// Clears a specific `Square` on the `BitBoard`, turning the bit at the square's position to '0'.
    #[inline]
    pub const fn pop_square(self, square: Square) -> Self {
        Self(self.0 & !(1u64 << square.to_index()))
    }

    /// Counts the number of set bits (i.e., the number of squares occupied) on the `BitBoard`.
    #[inline]
    pub const fn count_bits(self) -> u32 {
        self.0.count_ones()
    }

    /// Flips the bitboard vertically, swapping rows (ranks) across the horizontal axis.
    #[inline]
    pub const fn flip(self) -> Self {
        Self(self.0.swap_bytes())
    }
}

#[test]
fn bitboard_test(){
    let bitboard: BitBoard = BitBoard(2097152);
    assert_eq!(bitboard.to_square(), Square::F3);
    println!("{}", bitboard);
    let bitboard: BitBoard = bitboard.set_square(Square::G6);
    println!("{}", bitboard);
    assert_eq!(bitboard.get_square(Square::G6), true);
    let bitboard: BitBoard = bitboard.set_square(Square::B5);
    assert_eq!(bitboard.count_bits(), 3);
    println!("{}", bitboard);
    let bitboard: BitBoard = bitboard.pop_square(Square::G6);
    assert_eq!(bitboard.count_bits(), 2);
    println!("{}", bitboard);
}