use crate::utils::provide_feedback;
use bonsai_chess::prelude::*;
use bonsai_engine::best_move;
use leptos::prelude::*;
use std::time::Duration;

pub fn use_engine(game: ReadSignal<BoardFrontend>, set_game: WriteSignal<BoardFrontend>) {
    Effect::new(move |_| {
        let current_game = game.get();
        let turn = current_game.turn();
        let outcome = current_game.outcome();

        if turn == Team::Black && outcome.is_none() {
            set_timeout(
                move || {
                    if let Some(engine_ply) = best_move(current_game.clone(), 4) {
                        set_game.update(|g| g.make_move(&engine_ply));
                        provide_feedback(&engine_ply);
                    }
                },
                Duration::from_millis(100),
            );
        }
    });
}
