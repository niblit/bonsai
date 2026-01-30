use crate::components::piece::PieceView;
use bonsai_chess::prelude::*;
use leptos::prelude::*;

#[component]
pub fn PromotionModal(
    team: Team,
    on_select: Callback<ValidPromotions>,
    on_cancel: Callback<()>,
) -> impl IntoView {
    let options = [
        ValidPromotions::Queen,
        ValidPromotions::Rook,
        ValidPromotions::Bishop,
        ValidPromotions::Knight,
    ];

    view! {
        <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm">
            <div class="bg-zinc-800 p-6 rounded-2xl border-2 border-zinc-600 shadow-2xl flex flex-col items-center gap-4">
                <h2 class="text-xl font-bold text-white">Choose Promotion</h2>
                <div class="flex gap-4">
                    {options
                        .into_iter()
                        .map(|promo| {
                            let piece = Piece::new(team, Kind::from_valid_promotions(promo));
                            view! {
                                <button
                                    class="w-20 h-20 bg-zinc-700 hover:bg-zinc-600 rounded-xl flex items-center justify-center transition-colors"
                                    on:click=move |_| on_select.run(promo)
                                >
                                    <PieceView piece=piece />
                                </button>
                            }
                        })
                        .collect_view()}
                </div>

                // Cancel Button
                <button
                    class="px-6 py-2 bg-red-900/40 hover:bg-red-900/60 text-red-200 rounded-lg text-sm font-medium transition-colors border border-red-500/30"
                    on:click=move |_| on_cancel.run(())
                >
                    "Cancel Move"
                </button>
            </div>
        </div>
    }
}
