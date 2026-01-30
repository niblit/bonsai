use bonsai_chess::prelude::{Kind, Piece, Team};
use leptos::prelude::*;

#[component]
pub fn PieceView(piece: Piece) -> impl IntoView {
    let team_str = match piece.team() {
        Team::White => "w",
        Team::Black => "b",
    };
    let kind_str = match piece.kind() {
        Kind::Pawn => "P",
        Kind::Knight => "N",
        Kind::Bishop => "B",
        Kind::Rook => "R",
        Kind::Queen => "Q",
        Kind::King => "K",
    };
    let src = format!("/static/pieces/california/{team_str}{kind_str}.svg");

    view! { <img src=src class="w-[80%] h-[80%] z-10 pointer-events-none" /> }
}
