use console::Style;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
pub struct Entry {
    pub id: usize,
    pub name: String,
    pub color: u8,
    pub note: String,
    pub rank: usize,
    pub lost_against: Vec<usize>,
}

impl Entry {
    pub fn get_lost_len(&self) -> usize {
        self.lost_against.len()
    }
    pub fn new(entries_len: usize, name: &str, color: u8, note: &str) -> Entry {
        let id = entries_len + 1;
        Entry {
            id,
            name: name.to_string(),
            color,
            note: note.to_string(),
            rank: 0,
            lost_against: vec![],
        }
    }

    pub fn track_losses(&mut self, mut winner_ids: Vec<usize>) {
        self.lost_against.append(&mut winner_ids);
    }

    pub fn clear_winner(&mut self, winner_id: usize) {
        let i = self.lost_against.iter().position(|id| id == &winner_id);
        match i {
            Some(i) => {
                self.lost_against.swap_remove(i);
            }
            None => (),
        }
    }
    pub fn clear_losses(&mut self) {
        self.lost_against = vec![];
    }

    pub fn lost_against(&self) -> &Vec<usize> {
        &self.lost_against
    }

    pub fn clear_removed_ids(&mut self, rem_loser_ids: Vec<usize>) {
        let mut affected_indexes: Vec<usize> = vec![];

        for (i, entry_id) in self.lost_against.iter().enumerate() {
            if rem_loser_ids.contains(entry_id) {
                affected_indexes.push(i);
            }
        }
        for i in affected_indexes.iter().rev() {
            self.lost_against.remove(*i);
        }
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let style = Style::new().color256(self.color);
        write!(f, "{}", style.apply_to(&self.name))
    }
}
