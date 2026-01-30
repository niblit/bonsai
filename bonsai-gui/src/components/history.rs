use bonsai_chess::prelude::Ply;
use leptos::prelude::*;

#[component]
pub fn HistoryLog(history_list: Memo<Vec<Ply>>) -> impl IntoView {
    view! {
        <div class="flex-1 bg-zinc-800 p-4 rounded-lg shadow-lg border border-zinc-700 overflow-y-auto">
            <h3 class="font-bold border-b border-zinc-600 pb-2 mb-2 sticky top-0 bg-zinc-800">
                History
            </h3>
            <div class="flex flex-col gap-1 font-mono text-sm">
                {move || {
                    let history = history_list.get();
                    history
                        .chunks(2)
                        .enumerate()
                        .map(|(i, chunk)| {
                            let white_move = match chunk.first().cloned() {
                                Some(ply) => ply.to_string(),
                                None => String::new(),
                            };
                            let black_move = match chunk.get(1).cloned() {
                                Some(ply) => ply.to_string(),
                                None => String::new(),
                            };

                            view! {
                                <div class="grid grid-cols-[3rem_1fr_1fr] border-b border-zinc-700/50 py-1">
                                    <span class="text-zinc-500 select-none text-right pr-2">
                                        {format!("{}.-", i + 1)}
                                    </span>
                                    <span class="text-zinc-200">{white_move}</span>
                                    <span class="text-zinc-200">{black_move}</span>
                                </div>
                            }
                        })
                        .collect_view()
                }}
            </div>
        </div>
    }
}
