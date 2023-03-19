use console::Style;
use dialoguer::{theme::ColorfulTheme, theme::SimpleTheme, Input, MultiSelect, Select, Validator};
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    pub all_entries: Vec<Entry>,
    pub all_sheets: Vec<Sheet>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
pub struct Entry {
    pub id: usize,
    pub name: String,
    pub color: u8,
    pub note: String,
    pub rank: usize,
    pub lost_against: Vec<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sheet {
    pub id: usize,
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
        sheet_id: usize,
        name: &str,
        color: u8,
        note: &str,
    ) -> Entry {
        Entry {
            id: entries.len() as usize,
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

pub fn validate_selection(
    selection: Option<Vec<usize>>,
    choices_len: usize,
) -> Result<Vec<usize>, String> {
    match selection {
        Some(selec) => match selec.len() != choices_len && selec.len() != 0 {
            true => Ok(selec),
            false => {
                if selec.len() == 0 {
                    return Err("You must select something".to_owned());
                } else {
                    return Err("You need to leave one option unselected!".to_owned());
                }
            }
        },
        None => Err("Canceled".to_owned()),
    }
}

pub fn mult_menu_creation<T: std::fmt::Display + std::fmt::Debug>(
    choices: &[T],
    msg: &str,
) -> Result<Vec<usize>, String> {
    let selection_i = MultiSelect::with_theme(&SimpleTheme)
        .with_prompt(format!("Pick your {msg} (use space)"))
        .items(&choices)
        .interact_opt()
        .unwrap();
    validate_selection(selection_i, choices.len())
}
