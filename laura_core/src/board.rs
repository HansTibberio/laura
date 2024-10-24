
use std::str::FromStr;
use std::fmt;

use crate::bitboard::BitBoard;
use crate::castle_rights::{get_rook_castling, CastleRights};
use crate::moves::{Move, MoveType};
use crate::piece::{Piece, PieceType};
use crate::color::Color;
use crate::file::File;
use crate::rank::Rank;
use crate::square::Square;
use crate::zobrist::Zobrist;

/// Represents a chess board, with bitboards for tracking piece positions,
/// castling rights, en passant squares, the fifty-move rule counter, and
/// Zobrist hashing for fast state comparison.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Board {
    /// Array of bitboards, one for each type of piece. Each bitboard tracks
    /// the positions of that specific piece type on the board.
    pub pieces_bitboard: [BitBoard; Piece::NUM_PIECES],

    /// Bitboards for the sides: one for white pieces, one for black pieces.
    pub sides_bitboard: [BitBoard; 2],

    /// Maps squares to the piece occupying them, if any.
    pub piece_map: [Option<Piece>; Square::NUM_SQUARES],

    /// The square available for an en passant capture, if applicable.
    pub enpassant_square: Option<Square>,

    /// The castling rights of the current position.
    pub castling: CastleRights,
    
    /// Counter for the fifty-move rule, tracking half-moves since the last capture or pawn move.
    pub fifty_move: i16,

    /// The number of plies (half-moves) made in the current game.
    pub ply: u16,

    /// The Zobrist hash representing the current board state.
    pub zobrist: Zobrist,

    /// The side to move (either White or Black).
    pub side: Color,

    /// Bitboard representing squares that are currently in check.
    pub checkers: BitBoard,
}

/// Displays the current state of the chess board in a readable format, including
/// FEN notation, Zobrist hash, and a grid representation of the board.
///
/// The board is displayed using Unicode characters for better visual clarity.
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut board_str: String = format!(
            "\n FEN: {}\n Zobrist: {}\n\n\t┏━━━┳━━━┳━━━┳━━━┳━━━┳━━━┳━━━┳━━━┓", 
            self.to_fen(), 
            self.zobrist
        );

        for rank in (0..Rank::NUM_RANKS).rev() {
            board_str.push_str(format!("\n      {} ┃ ", rank + 1).as_str());

            for file in 0..File::NUM_FILES {
                let square_index: usize = rank * 8 + file;
                let piece_str: String = self.piece_map[square_index]
                    .map_or(String::from(" "), |p|p.to_string());
                board_str.push_str(&piece_str);
                board_str.push_str(" ┃ ");
            }

            if rank != Rank::One.to_index() {
                board_str.push_str("\n\t┣━━━╋━━━╋━━━╋━━━╋━━━╋━━━╋━━━╋━━━┫");
            }
        }
        
        board_str.push_str("\n\t┗━━━┻━━━┻━━━┻━━━┻━━━┻━━━┻━━━┻━━━┛\n\t  A   B   C   D   E   F   G   H\n");

        let enpassant_str = match self.enpassant_square {
            Some(square) => format!("{square}"),
            None => String::from("-"),
        };

        write!(
            f,
            "{}
            Side to move        : {}
            Castling Rights     : {}
            En Passante square  : {}
            Fifty Rule          : {}
            ", 
            board_str, 
            self.side, 
            self.castling, 
            enpassant_str, 
            self.fifty_move,
        )
    }
}

/// Parses a FEN string to create a new `Board` instance. The FEN string is split
/// into 6 parts: piece placement, active color, castling rights, en passant target
/// square, halfmove clock, and fullmove number.
impl FromStr for Board {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fen: Vec<&str> = s.split_whitespace().take(6).collect();

        if fen.len() != 6 {
            return Err("Invalid FEN!");
        }

        let mut board: Board = Self::new();
        let board_str: &str = fen[0];
        let mut count: i32 = 0;

        let (mut file, mut rank) = (File::A, Rank::Eight);
        for token in board_str.chars() {
            match token {
                '/' => {
                    if count != 8 {
                        return Err("Invalid FEN!");
                    };

                    rank = rank.down();
                    count = 0;
                }
                '1'..='8' => {
                    for _ in '1'..=token {
                        file = file.right();
                        count += 1;
                    }
                }
                _ => {
                    board.set_piece(Piece::try_from(token)?, Square::from_file_rank(file, rank));
                    file = file.right();
                    count += 1;
                }
            }
        }

        if count != 8 {
            return Err("Invalid FEN!");
        }

        match fen[1] {
            "w" => {
                board.side = Color::White;
                board.zobrist.hash_side();
            }
            "b" => board.side = Color::Black,
            _ => return Err("Invalid FEN!"),
        }

        let castle_rights: CastleRights = fen[2].parse()?;
        board.castling = castle_rights;
        board.zobrist.hash_castle(castle_rights);

        match fen[3] {
            "-" => board.enpassant_square = None,
            _ => {
                let ep_square: Square = fen[3].parse()?;

                board.enpassant_square = Some(ep_square);
                board.zobrist.hash_enpassant(ep_square);
            }
        }

        match fen[4].parse::<i16>() {
            Ok(half_move) => board.fifty_move = half_move,
            Err(_) => return Err("Invalid Halfmove Clock!"),
        }

        match fen[5].parse::<u16>() {
            Ok(full_move) => board.ply = full_move,
            Err(_) => return Err("Invalid Fullmove Number!"),
        }

        board.checkers = board.checkers();

        Ok(board)
    }
}

/// Constructs a default chess board, representing the standard starting position
/// for a chess game, using FEN notation. The default position is the classic setup
/// with castling rights and no en passant.
impl Default for Board {
    #[inline]
    fn default() -> Self {
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".parse().unwrap()
    }
}

impl Board {

    /// Creates a new empty board with no pieces. The bitboards are initialized as empty,
    /// and castling rights, en passant square, and other attributes are set to their
    /// default (empty or zero) values.
    pub const fn new() -> Self {
        Self {
            pieces_bitboard: [BitBoard::EMPTY; Piece::NUM_PIECES],
            sides_bitboard: [BitBoard::EMPTY; 2],
            piece_map: [None; Square::NUM_SQUARES],
            enpassant_square: None,
            castling: CastleRights::null(),
            fifty_move: 0,
            ply: 0,
            zobrist: Zobrist::null(),
            side: Color::White,
            checkers: BitBoard::EMPTY,
        }
    }

    /// Converts the current board state into a FEN (Forsyth-Edwards Notation) string.
    ///
    /// FEN is a standard notation for describing a particular board position of a chess game. 
    /// It includes information about the placement of pieces, which side is to move, castling rights, 
    /// en passant target squares, the half-move clock (for the fifty-move rule), and the full-move number.
    pub fn to_fen(&self)  -> String  {
        let mut fen: String = String::new();

        for rank in (0..Rank::NUM_RANKS).rev() {
            let mut empty_squares: i32 = 0;

            for file in 0..File::NUM_FILES {
                let square_index:usize  = rank * 8 + file;

                if let Some(piece) = self.piece_map[square_index] {
                    if empty_squares > 0 {
                        fen.push_str(&empty_squares.to_string());
                        empty_squares = 0;
                    }
                    fen.push(piece.to_char());
                } else {
                    empty_squares += 1;
                }
            }

            if empty_squares > 0 {
                fen.push_str(&empty_squares.to_string());
            }

            if rank != Rank::One.to_index() {
                fen.push('/');
            }
        }

        fen.push(' ');
        fen.push(match self.side {
            Color::White => 'w',
            Color::Black => 'b',
        });
        fen.push(' ');

        fen.push_str(&self.castling.to_string());
        fen.push(' ');

        if let Some(enpassant_square) = self.enpassant_square {
            fen.push_str(&enpassant_square.to_string());
        } else {
            fen.push('-');
        }

        fen.push_str(&format!(" {} {}", self.fifty_move, self.ply));

        fen
    }

    /// Sets a piece on the board at a given square and updates the corresponding bitboards
    /// and Zobrist hash. This method modifies both the specific piece bitboard and the
    /// side's bitboard (either White or Black).
    pub fn set_piece(&mut self, piece: Piece, square: Square) {
        let index: usize = piece.to_index();
        let color: usize = piece.color() as usize;

        self.pieces_bitboard[index] = self.pieces_bitboard[index].set_square(square);
        self.sides_bitboard[color] = self.sides_bitboard[color].set_square(square);
        self.piece_map[square.to_index()] = Some(piece);
        self.zobrist.hash_piece(piece, square);
    }

    /// Removes a piece from a square and updates the corresponding bitboards and
    /// Zobrist hash.
    pub fn remove_piece(&mut self, square: Square) {
        let piece: Piece = self.piece_on(square);
        let index = piece.to_index();
        let color = piece.color() as usize;

        self.pieces_bitboard[index] = self.pieces_bitboard[index].pop_square(square);
        self.sides_bitboard[color] = self.sides_bitboard[color].pop_square(square);
        self.piece_map[square.to_index()] = None;
        self.zobrist.hash_piece(piece, square);
    }

    /// Returns the piece located on the specified square.
    /// /// This function will panic if no piece is present on the specified square,
    /// as it calls `unwrap()` on an `Option`.
    #[inline]
    pub fn piece_on(&self, square: Square) -> Piece {
        self.piece_map[square.to_index()].unwrap()
    }

    /// Returns the bitboard representing all pieces for the white side.
    #[inline(always)]
    pub const fn white_bitboard(&self) -> BitBoard {
        self.sides_bitboard[Color::White as usize]
    }

    /// Returns the bitboard representing all pieces for the black side.
    #[inline(always)]
    pub const fn black_bitboard(&self) -> BitBoard {
        self.sides_bitboard[Color::Black as usize]
    }

    /// Returns a bitboard representing all pieces currently on the board for both sides.
    /// 
    /// This function combines the bitboards for both white and black pieces by performing
    /// a bitwise OR operation.
    #[inline(always)]
    pub const fn combined_bitboard(&self) -> BitBoard {
        BitBoard(self.white_bitboard().0 | self.black_bitboard().0)
    }

    /// Returns the side to move (white or black).
    #[inline(always)]
    pub const fn side(&self) -> Color {
        self.side
    }

    /// Returns the Zobrist hash of the current board position.
    /// 
    /// The Zobrist hash is a unique value representing the current state of the board.
    /// It is used for hashing positions in transposition tables.
    #[inline(always)]
    pub const fn zobrist(&self) -> Zobrist {
        self.zobrist
    }

    /// Returns the current value of the fifty-move counter.
    /// 
    /// The fifty-move rule in chess allows a draw to be claimed if no capture or pawn movement
    /// has occurred in the last fifty moves.
    #[inline(always)]
    pub const fn fifty_move(&self) -> i16 {
        self.fifty_move
    }

    /// Returns the bitboard representing the pieces that are checking the king.
    pub fn checkers(&self) -> BitBoard {
        BitBoard::EMPTY
    }

    /// Executes a move on the chessboard, updating the board state, castling rights, 
    /// en passant square, fifty-move rule counter, and Zobrist hash accordingly.
    ///
    /// This function clones the current board state, applies the given move, 
    /// and returns the resulting board. The move can include special cases such as captures, 
    /// pawn promotions, castling, and en passant captures.
    /// 
    /// ### Panics
    /// The function will panic if the source and destination squares of the move are the same.
    pub fn make_move(&self, mv: Move) -> Board {
        let mut board: Board = self.clone();
        
        // Ensure the source and destination squares are different.
        assert_ne!(mv.get_src(), mv.get_dest());

        let src: Square = mv.get_src();
        let dest: Square = mv.get_dest();
        let piece: Piece = self.piece_on(src);
        let piece_type = piece.piece_type();
        let move_type: MoveType = mv.get_type();
        let is_capture: bool = mv.is_capture();

        // Remove the piece from its source square
        board.remove_piece(src);

        // Update fifty-move rule counter
        board.fifty_move = if is_capture || piece_type == PieceType::Pawn { 0 } else { board.fifty_move + 1 };

        // Handle special move types (En Passant, Castling, Captures)
        match move_type {
            MoveType::EnPassant => {
                board.remove_piece(dest.forward(!self.side));
            },
            MoveType::KingCastle | MoveType::QueenCastle => {
                let rook: Piece = Piece::new(PieceType::Rook, self.side);
                let (rook_src, rook_dest) = get_rook_castling(dest);
                board.remove_piece(rook_src);
                board.set_piece(rook, rook_dest);
            },
            _ if is_capture => {
                board.remove_piece(dest);
            },
            _ => {},
        }

        // Handle promotions or move the piece to its destination
        if mv.is_promotion() {
            board.set_piece(mv.get_prom(self.side), dest);
        } else {
            board.set_piece(piece, dest);
        }

        // Update en passant square and Zobrist hash
        if let Some(square) = self.enpassant_square {
            board.enpassant_square = None;
            board.zobrist.hash_enpassant(square);
        }

        if move_type == MoveType::DoublePawn {
            let enpassant_target = src.forward(self.side);
            board.enpassant_square = Some(enpassant_target);
            board.zobrist.hash_enpassant(enpassant_target);
        }

        // Update castling rights and Zobrist hash
        let new_castling_rights: CastleRights = self.castling.update(src, dest);
        board.castling = new_castling_rights;
        board.zobrist.swap_castle_hash(self.castling, new_castling_rights);

        // Toggle side to move and update Zobrist hash
        board.side = !self.side;
        board.zobrist.hash_side();

        // Recalculate checkers for the new board state
        board.checkers = board.checkers();

        // Return the updated board
        board
    }

    /// Executes a null move, switching the turn to the opponent without making any actual moves.
    /// 
    /// This function is useful for certain algorithms where you want to evaluate a position
    /// as if the current player passed their turn. It asserts that the current player is not in check
    /// before performing the null move. The function will reset the en passant square and clear any checkers
    /// on the board.
    /// 
    /// ### Panics
    /// This function will panic if the current player's checkers are not empty, indicating that the
    /// game state is invalid for performing a null move.
    pub fn null_move(&self) -> Board {
        assert!(self.checkers.is_empty());

        let mut board: Board = self.clone();
        board.side = !self.side;
        board.zobrist.hash_side();

        board.enpassant_square = None;
        if let Some(square) = self.enpassant_square {
            board.zobrist.hash_enpassant(square);
        }

        board.checkers = BitBoard::EMPTY;

        board
    }
}

#[test]
fn test_board(){
    let board: Board = Board::new();
    println!("{}", board);
}

#[test]
fn test_from_string(){
    let board: Board = Board::from_str("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1").unwrap();
    println!("{}", board);
    println!("{}", board.to_fen());
}

#[test]
fn test_default(){
    let board: Board = Board::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    let board_default: Board = Board::default();
    println!("{}", board);
    assert_eq!(board, board_default);
}

#[test]
fn test_move(){
    let board: Board = Board::default();
    let mv: Move = Move::new(Square::G1, Square::F3, MoveType::Quiet);
    let board: Board = board.make_move(mv);
    println!("{}", board);
    let mv: Move = Move::new(Square::E7, Square::E6, MoveType::Quiet);
    let board: Board = board.make_move(mv);
    println!("{}", board);
    let mv: Move = Move::new(Square::B1, Square::C3, MoveType::Quiet);
    let board: Board = board.make_move(mv);
    println!("{}", board);
}
