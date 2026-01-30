//! Full fledged lexer and parser for the Forsyth-Edwards Notation
//! Source: [Forsyth-Edwards Notation](https://www.chessprogramming.org/Forsyth-Edwards_Notation)
//
// <FEN> ::=  <Piece Placement>
//     ' ' <Side to move>
//     ' ' <Castling ability>
//     ' ' <En passant target square>
//     ' ' <Halfmove clock>
//     ' ' <Fullmove counter>
//
// <Piece Placement> ::= <rank8>'/'<rank7>'/'<rank6>'/'<rank5>'/'<rank4>'/'<rank3>'/'<rank2>'/'<rank1>
// <ranki>       ::= [<digit17>]<piece> {[<digit17>]<piece>} [<digit17>] | '8'
// <piece>       ::= <white Piece> | <black Piece>
// <digit17>     ::= '1' | '2' | '3' | '4' | '5' | '6' | '7'
// <white Piece> ::= 'P' | 'N' | 'B' | 'R' | 'Q' | 'K'
// <black Piece> ::= 'p' | 'n' | 'b' | 'r' | 'q' | 'k'
//
// <Side to move> ::= {'w' | 'b'}
//
// <Castling ability> ::= '-' | ['K'] ['Q'] ['k'] ['q'] (1..4)
//
// <En passant target square> ::= '-' | <epsquare>
// <epsquare>   ::= <fileLetter> <eprank>
// <fileLetter> ::= 'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h'
// <eprank>     ::= '3' | '6'
//
// <Halfmove Clock> ::= <digit> {<digit>}
// <digit> ::= '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'
//
// <Fullmove counter> ::= <digit19> {<digit>}
// <digit19> ::= '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'
// <digit>   ::= '0' | <digit19>

use crate::{
    atoms::{CastlingRights, Coordinates, MoveCounter, Team},
    board::{Grid, PositionSnapshot},
    moves::CastlingSide,
    pieces::{Kind, Piece},
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum FenToken {
    WhiteSpace,

    // Piece Placement
    EmptySquares(usize),
    Piece(Piece),
    RankSeparator,

    // Side to move
    SideToMove(Team),

    // Castling
    NoCastling,
    CastlingEnabled(Team, CastlingSide),

    // En Passant
    NoEnPassant,
    EnPassantFile(char),
    EnPassantRank(char),

    // Halfmove Clock
    Halfmove(usize),

    // Fullmove Clock
    Fullmove(usize),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum FenParsingError {
    UnexpectedEndOfInput,
    InvalidPiecePlacement(String),
    InvalidSideToMove(String),
    InvalidCastlingRights(String),
    InvalidEnPassant(String),
    InvalidClock(String),
    UnexpectedToken(String),
}

impl std::fmt::Display for FenParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnexpectedEndOfInput => write!(f, "Unexpected end of FEN string"),
            Self::InvalidPiecePlacement(s) => write!(f, "Invalid piece placement: {s}"),
            Self::InvalidSideToMove(s) => write!(f, "Invalid side to move: {s}"),
            Self::InvalidCastlingRights(s) => write!(f, "Invalid castling rights: {s}"),
            Self::InvalidEnPassant(s) => write!(f, "Invalid en passant target: {s}"),
            Self::InvalidClock(s) => write!(f, "Invalid clock format: {s}"),
            Self::UnexpectedToken(s) => write!(f, "Unexpected token: {s}"),
        }
    }
}

impl std::error::Error for FenParsingError {}

/// Generates a FEN string from a `PositionSnapshot`.
///
/// Since `PositionSnapshot` does not store move counters, this function
/// defaults the Halfmove clock and Fullmove counter to "0 1".
#[must_use]
pub fn to_fen(position: PositionSnapshot, clocks: &MoveCounter) -> String {
    let mut fen = String::new();

    // 1. Piece Placement
    for (row_index, row_contents) in position.get_grid().iter().enumerate() {
        let mut empty_count = 0;
        for square in row_contents {
            match square {
                Some(piece) => {
                    if empty_count > 0 {
                        fen.push_str(&empty_count.to_string());
                        empty_count = 0;
                    }
                    fen.push_str(&piece.to_string());
                }
                None => empty_count += 1,
            }
        }
        if empty_count > 0 {
            fen.push_str(&empty_count.to_string());
        }
        if row_index < 7 {
            fen.push('/');
        }
    }

    fen.push(' ');

    // 2. Side to move
    match position.get_turn() {
        Team::White => fen.push('w'),
        Team::Black => fen.push('b'),
    }

    fen.push(' ');

    // 3. Castling
    let rights = position.get_castling_rights();
    let mut castling_str = String::new();
    if rights.white_king_side() {
        castling_str.push('K');
    }
    if rights.white_queen_side() {
        castling_str.push('Q');
    }
    if rights.black_king_side() {
        castling_str.push('k');
    }
    if rights.black_queen_side() {
        castling_str.push('q');
    }
    if castling_str.is_empty() {
        fen.push('-');
    } else {
        fen.push_str(&castling_str);
    }

    fen.push(' ');

    // 4. En Passant
    match position.get_en_passant() {
        Some(coord) => fen.push_str(&coord.to_algebraic_notation()),
        None => fen.push('-'),
    }

    fen.push(' ');

    // 5. Clocks
    let halfmove = &clocks.fifty_move_rule_counter().to_string();
    fen.push_str(halfmove);

    fen.push(' ');

    let fullmove = &clocks.fullmove().to_string();
    fen.push_str(fullmove);

    fen
}

/// Parses a FEN string into a `PositionSnapshot` and the associated `MoveCounter`.
#[must_use]
pub fn from_fen(fen: &str) -> Result<(PositionSnapshot, MoveCounter), FenParsingError> {
    let mut lexer = Lexer::new(fen);

    let mut grid = [[None; 8]; 8];
    let mut turn = Team::White;
    let mut castling = CastlingRights::no_rights();
    let mut en_passant = None;
    let mut halfmove_clock = 0;
    let mut fullmove_number = 1;

    // 1. Piece Placement
    let mut row = 0;
    let mut col = 0;
    loop {
        match lexer.next_token() {
            Some(FenToken::Piece(p)) => {
                if row > 7 || col > 7 {
                    return Err(FenParsingError::InvalidPiecePlacement(
                        "Out of bounds".to_string(),
                    ));
                }
                grid[row][col] = Some(p);
                col += 1;
            }
            Some(FenToken::EmptySquares(n)) => {
                if row > 7 || col + n > 8 {
                    return Err(FenParsingError::InvalidPiecePlacement(
                        "Row overflow".to_string(),
                    ));
                }
                col += n;
            }
            Some(FenToken::RankSeparator) => {
                if col != 8 {
                    return Err(FenParsingError::InvalidPiecePlacement(format!(
                        "Row {row} incomplete (width {col})"
                    )));
                }
                row += 1;
                col = 0;
            }
            Some(FenToken::WhiteSpace) => {
                if row != 7 || col != 8 {
                    return Err(FenParsingError::InvalidPiecePlacement(
                        "Board incomplete".to_string(),
                    ));
                }
                break;
            }
            Some(t) => {
                return Err(FenParsingError::UnexpectedToken(format!(
                    "{t:?} in Piece Placement"
                )));
            }
            None => return Err(FenParsingError::UnexpectedEndOfInput),
        }
    }

    // 2. Side to Move
    match lexer.next_token() {
        Some(FenToken::SideToMove(Team::White)) => turn = Team::White,
        Some(FenToken::SideToMove(Team::Black)) => turn = Team::Black,
        Some(t) => {
            return Err(FenParsingError::UnexpectedToken(format!(
                "{t:?} in Side to Move"
            )));
        }
        None => return Err(FenParsingError::UnexpectedEndOfInput),
    }

    // Expect Space
    match lexer.next_token() {
        Some(FenToken::WhiteSpace) => {}
        Some(t) => {
            return Err(FenParsingError::UnexpectedToken(format!(
                "{t:?} expected whitespace after side"
            )));
        }
        None => return Err(FenParsingError::UnexpectedEndOfInput),
    }

    // 3. Castling Rights
    loop {
        match lexer.next_token() {
            Some(FenToken::NoCastling) => {
                // Just continue to space
            }
            Some(FenToken::CastlingEnabled(team, side)) => match (team, side) {
                (Team::White, CastlingSide::Short) => castling.enable_white_king_side(),
                (Team::White, CastlingSide::Long) => castling.enable_white_queen_side(),
                (Team::Black, CastlingSide::Short) => castling.enable_black_king_side(),
                (Team::Black, CastlingSide::Long) => castling.enable_black_queen_side(),
            },
            Some(FenToken::WhiteSpace) => break,
            Some(t) => {
                return Err(FenParsingError::UnexpectedToken(format!(
                    "{t:?} in Castling Rights"
                )));
            }
            None => return Err(FenParsingError::UnexpectedEndOfInput),
        }
    }

    // 4. En Passant
    match lexer.next_token() {
        Some(FenToken::NoEnPassant) => en_passant = None,
        Some(FenToken::EnPassantFile(file)) => {
            if let Some(FenToken::EnPassantRank(rank_char)) = lexer.next_token() {
                let file_idx = match file {
                    'a' => 0,
                    'b' => 1,
                    'c' => 2,
                    'd' => 3,
                    'e' => 4,
                    'f' => 5,
                    'g' => 6,
                    'h' => 7,
                    _ => {
                        return Err(FenParsingError::InvalidEnPassant(
                            "Invalid file char".to_string(),
                        ));
                    }
                };

                let rank_val = rank_char.to_digit(10).unwrap() as usize;
                // FEN rank '3' -> index 5 (8-3)
                // FEN rank '6' -> index 2 (8-6)
                let row_idx = 8 - rank_val;

                en_passant = Coordinates::new(row_idx, file_idx);
            } else {
                return Err(FenParsingError::InvalidEnPassant(
                    "Missing rank after file".to_string(),
                ));
            }
        }
        Some(t) => {
            return Err(FenParsingError::UnexpectedToken(format!(
                "{t:?} in En Passant"
            )));
        }
        None => return Err(FenParsingError::UnexpectedEndOfInput),
    }

    // 5. Halfmove Clock
    if let Some(token) = lexer.next_token() {
        if token == FenToken::WhiteSpace {
            if let Some(FenToken::Halfmove(val)) = lexer.next_token() {
                halfmove_clock = val;
            } else {
                // Strict FEN requires it.
                return Err(FenParsingError::InvalidClock(
                    "Expected halfmove clock".into(),
                ));
            }
        } else {
            return Err(FenParsingError::UnexpectedToken(format!(
                "{token:?} expected whitespace before halfmove"
            )));
        }
    }

    // 6. Fullmove Counter
    if let Some(token) = lexer.next_token() {
        if token == FenToken::WhiteSpace {
            if let Some(FenToken::Fullmove(val)) = lexer.next_token() {
                fullmove_number = val;
            } else {
                return Err(FenParsingError::InvalidClock(
                    "Expected fullmove counter".into(),
                ));
            }
        } else {
            return Err(FenParsingError::UnexpectedToken(format!(
                "{token:?} expected whitespace before fullmove"
            )));
        }
    }

    // Calculate total halfmoves played
    // Formula: (Fullmove - 1) * 2 + (1 if Black to move else 0)
    let total_halfmoves =
        (fullmove_number.saturating_sub(1) * 2) + usize::from(turn == Team::Black);

    let move_counter = MoveCounter::from(halfmove_clock, total_halfmoves, fullmove_number);
    let position = PositionSnapshot::new(Grid::new(grid), turn, castling, en_passant);

    Ok((position, move_counter))
}

pub struct Lexer<'a> {
    input: std::iter::Peekable<std::str::Chars<'a>>,
    // 0: Board, 1: Side, 2: Castling, 3: EP, 4: Half, 5: Full
    current_field: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(fen: &'a str) -> Self {
        Self {
            input: fen.chars().peekable(),
            current_field: 0,
        }
    }

    fn next_token(&mut self) -> Option<FenToken> {
        // Peek to see if we have a space (field separator)
        if matches!(self.input.peek(), Some(' ')) {
            self.input.next(); // Consume space
            self.current_field += 1;
            return Some(FenToken::WhiteSpace);
        }

        match self.current_field {
            0 => self.lex_piece_placement(),
            1 => self.lex_side_to_move(),
            2 => self.lex_castling(),
            3 => self.lex_en_passant(),
            4 => self.lex_halfmove_clock(),
            5 => self.lex_fullmove_counter(),
            _ => None,
        }
    }

    fn lex_piece_placement(&mut self) -> Option<FenToken> {
        let c = self.input.next()?;
        match c {
            '/' => Some(FenToken::RankSeparator),
            '1'..='8' => Some(FenToken::EmptySquares(c.to_digit(10).unwrap() as usize)),
            'P' => Some(FenToken::Piece(Piece::new(Team::White, Kind::Pawn))),
            'N' => Some(FenToken::Piece(Piece::new(Team::White, Kind::Knight))),
            'B' => Some(FenToken::Piece(Piece::new(Team::White, Kind::Bishop))),
            'R' => Some(FenToken::Piece(Piece::new(Team::White, Kind::Rook))),
            'Q' => Some(FenToken::Piece(Piece::new(Team::White, Kind::Queen))),
            'K' => Some(FenToken::Piece(Piece::new(Team::White, Kind::King))),
            'p' => Some(FenToken::Piece(Piece::new(Team::Black, Kind::Pawn))),
            'n' => Some(FenToken::Piece(Piece::new(Team::Black, Kind::Knight))),
            'b' => Some(FenToken::Piece(Piece::new(Team::Black, Kind::Bishop))),
            'r' => Some(FenToken::Piece(Piece::new(Team::Black, Kind::Rook))),
            'q' => Some(FenToken::Piece(Piece::new(Team::Black, Kind::Queen))),
            'k' => Some(FenToken::Piece(Piece::new(Team::Black, Kind::King))),
            _ => None,
        }
    }

    fn lex_side_to_move(&mut self) -> Option<FenToken> {
        let c = self.input.next()?;
        match c {
            'w' => Some(FenToken::SideToMove(Team::White)),
            'b' => Some(FenToken::SideToMove(Team::Black)),
            _ => None,
        }
    }

    fn lex_castling(&mut self) -> Option<FenToken> {
        let c = self.input.next()?;
        match c {
            '-' => Some(FenToken::NoCastling),
            'K' => Some(FenToken::CastlingEnabled(Team::White, CastlingSide::Short)),
            'Q' => Some(FenToken::CastlingEnabled(Team::White, CastlingSide::Long)),
            'k' => Some(FenToken::CastlingEnabled(Team::Black, CastlingSide::Short)),
            'q' => Some(FenToken::CastlingEnabled(Team::Black, CastlingSide::Long)),
            _ => None,
        }
    }

    fn lex_en_passant(&mut self) -> Option<FenToken> {
        let c = self.input.next()?;
        match c {
            'a'..='h' => Some(FenToken::EnPassantFile(c)),
            '3' | '6' => Some(FenToken::EnPassantRank(c)), // Return char here, process in parser
            '-' => Some(FenToken::NoEnPassant),
            _ => None,
        }
    }

    fn lex_halfmove_clock(&mut self) -> Option<FenToken> {
        let mut num_str = String::new();
        while let Some(&c) = self.input.peek() {
            if c.is_ascii_digit() {
                num_str.push(c);
                self.input.next();
            } else {
                break;
            }
        }
        if num_str.is_empty() {
            None
        } else {
            Some(FenToken::Halfmove(num_str.parse().unwrap()))
        }
    }

    fn lex_fullmove_counter(&mut self) -> Option<FenToken> {
        let mut num_str = String::new();
        while let Some(&c) = self.input.peek() {
            if c.is_ascii_digit() {
                num_str.push(c);
                self.input.next();
            } else {
                break;
            }
        }
        if num_str.is_empty() {
            None
        } else {
            Some(FenToken::Fullmove(num_str.parse().unwrap()))
        }
    }
}
