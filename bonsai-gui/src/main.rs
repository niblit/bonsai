#![allow(clippy::multiple_crate_versions)]
use leptos::prelude::*;

use bonsai_chess::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    // The full game state, from the starting position
    let (game, set_game) = signal(BoardFrontend::from_starting_position());

    // Keep track of user's select square
    let (selected_square, set_selected_square) = signal::<Option<Coordinates>>(None);

    // Keep a move log for the side panel
    // TODO: use the BoardFrontend::move_log
    let (history_list, set_history_list) = signal::<Vec<String>>(Vec::new());

    // When user wants to undo a move
    let on_undo = move |_| {
        set_game.update(bonsai_chess::prelude::BoardFrontend::undo_last_move);
        set_history_list.update(|h| {
            h.pop();
        });
        set_selected_square.set(None);
    };

    let handle_square_click = move |row: usize, col: usize| {
        let last_click = Coordinates::new(row, col).unwrap();

        // We need to access the game state to check legality
        // We use .with() to avoid cloning the whole board just to read a piece
        let turn = game.with(bonsai_chess::prelude::BoardFrontend::turn);

        // 1. If we have a selection, try to move or change selection
        if let Some(selected) = selected_square.get() {
            if selected == last_click {
                set_selected_square.set(None); // Deselect
                return;
            }

            // Check if this is a legal move
            // We strictly need a clone here because get_legal_moves mutates self temporarily
            let mut current_game_clone = game.get();
            let legal_moves = current_game_clone.get_legal_moves();

            let matching_move = legal_moves
                .iter()
                .find(|m| m.starting_square() == selected && m.ending_square() == last_click);

            if let Some(ply) = matching_move {
                // EXECUTE MOVE
                set_game.update(|game_state| game_state.make_move(ply));

                // Update Log
                let move_str = format!(
                    "{}{}",
                    selected.to_algebraic_notation(),
                    last_click.to_algebraic_notation()
                );
                set_history_list.update(|h| h.push(move_str));

                set_selected_square.set(None);
            } else {
                // Check if we clicked our own piece to switch selection
                let clicked_piece =
                    game.with(|game_state: &BoardFrontend| game_state.backend().get(last_click));
                if let Some(piece) = clicked_piece {
                    if piece.team() == turn {
                        set_selected_square.set(Some(last_click));
                    } else {
                        set_selected_square.set(None);
                    }
                } else {
                    set_selected_square.set(None);
                }
            }
        }
        // 2. If no selection, try to select a piece
        else {
            let clicked_piece = game.with(|game_state| game_state.backend().get(last_click));
            if let Some(piece) = clicked_piece
                && piece.team() == turn
            {
                set_selected_square.set(Some(last_click));
            }
        }
    };

    // Calculate valid targets for the currently selected piece
    let valid_targets = move || {
        selected_square.get().map_or_else(Vec::new, |sel| {
            let mut game_state = game.get();
            game_state
                .get_legal_moves()
                .into_iter()
                .filter(|m| m.starting_square() == sel)
                .map(|m| m.ending_square())
                .collect::<Vec<Coordinates>>()
        })
    };

    view! {
        <div class="h-dvh w-screen bg-zinc-900 flex items-center justify-center text-zinc-100 font-sans">
            <div class="flex flex-row gap-8 items-start">

                // --- CHESS BOARD ---
                <div class="select-none">

                    <div class="grid grid-cols-8 border-4 border-zinc-700 bg-zinc-800 shadow-2xl aspect-square rounded-xl overflow-hidden">
                        {
                            BOARD_ROWS_RANGE.map(|row_idx| {
                                view! {
                                    {
                                        BOARD_COLUMNS_RANGE.map(|col_idx| {
                                            let coords = Coordinates::new(row_idx, col_idx).unwrap();

                                            // State accessors
                                            let is_selected = move || selected_square.get() == Some(coords);
                                            let is_valid_target = move || valid_targets().contains(&coords);

                                            // BG Color
                                            let bg_color = if (row_idx + col_idx) % 2 == 0 {
                                                "bg-[#f0d9b5]"
                                            } else {
                                                "bg-[#b58863]"
                                            };

                                            view! {
                                                <div
                                                    class=move || format!(
                                                        "w-16 h-16 flex items-center justify-center relative cursor-pointer \
                                                         {} {}",
                                                        bg_color,
                                                        if is_selected() { "ring-inset ring-5 ring-[#33cd63]" } else { "" },
                                                    )
                                                    on:click=move |_| handle_square_click(row_idx, col_idx)
                                                >
                                                    // Check Highlight
                                                    {move || {
                                                        let in_check = game.with(|game_state| {
                                                            game_state.backend().get(coords).is_some_and(|p| p.kind() == Kind::King &&
                                                                p.team() == game_state.turn() && game_state.is_in_check())
                                                        });

                                                        if in_check {
                                                            view! { <div class="absolute inset-0 bg-[#ea4865] rounded-full blur-md"></div> }.into_any()
                                                        } else {
                                                            view! {<div></div>}.into_any()
                                                        }
                                                    }}

                                                    // Piece Rendering
                                                    {move || {
                                                        let piece = game.with(|game_state| game_state.backend().get(coords));

                                                        piece.map_or_else(|| view! {<div></div>}.into_any(), |p| {
                                                            let team_str = match p.team() { Team::White => "w", Team::Black => "b" };
                                                            let kind_str = match p.kind() {
                                                                Kind::Pawn => "P", Kind::Knight => "N", Kind::Bishop => "B",
                                                                Kind::Rook => "R", Kind::Queen => "Q", Kind::King => "K",
                                                            };
                                                            let src = format!("/static/pieces/california/{team_str}{kind_str}.svg");

                                                            view! {
                                                                <img src=src class="w-14 h-14 z-10 pointer-events-none" />
                                                            }.into_any()
                                                        })
                                                    }}

                                                    // Valid Move Dot
                                                    {move || {
                                                        let is_target = is_valid_target();
                                                        let has_piece = game.with(|g| g.backend().get(coords).is_some());

                                                        if is_target {
                                                            let style = if has_piece {
                                                                "w-16 h-16 border-4 border-[#ea4865] rounded-full"
                                                            } else {
                                                                "w-8 h-8 bg-[#33cd63] rounded-full"
                                                            };
                                                            view! { <div class=format!("absolute z-20 pointer-events-none {}", style)></div> }.into_any()
                                                        } else {
                                                            view! {<div></div>}.into_any()
                                                        }
                                                    }}
                                                </div>
                                            }
                                        }).collect_view()
                                    }
                                }
                            }).collect_view()
                        }
                    </div>
                </div>

                // --- SIDE PANEL ---
                <div class="w-64 flex flex-col gap-4 h-[512px]">
                    <div class="bg-zinc-800 p-4 rounded-lg shadow-lg border border-zinc-700">
                        <div class="flex items-center gap-2 mb-4">
                            <span class="text-zinc-400">To Move:</span>
                            <span class=move || format!("font-bold {}", match game.with(bonsai_chess::prelude::BoardFrontend::turn) {
                                Team::White => "text-white",
                                Team::Black => "text-zinc-400"
                            })>
                                {move || format!("{:?}", game.with(bonsai_chess::prelude::BoardFrontend::turn))}
                            </span>
                        </div>

                        <button
                            class="w-full py-2 px-4 bg-red-600 hover:bg-red-700 text-white rounded transition"
                            on:click=on_undo
                        >
                            "Undo Move"
                        </button>
                    </div>

                // MOVE LOG
                    <div class="flex-1 bg-zinc-800 p-4 rounded-lg shadow-lg border border-zinc-700 overflow-y-auto">
                        <h3 class="font-bold border-b border-zinc-600 pb-2 mb-2 sticky top-0 bg-zinc-800">History</h3>
                        <div class="flex flex-col gap-1 font-mono text-sm">
                            {move || {
                                let history = history_list.get();
                                // Process the history vector in chunks of 2 (White, Black)
                                history.chunks(2).enumerate().map(|(i, chunk)| {
                                    let white_move = chunk.first().cloned().unwrap_or_default();
                                    let black_move = chunk.get(1).cloned().unwrap_or_default();

                                    view! {
                                        // Grid or fixed width is better than flex-gap for alignment
                                        <div class="grid grid-cols-[3rem_1fr_1fr] border-b border-zinc-700/50 py-1">
                                            // Move Number (e.g., "1.-")
                                            <span class="text-zinc-500 select-none text-right pr-2">
                                                {format!("{}.-", i + 1)}
                                            </span>

                                            // White's Move
                                            <span class="text-zinc-200">{white_move}</span>

                                            // Black's Move (only renders if it exists)
                                            <span class="text-zinc-200">{black_move}</span>
                                        </div>
                                    }
                                }).collect_view()
                            }}
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
