use bonsai_chess::prelude::*;
use leptos::prelude::*;

use crate::{
    components::{Board, PromotionModal, Sidebar, SourceCode},
    engine::{EngineRole, use_engine},
    state::GameState,
};

#[component]
pub fn App() -> impl IntoView {
    let (get_engine_role, set_engine_role) = signal(EngineRole::Black);

    let state = GameState::new(get_engine_role);

    use_engine(state.game, state.set_game, get_engine_role);

    view! {
      <main
        role="main"
        class="min-h-dvh w-screen bg-zinc-900 flex items-center justify-center text-zinc-100 font-sans py-8 md:py-0"
      >
        <SourceCode />

        <div class="flex flex-col md:flex-row gap-8 items-center justify-center">
          <Board
            game=state.game
            selected_square=state.selected_square
            valid_targets=state.valid_targets
            on_square_click=Callback::new(move |(r, c)| {
              state.handle_square_click(r, c);
            })
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
      </main>
    }
}
