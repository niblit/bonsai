use bonsai_chess::prelude::{Game, Side};
use leptos::prelude::*;

use crate::components::layout::undo::Undo;

#[component]
pub fn Controls(game: ReadSignal<Game>, on_undo: Callback<()>) -> impl IntoView {
    view! {
      <div class="bg-zinc-800 p-4 rounded-lg shadow-lg border border-zinc-700">
        <div class="flex items-center gap-2 mb-4">
          <span class="text-zinc-400">To Move:</span>
          <span class=move || {
            format!(
              "font-bold {}",
              match game.with(Game::turn) {
                Side::White => "text-white",
                Side::Black => "text-zinc-400",
              },
            )
          }>{move || format!("{:?}", game.with(Game::turn))}</span>
        </div>

        <Undo on_undo=on_undo />
      </div>
    }
}
