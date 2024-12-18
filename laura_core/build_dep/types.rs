/*
**  Types definition for Magic BitBoards generation.
*/

/// A `BitBoard` represents a 64-bit chessboard where each bit corresponds to a square.
/// It is useful for efficiently representing and manipulating chess positions.
#[derive(PartialEq, Eq, PartialOrd, Clone, Copy, Debug, Default, Hash)]
pub struct BitBoard(pub u64);

/// Macro to implement bitwise operators for a type.
///
/// This macro generates implementations for bitwise operations such as `&`, `|`, and `^` for the `BitBoard` type.
/// The generated code allows `BitBoard` objects to be used in bitwise operations with another `BitBoard`.
macro_rules! impl_bitwise_op {
    ($trait:ident, $func:ident) => {
        impl std::ops::$trait for BitBoard {
            type Output = Self;

            fn $func(self, other: Self) -> BitBoard {
                Self(std::ops::$trait::$func(self.0, other.0))
            }
        }
    };
}

/// Macro to implement bitwise assignment operators for a type.
///
/// This macro generates implementations for bitwise assignment operations such as `&=`, `|=`, and `^=` for the `BitBoard` type.
/// The generated code allows `BitBoard` objects to perform bitwise assignment operations with another `BitBoard`.
macro_rules! impl_bitwise_assign_op {
    ($trait:ident, $func:ident) => {
        impl std::ops::$trait for BitBoard {
            fn $func(&mut self, other: Self) {
                std::ops::$trait::$func(&mut self.0, other.0)
            }
        }
    };
}

// Implementing bitwise operators for BitBoard
impl_bitwise_op!(BitAnd, bitand);
impl_bitwise_op!(BitOr, bitor);
impl_bitwise_op!(BitXor, bitxor);

// Implementing bitwise assignment operators for BitBoard
impl_bitwise_assign_op!(BitAndAssign, bitand_assign);
impl_bitwise_assign_op!(BitOrAssign, bitor_assign);
impl_bitwise_assign_op!(BitXorAssign, bitxor_assign);

/// Implements the `Not` trait for `BitBoard`, allowing the bitwise NOT operation `!`.
/// The bitwise NOT flips all bits in the `BitBoard`, effectively inverting the board state.
impl std::ops::Not for BitBoard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

/// Implements display formatting for the `BitBoard` struct.
/// This allows for the `BitBoard` to be printed in a human-readable format,
/// where filled squares are shown as '★' and empty squares as '·'.
impl std::fmt::Display for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

/// Methods for the `BitBoard` struct, including utilities for manipulating bits and interacting with squares.
impl BitBoard {
    pub const EMPTY: BitBoard = BitBoard(0);

    pub const fn set_square(self, square: Square) -> Self {
        Self(self.0 | 1u64 << square.to_index())
    }

    pub const fn get_square(self, square: Square) -> bool {
        self.0 & (1u64 << square.to_index()) != 0
    }

    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
}

/// Enum representing each square on a chessboard, from A1 to H8.
/// The squares are ordered by rank (rows) and file (columns), with A1 as the bottom-left and H8 as the top-right.
#[derive(PartialEq, Ord, Eq, PartialOrd, Copy, Clone, Debug, Hash)]
#[repr(u8)]
#[rustfmt::skip]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1, A2, B2, C2, D2, E2, F2, G2, H2, A3, B3, C3, D3, E3, F3, G3, H3, A4, B4, C4, D4, E4, F4, G4, H4, A5, B5, C5, D5, E5, F5, G5, H5, A6, B6, C6, D6, E6, F6, G6, H6, A7, B7, C7, D7, E7, F7, G7, H7, A8, B8, C8, D8, E8, F8, G8, H8,
}

impl Square {
    pub const NUM_SQUARES: usize = 64;

    pub const fn from_file_rank(file: File, rank: Rank) -> Self {
        let index: u8 = (rank as u8) << 3 ^ (file as u8);
        unsafe { std::mem::transmute(index & 63) }
    }

    pub const fn from_index(index: usize) -> Self {
        unsafe { std::mem::transmute(index as u8 & 63) }
    }

    pub const fn to_index(self) -> usize {
        self as usize
    }

    pub const fn to_bitboard(self) -> BitBoard {
        BitBoard(1u64 << self.to_index())
    }

    pub const fn rank(self) -> Rank {
        unsafe { std::mem::transmute((self as u8 >> 3) & 7) }
    }

    pub const fn file(self) -> File {
        unsafe { std::mem::transmute(self as u8 & 7) }
    }
}

/// Enum representing the files (columns) on a chessboard.
/// Files are labeled from 'A' to 'H'.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Debug, Hash)]
#[repr(u8)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

/// Enum representing the ranks (rows) on a chessboard.
/// Ranks are numbered from 'One' (1) to 'Eight' (8).
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Debug, Hash)]
#[repr(u8)]
pub enum Rank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}
