use bonsai_chess::prelude::*;
use leptos::prelude::*;

#[component]
pub fn GameOverModal(outcome: Outcome, on_restart: Callback<()>) -> impl IntoView {
    // Determine the main title (e.g., "White Wins!" or "Draw")
    let title = match outcome {
        Outcome::Win { winner, .. } => format!("{winner:?} Wins!"),
        Outcome::Draw { .. } => "Game Drawn".to_string(),
    };

    // Determine the subtitle based on the specific reason from your enum
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
        <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/70 backdrop-blur-sm animate-in fade-in duration-200">
            <div class="bg-zinc-800 p-8 rounded-2xl border-2 border-zinc-600 shadow-2xl flex flex-col items-center gap-6 max-w-sm w-full text-center">
                <div class="space-y-2">
                    <h2 class="text-3xl font-bold text-white tracking-tight">{title}</h2>
                    <p class="text-zinc-400 text-lg font-medium">{description}</p>
                </div>

                <button
                    class="px-8 py-3 bg-emerald-600 hover:bg-emerald-500 text-white rounded-xl font-bold text-lg transition-all shadow-lg hover:shadow-emerald-900/30 active:scale-95"
                    on:click=move |_| on_restart.run(())
                >
                    "New Game"
                </button>
            </div>
        </div>
    }
}
