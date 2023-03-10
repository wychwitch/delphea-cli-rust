use console::Style;
use dialoguer::{theme::ColorfulTheme, Input, MultiSelect, Select};
use enum_iterator::{all, Sequence};
use rand::{seq::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};
use std::{
    borrow::BorrowMut,
    fmt::{self, Display},
};

#[derive(Debug, PartialEq, Sequence, Clone)]
pub enum AvailableColors {
    Pink = 224,
    Yellow = 222,
    Lavender = 182,
    Orange = 173,
    Ruddy = 167,
    Bluish = 146,
    Brown = 138,
    Magenta = 132,
    Green = 108,
    Sky = 105,
    Storm = 103,
    Purple = 97,
    Plum = 96,
    NeonViolet = 91,
    Ruby = 89,
    Red = 1,
}

#[derive(Serialize, Deserialize)]
pub struct Database {
    pub all_entries: Vec<Entry>,
    pub all_sheets: Vec<Sheet>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
pub struct Entry {
    pub id: i32,
    pub name: String,
    pub color: u8,
    pub note: String,
    pub rank: usize,
    pub lost_against: Vec<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sheet {
    pub id: i32,
    pub name: String,
    pub color: u8,
    pub note: String,
    pub entries: Vec<Entry>,
}
#[derive(Debug, Clone)]
pub struct EntryBag {
    pub len: usize,
    pub loss_len: usize,
    pub entries: Vec<Entry>,
}

//
//Implementations
//

impl Entry {
    pub fn get_lost_len(&self) -> usize {
        self.lost_against.len()
    }
    pub fn new(
        entries: &mut Vec<Entry>,
        sheet_id: i32,
        name: &str,
        color: u8,
        note: &str,
    ) -> Entry {
        Entry {
            id: entries.len() as i32,
            name: name.to_string(),
            color,
            note: note.to_string(),
            rank: 0,
            lost_against: vec![],
        }
    }
    pub fn interactive_create(&self, sheets: &Vec<Sheet>) -> Entry {
        let (name, id, color, note) = self.interactive_create_root();

        let sheet_i = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Pick your sheet (use space)")
            .items(&sheets)
            .interact()
            .unwrap();

        let sheet_id = sheets[sheet_i].id;
        Entry {
            id,
            name,
            color,
            note,
            rank: 0,
            lost_against: vec![],
        }
    }

    pub fn track_losses(&mut self, mut winner_ids: Vec<i32>) {
        self.lost_against.append(&mut winner_ids);
    }

    pub fn clear_losses(&mut self) {
        self.lost_against = vec![];
    }

    pub fn lost_against(&self) -> &Vec<i32> {
        &self.lost_against
    }

    pub fn clear_removed_ids(&mut self, rem_loser_ids: Vec<i32>) {
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

    pub fn id_to_entry(entries: Vec<Entry>, entry_id: i32) -> Entry {
        entries
            .clone()
            .into_iter()
            .find(|entry| entry.id == entry_id)
            .unwrap()
    }

    pub fn get_entries_as_ids(entries: &Vec<Entry>) -> Vec<i32> {
        entries
            .into_iter()
            .clone()
            .map(|entry| entry.id)
            .collect::<Vec<i32>>()
    }

    pub fn check_if_favorite(&mut self, all_sheet_entries: &Vec<Entry>) -> bool {
        let full_entries: Vec<&Entry> = all_sheet_entries
            .iter()
            .filter(|e| self.lost_against.contains(&e.id))
            .collect();

        if full_entries.iter().all(|e| e.rank != 0) {
            let ranks = full_entries.iter().map(|e| e.rank);
            self.rank = ranks.max().unwrap();
            true
        } else {
            false
        }
    }
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

    pub fn new(all_sheets: &Vec<Sheet>, name: &str, color: u8, note: &str) -> Sheet {
        let next_index = all_sheets.len() as i32 + 1;
        Sheet {
            id: next_index,
            name: name.into(),
            color,
            note: note.into(),
            entries: vec![],
        }
    }

    pub fn new_debug(
        id: i32,
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

    pub fn interactive_create(&self, all_sheets: &Vec<Sheet>) -> Sheet {
        let (name, _, color, note) = self.interactive_create_root();
        Sheet::new(all_sheets, &name, color, &note)
    }

    pub fn get_sheet_by_id(sheets: Vec<Sheet>, sheet_id: i32) -> Sheet {
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
}

impl Display for Sheet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let style = Style::new().color256(self.color);
        write!(f, "{}", style.apply_to(&self.name))
    }
}

impl EntryBag {
    pub fn is_all_ranked(&self) -> bool {
        self.entries.iter().all(|e| e.rank > 0)
    }
}

impl Database {
    pub fn save(&self) {}

    pub fn pick_sheet_idx(&self) -> usize {
        let sheet_id = menu_creation(&self.all_sheets, "sheet");
        sheet_id
    }
}

impl Display for AvailableColors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AvailableColors::Pink => write!(f, "Pink"),
            AvailableColors::Yellow => write!(f, "Yellow"),
            AvailableColors::Lavender => write!(f, "Lavender"),
            AvailableColors::Orange => write!(f, "Orange"),
            AvailableColors::Ruddy => write!(f, "Ruddy"),
            AvailableColors::Bluish => write!(f, "Bluish"),
            AvailableColors::Brown => write!(f, "Brown"),
            AvailableColors::Magenta => write!(f, "Magenta"),
            AvailableColors::Green => write!(f, "Green"),
            AvailableColors::Sky => write!(f, "Sky"),
            AvailableColors::Storm => write!(f, "Storm"),
            AvailableColors::Purple => write!(f, "Purple"),
            AvailableColors::Plum => write!(f, "Plum"),
            AvailableColors::NeonViolet => write!(f, "Neon Violet"),
            AvailableColors::Ruby => write!(f, "Ruby"),
            AvailableColors::Red => write!(f, "Red"),
        }
    }
}

//
// Traits
//

impl InteractiveCreate for Entry {}
impl InteractiveCreate for Sheet {}
trait InteractiveCreate {
    fn interactive_create_root(&self) -> (String, i32, u8, String) {
        let colors = all::<AvailableColors>().collect::<Vec<_>>();

        let name: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Pick your sheet (use space)")
            .interact()
            .unwrap();
        let id: i32 = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Pick your sheet (use space)")
            .interact()
            .unwrap();
        let color: usize = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Pick your sheet (use space)")
            .items(&colors)
            .interact()
            .unwrap();
        let note: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Pick your sheet (use space)")
            .interact()
            .unwrap();
        (name, id, color.try_into().unwrap(), note)
    }
}

trait InteractiveEdit {
    //for sheet and entry
    fn interactive_edit_root(&self) {
        //todo finish thisd
    }
}

trait InteractiveDelete {
    //for db and sheet
    fn interactive_delete_root(&self) {
        //todo finish this
    }
}

///
/// helper
///

pub fn menu_creation<T: std::fmt::Display>(choices: &Vec<T>, msg: &str) -> usize {
    let selection_i: usize = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Pick your {msg} (use space)"))
        .items(&choices)
        .interact()
        .unwrap();

    selection_i
}
