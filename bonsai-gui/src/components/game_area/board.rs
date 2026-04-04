use bonsai_chess::prelude::*;
use leptos::prelude::*;

use crate::components::game_area::Square;

#[component]
pub fn Board(
    game: ReadSignal<Game>,
    selected_square: ReadSignal<Option<Coordinate>>,
    valid_targets: Memo<Vec<Coordinate>>,
    outcome: Memo<Option<Outcome>>,
    on_square_click: Callback<(usize, usize)>,
) -> impl IntoView {
    view! {
      <div class="select-none">
        <div
          class="grid grid-cols-8 grid-rows-8 border-4 border-zinc-700 bg-zinc-800 shadow-2xl aspect-square rounded-xl overflow-hidden w-[80vmin] h-[80vmin] transition-all duration-700"
          class:blur-xs=move || outcome.get().is_some()
          class:brightness-80=move || outcome.get().is_some()
          class:opacity-80=move || outcome.get().is_some()
          class:pointer-events-none=move || outcome.get().is_some()
        >
          {BOARD_ROWS_RANGE
            .map(|row_idx| {
              view! {
                {BOARD_COLUMNS_RANGE
                  .map(|col_idx| {
                    view! {
                      <Square
                        row=row_idx
                        col=col_idx
                        game=game
                        selected_square=selected_square
                        valid_targets=valid_targets
                        on_click=on_square_click
                      />
                    }
                  })
                  .collect_view()}
              }
            })
            .collect_view()}
        </div>
      </div>
    }
}
