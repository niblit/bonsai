use crate::components::controls::Controls;
use crate::components::history::HistoryLog;
use bonsai_chess::prelude::BoardFrontend;
use leptos::prelude::*;

#[component]
pub fn Sidebar(
    game: ReadSignal<BoardFrontend>,
    history_list: ReadSignal<Vec<String>>,
    fen: Memo<String>,
    on_undo: Callback<()>,
) -> impl IntoView {
    view! {
        <div class="w-[80vmin] md:w-64 flex flex-col gap-4 h-64 md:h-[80vmin]">
            <Controls game=game on_undo=on_undo />

            <div class="flex flex-col gap-1">
                <label class="text-xs text-zinc-400 font-bold uppercase tracking-wider">
                    "FEN"
                </label>
                <div class="w-full bg-zinc-800 border border-zinc-700 text-zinc-300 text-xs p-2 rounded font-mono break-all whitespace-normal select-all">
                    {fen}
                </div>
            </div>

            <HistoryLog history_list=history_list />
        </div>
    }
}
