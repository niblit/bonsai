use bonsai_chess::prelude::{Piece, Team};
use leptos::prelude::*;

#[component]
pub fn PieceView(piece: Piece) -> impl IntoView {
    let team_str = match piece.team() {
        Team::White => "w",
        Team::Black => "b",
    };

    let kind_str = piece.kind().to_string();

    let src = format!("/static/pieces/california/{team_str}{kind_str}.svg");
    let alt_txt = format!("{:?} {:?}", piece.team(), piece.kind());

    view! { <img alt=alt_txt src=src class="w-[80%] h-[80%] z-10 pointer-events-none" /> }
}
