use crate::{board::BoardBackend, located_piece::LocatedPiece, move_generator::{bishop, rook}, ply::Ply};

pub fn pseudo_legal_moves(what_to_move: LocatedPiece, backend: &impl BoardBackend) -> Vec<Ply> {
    let mut add_rook_and_bishop = Vec::new();

    add_rook_and_bishop.append(
        &mut rook::pseudo_legal_moves(what_to_move, backend)
    );

    add_rook_and_bishop.append(
        &mut bishop::pseudo_legal_moves(what_to_move, backend)
    );

    add_rook_and_bishop
}
