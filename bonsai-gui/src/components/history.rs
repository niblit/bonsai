use bonsai_chess::prelude::Ply;
use leptos::html::Div;
use leptos::prelude::*;

#[component]
pub fn HistoryLog(history_list: Memo<Vec<Ply>>) -> impl IntoView {
    // Reference to an invisible element at the bottom of the list
    let end_of_list_ref = NodeRef::<Div>::new();

    Effect::new(move |_| {
        history_list.track();

        // request_animation_frame ensures the DOM has updated before we try to scroll
        request_animation_frame(move || {
            if let Some(el) = end_of_list_ref.get() {
                // This is more reliable than calculating scroll_height manually
                el.scroll_into_view();
            }
        });
    });

    view! {
        <div class="flex flex-col gap-1 h-full min-h-0">
            <label class="text-xs text-zinc-400 font-bold tracking-wider">"Move history"</label>
            <div class="flex-1 bg-zinc-800 p-4 rounded-lg shadow-lg border border-zinc-700 overflow-y-auto min-h-0">
                <div class="flex flex-col gap-1 font-mono text-sm">
                    {move || {
                        let history = history_list.get();
                        let total_moves = history.len();
                        history
                            .chunks(2)
                            .enumerate()
                            .map(|(chunk_idx, chunk)| {
                                let is_last_chunk = (chunk_idx + 1) * 2 >= total_moves
                                    && (chunk_idx + 1) * 2 - 1 <= total_moves + 1;
                                let white_ply = chunk.first().copied();
                                let black_ply = chunk.get(1).copied();
                                let white_move = white_ply.map_or(String::new(), |p| p.to_string());
                                let black_move = black_ply.map_or(String::new(), |p| p.to_string());
                                let highlight_class = "bg-green-400/90 text-black font-bold rounded px-2";
                                let default_class = "text-zinc-200 px-1";
                                let white_class = if is_last_chunk && black_ply.is_none() {
                                    highlight_class
                                } else {
                                    default_class
                                };
                                let black_class = if is_last_chunk && black_ply.is_some() {
                                    highlight_class
                                } else {
                                    default_class
                                };

                                // Determine if this is the last row being rendered

                                // Logic to extract moves and highlight the correct one

                                // Highlight logic:
                                // 1. White is highlighted if it's the last chunk AND there is no black move yet.
                                // 2. Black is highlighted if it's the last chunk AND the black move exists.

                                view! {
                                    <div class="grid grid-cols-[3rem_1fr_1fr] border-b border-zinc-700/50 py-1">
                                        <span class="text-zinc-500 select-none text-right pr-2">
                                            {format!("{}.-", chunk_idx + 1)}
                                        </span>
                                        // Container for White Move
                                        <div class="flex items-center">
                                            <span class=white_class>{white_move}</span>
                                        </div>
                                        // Container for Black Move
                                        <div class="flex items-center">
                                            <span class=black_class>{black_move}</span>
                                        </div>
                                    </div>
                                }
                            })
                            .collect_view()
                    }} // This invisible div is the "Anchor" we scroll to
                    <div node_ref=end_of_list_ref class="h-0 w-0"></div>
                </div>
            </div>
        </div>
    }
}
