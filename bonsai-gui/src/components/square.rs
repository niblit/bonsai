use super::piece::PieceView;
use bonsai_chess::prelude::*;
use leptos::prelude::*;

#[component]
pub fn Square(
    row: usize,
    col: usize,
    game: ReadSignal<BoardFrontend>,
    selected_square: ReadSignal<Option<Coordinates>>,
    valid_targets: Memo<Vec<Coordinates>>,
    on_click: Callback<(usize, usize)>, // Changed from Box<dyn Fn...>
) -> impl IntoView {
    let coords = Coordinates::new(row, col).unwrap();

    // Derived signals for UI state
    let is_selected = move || selected_square.get() == Some(coords);
    let is_valid_target = move || valid_targets.get().contains(&coords);

    // Background Color Logic
    let bg_color = if (row + col).is_multiple_of(2) {
        "bg-[#f0d9b5]"
    } else {
        "bg-[#b58863]"
    };

    // Check Logic
    let is_in_check = move || {
        game.with(|game_state| {
            game_state.backend().get(coords).is_some_and(|p| {
                p.kind() == Kind::King && p.team() == game_state.turn() && game_state.is_in_check()
            })
        })
    };

    // get last move played
    let highlight_last_move = move || {
        game.with(|g| {
            g.get_move_log().last().map_or("", |last_ply| {
                if coords == last_ply.starting_square() || coords == last_ply.ending_square() {
                    // Added mix-blend-mode or lower opacity so you can still see the square color
                    "absolute inset-0 bg-blue-500/50"
                } else {
                    ""
                }
            })
        })
    };

    view! {
        <div
            class=move || {
                format!(
                    "w-full h-full flex items-center justify-center relative cursor-pointer {} {}",
                    bg_color,
                    if is_selected() { "ring-inset ring-5 ring-[#33cd63]" } else { "" },
                )
            }
            // Use .run() for Callbacks
            on:click=move |_| on_click.run((row, col))
        >
            // Red Check Highlight
            {move || {
                if is_in_check() {
                    view! { <div class="absolute inset-0 bg-[#ea4865] rounded-full blur-md"></div> }
                        .into_any()
                } else {
                    view! { <div></div> }.into_any()
                }
            }}

            <div class=highlight_last_move></div>

            // Render Piece
            {move || {
                let piece = game.with(|game_state| game_state.backend().get(coords));
                piece
                    .map_or_else(
                        || view! { <div></div> }.into_any(),
                        |p| view! { <PieceView piece=p /> }.into_any(),
                    )
            }}

            // Valid Move Target Dot/Ring
            {move || {
                let is_target = is_valid_target();
                let has_piece = game.with(|g| g.backend().get(coords).is_some());
                if is_target {
                    let style = if has_piece {
                        "w-full h-full border-4 border-[#ea4865] rounded-full"
                    } else {
                        "w-[50%] h-[50%] bg-[#33cd63] rounded-full"
                    };

                    view! {
                        <div class=format!("absolute z-20 pointer-events-none {}", style)></div>
                    }
                        .into_any()
                } else {
                    view! { <div></div> }.into_any()
                }
            }}
        </div>
    }
}
