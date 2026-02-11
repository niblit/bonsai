use std::time::Duration;

use leptos::prelude::*;

use bonsai_chess::prelude::*;
use bonsai_engine::best_move;

use crate::{
    components::{Board, GameOverModal, PromotionModal, Sidebar},
    feedback::provide_feedback,
};

#[component]
pub fn App() -> impl IntoView {
    // Main game state
    let (game, set_game) = signal(BoardFrontend::from_starting_position());

    // User click on board
    let (selected_square, set_selected_square) = signal::<Option<Coordinates>>(None);

    // Wether to ask for user input to choose the promotion piece
    let (pending_promotion, set_pending_promotion) = signal::<Option<Ply>>(None);

    // The current board position
    let current_fen = Memo::new(move |_| game.get().to_fen());

    // The move history
    let move_log = Memo::new(move |_| game.get().get_move_log());

    // Use the engine for black moves
    Effect::new(move |_| {
        let current_game = game.get();
        let turn = current_game.turn();
        let outcome = current_game.outcome();

        // If it is Black's turn and the game is not over, trigger the engine.
        if turn == Team::Black && outcome.is_none() {
            // We use set_timeout to let the browser repaint (update the board with the user's move)
            // before the engine blocks the main thread with its calculation.
            set_timeout(
                move || {
                    if let Some(engine_ply) = best_move(current_game.clone(), 4) {
                        set_game.update(|g| g.make_move(&engine_ply));
                        provide_feedback(&engine_ply);
                    }
                },
                Duration::from_millis(100),
            );
        }
    });

    // Compute the valid ending squares for the user selected piece
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

    // Undo the last two moves (the engine move and then white's move)
    let on_undo = move || {
        set_game.update(BoardFrontend::undo_last_move);
        set_game.update(BoardFrontend::undo_last_move);
        set_selected_square.set(None);
    };

    // When a user clicks on the board, handle the new click
    let handle_square_click = move |(row, col): (usize, usize)| {
        let last_click = Coordinates::new(row, col).unwrap();
        let turn = game.with(BoardFrontend::turn);

        // Disable interaction if it's the Engine's turn (Black)
        if turn == Team::Black {
            return;
        }

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
                    set_selected_square.set(None);
                } else {
                    // Standard move execution
                    set_game.update(|game_state| game_state.make_move(&ply));
                    set_selected_square.set(None);
                    provide_feedback(&ply);
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

            set_pending_promotion.set(None);
            set_selected_square.set(None);

            if let Some(ply) = final_ply {
                set_game.update(|g| g.make_move(&ply));
                provide_feedback(&ply);
            }
        }
    };

    // Add a cancel handler
    let on_cancel_promotion = move |()| {
        set_pending_promotion.set(None);
        set_selected_square.set(None); // also deselect the square
    };

    let on_restart = move || {
        set_game.set(BoardFrontend::from_starting_position());
        set_selected_square.set(None);
        set_pending_promotion.set(None);
    };

    // --- VIEW ---
    view! {
        <main role="main">
            <div class="min-h-dvh w-screen bg-zinc-900 flex items-center justify-center text-zinc-100 font-sans py-8 md:py-0">
                <div class="flex flex-col md:flex-row gap-8 items-center justify-center">
                    <Board
                        game=game
                        selected_square=selected_square
                        valid_targets=valid_targets
                        on_square_click=Callback::new(handle_square_click)
                    />

                    <Sidebar
                        game=game
                        history_list=move_log
                        fen=current_fen
                        on_undo=Callback::new(move |()| on_undo())
                    />
                </div>

                {move || {
                    pending_promotion
                        .get()
                        .map(|_| {
                            view! {
                                <PromotionModal
                                    team=game.with(bonsai_chess::prelude::BoardFrontend::turn)
                                    on_select=Callback::new(on_promote)
                                    on_cancel=Callback::new(on_cancel_promotion)
                                />
                            }
                        })
                }}

                {move || {
                    game.get()
                        .outcome()
                        .map(|outcome| {
                            view! {
                                <GameOverModal
                                    outcome=outcome
                                    on_restart=Callback::new(move |()| on_restart())
                                    on_undo=Callback::new(move |()| on_undo())
                                />
                            }
                        })
                }}
            </div>
        </main>
    }
}
