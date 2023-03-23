use crate::colors::AvailableColors;
use crate::entries::Entry;
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
    pub fn debug_add_entries(&mut self, entries: &mut Vec<Entry>) {
        for i in 1..=80 {
            let entry = Entry::new(
                entries,
                self.id,
                &format!("Entry {}", i),
                AvailableColors::Lavender as u8,
                "",
            );
            entries.push(entry);
        }
    }

    pub fn new(sheet_len: usize, name: &str, color: u8, note: &str) -> Sheet {
        Sheet {
            id: sheet_len + 1,
            name: name.into(),
            color,
            note: note.into(),
            entries: vec![],
        }
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
    pub fn get_entries(&mut self) -> Vec<Entry> {
        self.entries.to_owned()
    }

    pub fn get_sheet_by_id(sheets: Vec<Sheet>, sheet_id: usize) -> Sheet {
        sheets
            .clone()
            .into_iter()
            .find(|sheet| sheet.id == sheet_id)
            .unwrap()
    }
    pub fn clear_all_favorites(&mut self) {
        for i in 0..self.entries.len() {
            self.entries[i].clear_losses();
            self.entries[i].rank = 0;
        }
    }
    pub fn interactive_create_root(msg: &str) -> (String, u8, String) {
        let colors = all::<AvailableColors>().collect::<Vec<_>>();

        let name: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("Enter this {msg}'s name"))
            .interact()
            .unwrap();
        let color: usize = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Pick a color")
            .items(&colors)
            .interact()
            .unwrap();
        let note: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Any notes?")
            .interact()
            .unwrap();
        (name, color.try_into().unwrap(), note)
    }
    pub fn interactive_create(sheet_len: usize) -> Sheet {
        let (name, color, note) = Sheet::interactive_create_root("Sheet");
        Sheet::new(sheet_len, &name, color, &note)
    }
    pub fn interactive_create_entry(&mut self) {
        let (name, color, note) = Sheet::interactive_create_root("Entry");
        let entry = Entry::new(&self.get_entries(), self.id, &name, color, &note);
        self.entries.push(entry);
    }
}

impl Display for Sheet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
