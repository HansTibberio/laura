use std::mem::transmute;
use crate::bitboard::BitBoard;
use crate::color::Color;
use crate::square::Square;


/// Defines the deltas for white pawn attacks. These tuples represent the 
/// relative movements a white pawn can make to attack diagonally forward.
const WHITE_PAWN_DELTAS: [(i8, i8); 2] = [
    (1, -1), (1, 1),
];

/// Defines the deltas for black pawn attacks. These tuples represent the 
/// relative movements a black pawn can make to attack diagonally forward.
const BLACK_PAWN_DELTAS: [(i8, i8); 2] = [
    (-1, -1), (-1, 1),
];

/// Precomputed pawn attack BitBoards for all squares on the chessboard.
///
/// This constant provides a quick lookup for the possible attack positions
/// for both white and black pawns from each square. It is structured as a
/// 2D array where the first dimension represents the pawn color (0 for
/// white, 1 for black) and the second dimension corresponds to the square
/// index (0 to 63). Each entry in the array contains a BitBoard indicating
/// the squares that the pawn can attack from that position.
pub const PAWN_ATTACKS: [[BitBoard; 64]; 2] = [
    [BitBoard(512), BitBoard(1280), BitBoard(2560), BitBoard(5120), BitBoard(10240), BitBoard(20480), BitBoard(40960), BitBoard(16384), BitBoard(131072), BitBoard(327680), BitBoard(655360), BitBoard(1310720), BitBoard(2621440), BitBoard(5242880), BitBoard(10485760), BitBoard(4194304), BitBoard(33554432), BitBoard(83886080), BitBoard(167772160), BitBoard(335544320), BitBoard(671088640), BitBoard(1342177280), BitBoard(2684354560), BitBoard(1073741824), BitBoard(8589934592), BitBoard(21474836480), BitBoard(42949672960), BitBoard(85899345920), BitBoard(171798691840), BitBoard(343597383680), BitBoard(687194767360), BitBoard(274877906944), BitBoard(2199023255552), BitBoard(5497558138880), BitBoard(10995116277760), BitBoard(21990232555520), BitBoard(43980465111040), BitBoard(87960930222080), BitBoard(175921860444160), BitBoard(70368744177664), BitBoard(562949953421312), BitBoard(1407374883553280), BitBoard(2814749767106560), BitBoard(5629499534213120), BitBoard(11258999068426240), BitBoard(22517998136852480), BitBoard(45035996273704960), BitBoard(18014398509481984), BitBoard(144115188075855872), BitBoard(360287970189639680), BitBoard(720575940379279360), BitBoard(1441151880758558720), BitBoard(2882303761517117440), BitBoard(5764607523034234880), BitBoard(11529215046068469760), BitBoard(4611686018427387904), BitBoard(0), BitBoard(0), BitBoard(0), BitBoard(0), BitBoard(0), BitBoard(0), BitBoard(0), BitBoard(0)],
    [BitBoard(0), BitBoard(0), BitBoard(0), BitBoard(0), BitBoard(0), BitBoard(0), BitBoard(0), BitBoard(0), BitBoard(2), BitBoard(5), BitBoard(10), BitBoard(20), BitBoard(40), BitBoard(80), BitBoard(160), BitBoard(64), BitBoard(512), BitBoard(1280), BitBoard(2560), BitBoard(5120), BitBoard(10240), BitBoard(20480), BitBoard(40960), BitBoard(16384), BitBoard(131072), BitBoard(327680), BitBoard(655360), BitBoard(1310720), BitBoard(2621440), BitBoard(5242880), BitBoard(10485760), BitBoard(4194304), BitBoard(33554432), BitBoard(83886080), BitBoard(167772160), BitBoard(335544320), BitBoard(671088640), BitBoard(1342177280), BitBoard(2684354560), BitBoard(1073741824), BitBoard(8589934592), BitBoard(21474836480), BitBoard(42949672960), BitBoard(85899345920), BitBoard(171798691840), BitBoard(343597383680), BitBoard(687194767360), BitBoard(274877906944), BitBoard(2199023255552), BitBoard(5497558138880), BitBoard(10995116277760), BitBoard(21990232555520), BitBoard(43980465111040), BitBoard(87960930222080), BitBoard(175921860444160), BitBoard(70368744177664), BitBoard(562949953421312), BitBoard(1407374883553280), BitBoard(2814749767106560), BitBoard(5629499534213120), BitBoard(11258999068426240), BitBoard(22517998136852480), BitBoard(45035996273704960), BitBoard(18014398509481984)]
];

/// Generates the attack tables for both white and black pawns for all squares
/// on the chessboard.
///
/// This function iterates over all squares (0 to 63) and calculates the possible
/// attack moves for both white and black pawns based on their movement deltas.
pub fn gen_pawn_attacks() -> [[BitBoard; 64]; 2] {
    let mut pawn_attacks: [[BitBoard; 64]; 2] = [[BitBoard::EMPTY; 64]; 2];

    for sq in 0..Square::NUM_SQUARES {
        let square: Square = Square::from_index(sq);
        let rank: i8 = square.rank() as i8;
        let file: i8 = square.file() as i8;
        let white: usize = Color::White as usize;
        let black: usize = Color::Black as usize;

        for (dr, df) in WHITE_PAWN_DELTAS.iter() {
            let new_rank: i8 = rank + dr;
            let new_file: i8 = file + df;

            if (0..8).contains(&new_rank) && (0..8).contains(&new_file) {
                let new_square: Square = Square::from_file_rank(
                    unsafe { transmute(new_file as u8) },
                    unsafe { transmute(new_rank as u8) }
                );
                pawn_attacks[white][sq] = pawn_attacks[white][sq].set_square(new_square);
            }
        }

        for (dr, df) in BLACK_PAWN_DELTAS.iter() {
            let new_rank: i8 = rank + dr;
            let new_file: i8 = file + df;

            if (0..8).contains(&new_rank) && (0..8).contains(&new_file) {
                let new_square: Square = Square::from_file_rank(
                    unsafe { transmute(new_file as u8) },
                    unsafe { transmute(new_rank as u8) }
                );
                pawn_attacks[black][sq] = pawn_attacks[black][sq].set_square(new_square);
            }
        }
    }

    pawn_attacks
}

#[test]
fn test_gen_attacks(){
    let pawn_attacks: [[BitBoard; 64]; 2] = gen_pawn_attacks();

    let square: usize = Square::E3.to_index();
    let color:Color = Color::White;
    println!("Pawn attacks ({}) from E3: {}", color, pawn_attacks[color as usize][square]);

    let square: usize = Square::D5.to_index();
    let color:Color = Color::Black;
    println!("Pawn attacks ({}) from D5: {}", color, pawn_attacks[color as usize][square]);
}

#[test]
fn gen_attacks(){
    let attacks: [[BitBoard; 64]; 2] = gen_pawn_attacks();
    println!("{:?}", attacks);
}