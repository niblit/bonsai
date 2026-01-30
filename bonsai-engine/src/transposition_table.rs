use bonsai_chess::prelude::{Ply, PositionSnapshot};
use std::collections::HashMap;

use crate::config::TRANSPOSITION_TABLE_INITIAL_SIZE;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeType {
    Exact, // The score is exact
    Upper, // The score is an upper bound (beta cutoff)
    Lower, // The score is a lower bound (alpha improvement)
}

#[derive(Clone, Copy, Debug)]
pub struct Entry {
    pub score: isize,
    pub depth: usize,
    pub node_type: NodeType,
    pub best_move: Option<Ply>,
}

pub struct TranspositionTable {
    table: HashMap<PositionSnapshot, Entry>,
}

impl TranspositionTable {
    pub fn new() -> Self {
        Self {
            table: HashMap::with_capacity(TRANSPOSITION_TABLE_INITIAL_SIZE),
        }
    }

    pub fn get(&self, snapshot: &PositionSnapshot) -> Option<&Entry> {
        self.table.get(snapshot)
    }

    pub fn insert(&mut self, snapshot: PositionSnapshot, entry: Entry) {
        // Simple replacement strategy: replace if the new search was deeper
        if let Some(existing) = self.table.get(&snapshot) {
            if entry.depth >= existing.depth {
                self.table.insert(snapshot, entry);
            }
        } else {
            self.table.insert(snapshot, entry);
        }
    }
}
