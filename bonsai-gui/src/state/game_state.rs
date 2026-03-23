use crate::utils::provide_feedback;
use bonsai_chess::prelude::*;
use leptos::prelude::*;

#[derive(Copy, Clone)]
pub struct GameState {
    pub game: ReadSignal<BoardFrontend>,
    pub set_game: WriteSignal<BoardFrontend>,

    pub selected_square: ReadSignal<Option<Coordinates>>,
    set_selected_square: WriteSignal<Option<Coordinates>>,

    pub pending_promotion: ReadSignal<Option<Ply>>,
    set_pending_promotion: WriteSignal<Option<Ply>>,

    // Derived State
    pub fen: Memo<String>,
    pub outcome: Memo<Option<Outcome>>,
    pub move_log: Memo<Vec<Ply>>,
    pub valid_targets: Memo<Vec<Coordinates>>,
}

impl GameState {
    pub fn new() -> Self {
        let (game, set_game) = signal(BoardFrontend::from_starting_position());
        let (selected_square, set_selected_square) = signal::<Option<Coordinates>>(None);
        let (pending_promotion, set_pending_promotion) = signal::<Option<Ply>>(None);

        let fen = Memo::new(move |_| game.get().to_fen());
        let outcome = Memo::new(move |_| game.get().outcome());
        let move_log = Memo::new(move |_| game.get().get_move_log());

        let valid_targets = Memo::new(move |_| {
            selected_square.get().map_or_else(Vec::new, |sel| {
                game.get()
                    .get_legal_moves()
                    .into_iter()
                    .filter(|m| m.starting_square() == sel)
                    .map(|m| m.ending_square())
                    .collect()
            })
        });

        Self {
            game,
            set_game,
            selected_square,
            set_selected_square,
            pending_promotion,
            set_pending_promotion,
            fen,
            outcome,
            move_log,
            valid_targets,
        }
    }

    pub fn handle_square_click(&self, row: usize, col: usize) {
        let last_click = Coordinates::new(row, col).unwrap();
        let mut current_game = self.game.get();
        let turn = current_game.turn();

        // Lock interaction if Engine is thinking
        if turn == Team::Black {
            return;
        }

        if let Some(selected) = self.selected_square.get() {
            if selected == last_click {
                self.set_selected_square.set(None);
                return;
            }

            let matching_move = current_game
                .get_legal_moves()
                .into_iter()
                .find(|m| m.starting_square() == selected && m.ending_square() == last_click);

            if let Some(ply) = matching_move {
                self.execute_move(ply);
            } else {
                self.select_own_piece(last_click, turn, &current_game);
            }
        } else {
            self.select_own_piece(last_click, turn, &current_game);
        }
    }

    fn execute_move(&self, ply: Ply) {
        if matches!(ply.special_move(), Some(SpecialMove::Promotion(_))) {
            self.set_pending_promotion.set(Some(ply));
            self.set_selected_square.set(None);
        } else {
            self.set_game.update(|g| g.make_move(&ply));
            self.set_selected_square.set(None);
            provide_feedback(&ply);
        }
    }

    fn select_own_piece(&self, coord: Coordinates, turn: Team, game: &BoardFrontend) {
        if game.backend().get(coord).is_some_and(|p| p.team() == turn) {
            self.set_selected_square.set(Some(coord));
        } else {
            self.set_selected_square.set(None);
        }
    }

    pub fn promote(&self, choice: ValidPromotions) {
        if let Some(base_ply) = self.pending_promotion.get() {
            let final_ply = self.game.get().get_legal_moves().into_iter().find(|m| {
                m.starting_square() == base_ply.starting_square()
                    && m.ending_square() == base_ply.ending_square()
                    && matches!(m.special_move(), Some(SpecialMove::Promotion(c)) if c == choice)
            });

            self.cancel_promotion(); // Clears selection & pending states

            if let Some(ply) = final_ply {
                self.set_game.update(|g| g.make_move(&ply));
                provide_feedback(&ply);
            }
        }
    }

    pub fn cancel_promotion(&self) {
        self.set_pending_promotion.set(None);
        self.set_selected_square.set(None);
    }

    pub fn undo(&self) {
        self.set_game.update(BoardFrontend::undo_last_move);
        self.set_game.update(BoardFrontend::undo_last_move);
        self.set_selected_square.set(None);
    }

    pub fn restart(&self) {
        self.set_game.set(BoardFrontend::from_starting_position());
        self.set_selected_square.set(None);
        self.set_pending_promotion.set(None);
    }
}
