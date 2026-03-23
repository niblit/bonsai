// src/app.rs
use bonsai_chess::prelude::*;
use leptos::prelude::*;

use crate::{
    components::Board, components::PromotionModal, components::Sidebar, engine::use_engine,
    state::GameState,
};

#[component]
pub fn App() -> impl IntoView {
    let state = GameState::new();

    use_engine(state.game, state.set_game);

    view! {
        <main role="main">
            <a
                href="https://github.com/niblit/bonsai"
                target="_blank"
                rel="noopener noreferrer"
                class="fixed top-4 right-6 text-zinc-500 hover:text-white transition-colors duration-200 z-50"
                aria-label="View source on GitHub"
            >
                <svg viewBox="0 0 16 16" width="28" height="28" fill="currentColor" aria-hidden="true">
                    <path d="M8 0c4.42 0 8 3.58 8 8a8.013 8.013 0 0 1-5.45 7.59c-.4.08-.55-.17-.55-.38 0-.27.01-1.13.01-2.2 0-.75-.25-1.23-.54-1.48 1.78-.2 3.65-.88 3.65-3.95 0-.88-.31-1.59-.82-2.15.08-.2.36-1.02-.08-2.12 0 0-.67-.22-2.2.82-.64-.18-1.32-.27-2-.27-.68 0-1.36.09-2 .27-1.53-1.03-2.2-.82-2.2-.82-.44 1.1-.16 1.92-.08 2.12-.51.56-.82 1.28-.82 2.15 0 3.06 1.86 3.75 3.64 3.95-.23.2-.44.55-.51 1.07-.46.21-1.61.55-2.33-.66-.15-.24-.6-.83-1.23-.82-.67.01-.27.38.01.53.34.19.73.9.82 1.13.16.45.68 1.31 2.69.94 0 .67.01 1.3.01 1.49 0 .21-.15.45-.55.38A7.995 7.995 0 0 1 0 8c0-4.42 3.58-8 8-8Z"></path>
                </svg>
            </a>

            <div class="min-h-dvh w-screen bg-zinc-900 flex items-center justify-center text-zinc-100 font-sans py-8 md:py-0">
                <div class="flex flex-col md:flex-row gap-8 items-center justify-center">
                    <Board
                        game=state.game
                        selected_square=state.selected_square
                        valid_targets=state.valid_targets
                        on_square_click=Callback::new(move |(r, c)| state.handle_square_click(r, c))
                        outcome=state.outcome
                    />

                    <Sidebar
                        game=state.game
                        history_list=state.move_log
                        fen=state.fen
                        on_undo=Callback::new(move |()| state.undo())
                        on_restart=Callback::new(move |()| state.restart())
                        outcome=state.outcome
                    />
                </div>

                {move || {
                    state
                        .pending_promotion
                        .get()
                        .map(|_| {
                            view! {
                                <PromotionModal
                                    team=state.game.with(BoardFrontend::turn)
                                    on_select=Callback::new(move |c| state.promote(c))
                                    on_cancel=Callback::new(move |()| state.cancel_promotion())
                                />
                            }
                        })
                }}
            </div>
        </main>
    }
}
