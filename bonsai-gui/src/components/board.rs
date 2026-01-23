use crate::components::square::Square;
use bonsai_chess::prelude::*;
use leptos::prelude::*;

#[component]
pub fn Board(
    game: ReadSignal<BoardFrontend>,
    selected_square: ReadSignal<Option<Coordinates>>,
    valid_targets: Memo<Vec<Coordinates>>,
    on_square_click: Callback<(usize, usize)>, // Changed to Callback
) -> impl IntoView {
    view! {
        <div class="select-none">
            <div class="grid grid-cols-8 grid-rows-8 border-4 border-zinc-700 bg-zinc-800 shadow-2xl aspect-square rounded-xl overflow-hidden w-[80vmin] h-[80vmin]">
                {
                    BOARD_ROWS_RANGE.map(|row_idx| {
                        view! {
                            {
                                BOARD_COLUMNS_RANGE.map(|col_idx| {
                                    // Callback is Copy, so we can pass it directly
                                    view! {
                                        <Square
                                            row=row_idx
                                            col=col_idx
                                            game=game
                                            selected_square=selected_square
                                            valid_targets=valid_targets
                                            on_click=on_square_click
                                        />
                                    }
                                }).collect_view()
                            }
                        }
                    }).collect_view()
                }
            </div>
        </div>
    }
}
