use bonsai_chess::prelude::{DrawReason, Outcome, WinReason};
use leptos::prelude::*;

#[component]
pub fn GameOver(
    outcome: Outcome,
    on_restart: Callback<()>,
    on_undo: Callback<()>,
) -> impl IntoView {
    // Determine the main title (e.g., "White Wins!" or "Game Drawn")
    let title = match outcome {
        Outcome::Win { winner, .. } => format!("{winner:?} Wins!"),
        Outcome::Draw { .. } => "Game Drawn".to_string(),
    };

    // Determine the subtitle based on the specific reason
    let description = match outcome {
        Outcome::Win { reason, .. } => match reason {
            WinReason::Checkmate => "by Checkmate",
            WinReason::Resign => "Opponent Resigned",
            WinReason::WinOnTime => "on Time",
            WinReason::Forfeit => "by Forfeit",
        },
        Outcome::Draw { reason } => match reason {
            DrawReason::Stalemate => "Stalemate",
            DrawReason::ThreefoldRepetition => "Threefold Repetition",
            DrawReason::FiftyMoveRule => "50-Move Rule",
            DrawReason::DeadPosition => "Insufficient Material",
            DrawReason::DrawByAgreement => "Agreed Draw",
            DrawReason::DrawOnTime => "Time vs Insufficient Material",
            DrawReason::Forfeit => "Forfeit",
        },
    };

    view! {
        <div class="bg-zinc-800/90 p-5 rounded-2xl shadow-xl border border-zinc-700/50 flex flex-col gap-5 backdrop-blur-sm">
            
            // Text Header Area
            <div class="flex flex-col items-center justify-center text-center mt-2">
                <h2 class="text-2xl font-black text-zinc-100 tracking-tight drop-shadow-sm">
                    {title}
                </h2>
                <span class="text-xs font-bold text-zinc-400 uppercase tracking-widest mt-1">
                    {description}
                </span>
            </div>

            // Buttons Area
            <div class="flex flex-col gap-3 w-full">
                // Primary Action: New Game
                <button
                    class="w-full px-6 py-3 bg-emerald-600 hover:bg-emerald-500 text-white rounded-xl font-bold text-base transition-all shadow-lg shadow-emerald-900/20 active:scale-[0.98]"
                    on:click=move |_| on_restart.run(())
                >
                    "New Game"
                </button>

                // Secondary Action: Undo Move
                <button
                    class="w-full px-6 py-3 bg-rose-400/40 hover:bg-rose-600 text-rose-200 hover:text-white border border-rose-900/50 hover:border-rose-500 rounded-xl font-bold text-base transition-all shadow-lg active:scale-[0.98]"
                    on:click=move |_| on_undo.run(())
                >
                    "Undo Move"
                </button>
            </div>
        </div>
    }
}