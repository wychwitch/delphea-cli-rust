use crate::colors::AvailableColors;
use crate::entries::Entry;
use crate::menus::{confirm, create_select};
use console::Style;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use enum_iterator::all;
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sheet {
    pub id: usize,
    pub name: String,
    pub color: u8,
    pub note: String,
    pub entries: Vec<Entry>,
}

impl Sheet {
    pub fn new(sheet_len: usize, name: &str, color: u8, note: &str) -> Sheet {
        Sheet {
            id: sheet_len + 1,
            name: name.into(),
            color,
            note: note.into(),
            entries: vec![],
        }
    }
    pub fn check_if_all_unranked(&self) -> bool {
        return self.entries.iter().all(|e| e.rank == 0);
    }

    pub fn check_if_all_ranked(&self) -> bool {
        return self.entries.iter().all(|e| e.rank > 0);
    }

    pub fn new_debug(
        id: usize,
        name: &str,
        color: u8,
        note: &str,
        entries: &mut Vec<Entry>,
    ) -> Sheet {
        Sheet {
            id,
            name: name.into(),
            color,
            note: note.into(),
            entries: entries.to_owned(),
        }
    }

    pub fn clear_all_ranked(&mut self) {
        for i in 0..self.entries.len() {
            self.entries[i].clear_losses();
            self.entries[i].rank = 0;
        }
    }
    pub fn select_from_all_entries(&self, msg: &str) -> usize {
        let entries = &self.entries;
        create_select(entries, msg)
    }
    pub fn interactive_create_root(msg: &str) -> (String, u8, String) {
        let colors = all::<AvailableColors>().collect::<Vec<_>>();

        let name: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("Enter this {msg}'s name"))
            .interact()
            .unwrap();
        let color_i: usize = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Pick a color")
            .items(&colors)
            .interact()
            .unwrap();
        let note: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Any notes?")
            .allow_empty(true)
            .interact()
            .unwrap();
        (name, colors[color_i].clone() as u8, note)
    }
    pub fn interactive_edit_entry(&mut self, msg: &str, original_entry_i: usize) {
        let original_entry = &self.entries[original_entry_i];
        let colors = all::<AvailableColors>().collect::<Vec<_>>();
        let color_i = if let Some(color_i) = colors.iter().position(|c| {
            let color = c.to_owned();
            color as u8 == original_entry.color
        }) {
            color_i
        } else {
            0 as usize
        };
        let name: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("Enter this {msg}'s name"))
            .with_initial_text(original_entry.name.to_owned())
            .interact()
            .unwrap();
        let color_idx: usize = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Pick a color")
            .items(&colors)
            .default(color_i)
            .interact()
            .unwrap();
        let color = colors[color_idx].clone();
        let note: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Any notes?")
            .allow_empty(true)
            .with_initial_text(original_entry.note.to_owned())
            .interact()
            .unwrap();
        self.entries[original_entry_i] = Entry {
            name,
            lost_against: original_entry.lost_against.to_owned(),
            color: color as u8,
            id: original_entry.id,
            note,
            rank: original_entry.rank,
        }
    }

    pub fn interactive_create_entry(&mut self, entry_len: usize) {
        let (name, color, note) = Sheet::interactive_create_root("Entries");
        let entry = Entry::new(entry_len, &name, color, &note);
        self.entries.push(entry);
    }

    pub fn delete_entry(&mut self) {
        let entry_idx = self.select_from_all_entries("Select a sheet to delete");
        let entry_name = &self.entries[entry_idx].name;
        match confirm(&format!("Are you sure you want to delete {}", entry_name)) {
            Ok(choice) => match choice {
                true => {
                    self.entries.swap_remove(entry_idx);
                    println!("Sheet deleted!");
                }
                false => println!("Delete Aborted!"),
            },
            Err(_) => println!("Delete Aborted!"),
        }
    }

    pub fn view_entries(&mut self) {
        self.entries.sort_by(|a, b| a.rank.cmp(&b.rank));
        let unranked_entries_count = self.entries.iter().filter(|&e| e.rank == 0).count();
        self.entries.rotate_left(unranked_entries_count);
        for entry in &self.entries {
            let rank = match entry.rank {
                0 => "unranked".to_string(),
                _ => entry.rank.to_string(),
            };
            println!("{rank}: {entry}");
        }
        println!();
    }
}

impl Display for Sheet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let style = Style::new().color256(self.color);
        write!(f, "{}", style.apply_to(&self.name))
    }
}
