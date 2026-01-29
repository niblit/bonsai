//! Full fledged lexer and parser for the Forsyth-Edwards Notation
//! Source: [Forsyth-Edwards Notation](https://www.chessprogramming.org/Forsyth-Edwards_Notation)

use crate::{atoms::Team, moves::CastlingSide, pieces::Piece};

/// <FEN> ::=  <Piece Placement>
///     ' ' <Side to move>
///     ' ' <Castling ability>
///     ' ' <En passant target square>
///     ' ' <Halfmove clock>
///     ' ' <Fullmove counter>

/// <Piece Placement> ::= <rank8>'/'<rank7>'/'<rank6>'/'<rank5>'/'<rank4>'/'<rank3>'/'<rank2>'/'<rank1>
/// <ranki>       ::= [<digit17>]<piece> {[<digit17>]<piece>} [<digit17>] | '8'
/// <piece>       ::= <white Piece> | <black Piece>
/// <digit17>     ::= '1' | '2' | '3' | '4' | '5' | '6' | '7'
/// <white Piece> ::= 'P' | 'N' | 'B' | 'R' | 'Q' | 'K'
/// <black Piece> ::= 'p' | 'n' | 'b' | 'r' | 'q' | 'k'

/// <Side to move> ::= {'w' | 'b'}

/// <Castling ability> ::= '-' | ['K'] ['Q'] ['k'] ['q'] (1..4)

/// <En passant target square> ::= '-' | <epsquare>
/// <epsquare>   ::= <fileLetter> <eprank>
/// <fileLetter> ::= 'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h'
/// <eprank>     ::= '3' | '6'

/// <Halfmove Clock> ::= <digit> {<digit>}
/// <digit> ::= '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'

/// <Fullmove counter> ::= <digit19> {<digit>}
/// <digit19> ::= '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'
/// <digit>   ::= '0' | <digit19>

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
    EnPassantRank(usize),
    
    // Halfmove Clock
    Halfmove(usize),
    
    // Fullmove Clock
    Fullmove(usize)
}

pub struct Lexer<'a> {
    input: std::str::Chars<'a>,

    // 0: Board, 1: Side, 2: Castling, 3: EP, 4: Half, 5: Full
    current_field: usize, 
}

impl<'a> Lexer<'a> {
    fn next_token(&mut self) -> Option<FenToken> {
        let c = self.input.peek()?;

        // If we hit a space, we advance the state (field index)
        if *c == ' ' {
            self.current_field += 1;
            self.input.next();
            return Some(FenToken::WhiteSpace);
        }

        match self.current_field {
            0 => self.lex_piece_placement(),
            1 => self.lex_side_side_to_move(),
            2 => self.lex_castling(),
            3 => self.lex_en_passant(),
            4 => self.lex_halfmove_token(),
            5 => self.lex_fullmove_token(),
            _ => None
        }
    }
    fn lex_piece_placement(&mut self) -> Option<FenToken> {
        todo!()
    }
    fn lex_side_side_to_move(&mut self) -> Option<FenToken> {
        todo!()
    }
    fn lex_castling(&mut self) -> Option<FenToken> {
        todo!()
    }
    fn lex_en_passant(&mut self) -> Option<FenToken> {
        let c = self.input.next()?;
        match c {
            'a'..='h' => Some(FenToken::EnPassantFile(c)),
            '3' | '6' => Some(FenToken::EnPassantRank(c.to_digit(10).unwrap() as usize)),
            '-' => Some(FenToken::NoEnPassant),
            _ => panic!("Invalid En Passant character"),
        }
    }
    fn lex_halfmove_token(&mut self) -> Option<FenToken> {
        todo!()
    }
    fn lex_fullmove_token(&mut self) -> Option<FenToken> {
        todo!()
    }
    
}