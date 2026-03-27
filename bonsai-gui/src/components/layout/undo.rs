use leptos::prelude::*;

#[component]
pub fn Undo(on_undo: Callback<()>) -> impl IntoView {
    view! {
        <button
            class="w-full px-6 py-3 hover:bg-rose-500/80 bg-rose-600 hover:text-rose-200 text-white border hover:border-rose-900/50 border-rose-500 rounded-xl font-bold text-base transition-all shadow-lg active:scale-[0.98]"
            on:click=move |_| on_undo.run(())
        >
            "Undo Move"
        </button>
    }
}
