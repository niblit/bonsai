use bonsai_chess::prelude::*;
use leptos::prelude::*;

use crate::components::{Board, PromotionModal, Sidebar};

#[component]
pub fn App() -> impl IntoView {
    // --- STATE ---
    let (game, set_game) = signal(BoardFrontend::from_starting_position());
    let (selected_square, set_selected_square) = signal::<Option<Coordinates>>(None);
    let (history_list, set_history_list) = signal::<Vec<String>>(Vec::new());
    let (pending_promotion, set_pending_promotion) = signal::<Option<Ply>>(None);

    // --- LOGIC ---
    let valid_targets = Memo::new(move |_| {
        selected_square.get().map_or_else(Vec::new, |sel| {
            let mut game_state = game.get();
            game_state
                .get_legal_moves()
                .into_iter()
                .filter(|m| m.starting_square() == sel)
                .map(|m| m.ending_square())
                .collect::<Vec<Coordinates>>()
        })
    });

    let on_undo = move || {
        set_game.update(BoardFrontend::undo_last_move);
        set_history_list.update(|h| {
            h.pop();
        });
        set_selected_square.set(None);
    };

    let handle_square_click = move |(row, col): (usize, usize)| {
        let last_click = Coordinates::new(row, col).unwrap();
        let turn = game.with(BoardFrontend::turn);

        if let Some(selected) = selected_square.get() {
            if selected == last_click {
                set_selected_square.set(None);
                return;
            }

            let mut current_game_clone = game.get();
            let matching_move = current_game_clone
                .get_legal_moves()
                .iter()
                .find(|m| m.starting_square() == selected && m.ending_square() == last_click)
                .copied();

            if let Some(ply) = matching_move {
                if let Some(SpecialMove::Promotion(_)) = ply.special_move() {
                    set_pending_promotion.set(Some(ply));
                } else {
                    // Standard move execution
                    set_game.update(|game_state| game_state.make_move(&ply));
                    let move_str = format!(
                        "{}{}",
                        selected.to_algebraic_notation(),
                        last_click.to_algebraic_notation()
                    );
                    set_history_list.update(|h| h.push(move_str));
                    set_selected_square.set(None);
                }
            } else {
                let clicked_piece = game.with(|g| g.backend().get(last_click));
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
        } else {
            let clicked_piece = game.with(|g| g.backend().get(last_click));
            if let Some(piece) = clicked_piece
                && piece.team() == turn
            {
                set_selected_square.set(Some(last_click));
            }
        }
    };

    // Add a helper to finish the promotion:
    let on_promote = move |choice: ValidPromotions| {
        if let Some(base_ply) = pending_promotion.get() {
            let mut current_game = game.get();
            // Find the specific ply that matches the chosen promotion
            let final_ply = current_game.get_legal_moves().into_iter().find(|m| {
                m.starting_square() == base_ply.starting_square()
                    && m.ending_square() == base_ply.ending_square()
                    && matches!(m.special_move(), Some(SpecialMove::Promotion(c)) if c == choice)
            });

            if let Some(ply) = final_ply {
                set_game.update(|g| g.make_move(&ply));
                // ... update history list ...
            }
            set_pending_promotion.set(None);
            set_selected_square.set(None);
        }
    };

    // Add a cancel handler
    let on_cancel_promotion = move |()| {
        set_pending_promotion.set(None);
        set_selected_square.set(None); // also deselect the square
    };

    // --- VIEW ---
    view! {
        <div class="min-h-dvh w-screen bg-zinc-900 flex items-center justify-center text-zinc-100 font-sans py-8 md:py-0">
            <div class="flex flex-col md:flex-row gap-8 items-center justify-center">
                <Board
                    game=game
                    selected_square=selected_square
                    valid_targets=valid_targets
                    // Use Callback::new() here
                    on_square_click=Callback::new(handle_square_click)
                />

                <Sidebar
                    game=game
                    history_list=history_list
                    // Use Callback::new() here
                    on_undo=Callback::new(move |()| on_undo())
                />
            </div>

            {move || pending_promotion.get().map(|_| {
                view! {
                    <PromotionModal
                        team=game.with(bonsai_chess::prelude::BoardFrontend::turn)
                        on_select=Callback::new(on_promote)
                        on_cancel=Callback::new(on_cancel_promotion)
                    />
                }
            })}
        </div>
    }
}
