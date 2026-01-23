use crate::components::controls::Controls;
use crate::components::history::HistoryLog;
use bonsai_chess::prelude::BoardFrontend;
use leptos::prelude::*;

#[component]
pub fn Sidebar(
    game: ReadSignal<BoardFrontend>,
    history_list: ReadSignal<Vec<String>>,
    on_undo: Callback<()>, // Changed to Callback
) -> impl IntoView {
    view! {
        <div class="w-[80vmin] md:w-64 flex flex-col gap-4 h-64 md:h-[80vmin]">
            <Controls game=game on_undo=on_undo />
            <HistoryLog history_list=history_list />
        </div>
    }
}
