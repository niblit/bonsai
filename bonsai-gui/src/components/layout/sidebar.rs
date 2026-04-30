use bonsai_chess::prelude::{Game, Outcome, Ply};
use leptos::prelude::*;

use crate::components::layout::controls::Controls;
use crate::components::layout::game_over::GameOver;
use crate::components::layout::history::HistoryLog;

#[component]
pub fn Sidebar(
    game: ReadSignal<Game>,
    history_list: Memo<Vec<Ply>>,
    fen: Memo<String>,
    outcome: Memo<Option<Outcome>>,
    on_undo: Callback<()>,
    on_restart: Callback<()>,
) -> impl IntoView {
    view! {
      <div class="w-[80vmin] md:w-64 flex flex-col gap-4 h-64 md:h-[80vmin]">
        {move || {
          outcome
            .get()
            .map_or_else(
              || view! { <Controls game=game on_undo=on_undo /> }.into_any(),
              |o| {
                view! {
                  <GameOver outcome=o on_restart=on_restart on_undo=on_undo />
                }
                  .into_any()
              },
            )
        }} <HistoryLog history_list=history_list game=game/>
        <div class="flex flex-col gap-1">
          <label class="text-xs text-zinc-400 font-bold uppercase tracking-wider">
            "FEN"
          </label>
          <div class="w-full bg-zinc-800 border border-zinc-700 text-zinc-300 text-xs p-2 rounded font-mono break-all whitespace-normal select-all">
            {fen}
          </div>
        </div>
      </div>
    }
}
