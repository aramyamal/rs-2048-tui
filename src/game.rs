use crate::game_state::GameState;
use std::collections::VecDeque;

pub struct Game<const DIM: usize> {
    history: VecDeque<GameState<DIM>>,
    undos_left: usize,
}

impl<const DIM: usize> Game<DIM> {
    pub fn new(undos: usize) -> Self {
        let mut game = Self {
            history: VecDeque::with_capacity(undos + 1),
            undos_left: undos,
        };
        game.push_state(GameState::new());
        game
    }

    fn push_state(&mut self, gs: GameState<DIM>) {
        if self.history.len() > self.undos_left {
            self.history.pop_back();
        }
        self.history.push_front(gs);
    }

    pub fn undo(&mut self) {
        if self.undos_left > 0 {
            self.history.pop_front();
            self.undos_left -= 1;
        }
    }

    pub fn left(&mut self) {
        let current_gs = self
            .history
            .front()
            .expect("history always contains at least one value");

        let mut next_gs = current_gs.clone();
        next_gs.slide_and_merge_left();
        if next_gs == *current_gs {
            return;
        }

        self.push_state(next_gs);
    }

    pub fn down(&mut self) {
        let current_gs = self
            .history
            .front()
            .expect("history always contains at least one value");

        let mut next_gs = current_gs.clone();
        next_gs.rotate90();
        next_gs.slide_and_merge_left();
        next_gs.rotate270();
        if next_gs == *current_gs {
            return;
        }

        self.push_state(next_gs);
    }

    pub fn up(&mut self) {
        let current_gs = self
            .history
            .front()
            .expect("history always contains at least one value");

        let mut next_gs = current_gs.clone();
        next_gs.rotate270();
        next_gs.slide_and_merge_left();
        next_gs.rotate90();
        if next_gs == *current_gs {
            return;
        }

        self.push_state(next_gs);
    }

    pub fn right(&mut self) {
        let current_gs = self
            .history
            .front()
            .expect("history always contains at least one value");

        let mut next_gs = current_gs.clone();
        next_gs.rotate180();
        next_gs.slide_and_merge_left();
        next_gs.rotate180();
        if next_gs == *current_gs {
            return;
        }

        self.push_state(next_gs);
    }
}
