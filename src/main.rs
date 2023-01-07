pub mod models;
pub mod schema;
use console::Term;
use console::{Emoji, Style};
use dialoguer::{theme::ColorfulTheme, Input, MultiSelect, Select};
use enum_iterator::{all, Sequence};
use indicatif::ProgressBar;
use rand::{seq::IteratorRandom, thread_rng};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fmt::Display;
use std::thread;
use std::time::Duration;
use std::{env, fmt};
use std::{io, vec};

use crate::models::Entry;
use crate::models::Sheet;
use crate::models::Win;

#[derive(Debug, PartialEq, Sequence, Clone)]
enum AvailableColors {
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

fn load_db(connection: &mut SqliteConnection) -> Database {
    use self::schema::entries::dsl::*;
    use self::schema::sheets::dsl::*;
    use self::schema::wins::dsl::*;

    let all_sheets = sheets
        .load::<Sheet>(connection)
        .expect("Error loading sheets");

    let all_entries = entries
        .load::<Entry>(connection)
        .expect("Error loading sheets");

    let all_wins = wins.load::<Win>(connection).expect("Error loading sheets");

    Database {
        all_entries,
        all_sheets,
        all_wins,
    }
}

impl Database {
    pub fn save_db(&self) {}
}

impl Display for Sheet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

struct Database {
    all_entries: Vec<Entry>,
    all_sheets: Vec<Sheet>,
    all_wins: Vec<Win>,
}

impl Win {
    pub fn new(winner_id: i32, loser_id: i32, all_wins_length: i32) -> Win {
        Win {
            id: all_wins_length,
            winner_id,
            loser_id,
        }
    }
}

impl Entry {
    pub fn new(entries: Vec<Entry>, sheet_id: i32, name: &str, color: &str, note: &str) -> Entry {
        Entry {
            id: entries.len() as i32,
            sheet_id,
            name: name.to_string(),
            color: color.to_uppercase(),
            note: note.to_string(),
            favorited: false,
        }
    }

    pub fn get_sheet(&self, sheets: Vec<Sheet>) -> Sheet {
        sheets
            .into_iter()
            .find(|sheet| sheet.id == self.sheet_id)
            .expect("valid sheet id")
            .to_owned()
    }

    pub fn track_wins(&self, losers: &Vec<Entry>, all_wins: &mut Vec<Win>) {
        let mut picked: Vec<Win> = losers
            .into_iter()
            .map(|loser| Win::new(self.id, loser.id, all_wins.len() as i32))
            .collect();
        all_wins.append(&mut picked);
    }

    pub fn clear_wins(&self, all_wins: &mut Vec<Win>) {
        let mut i: usize = 0;

        let mut affected_indexes: Vec<usize> = Vec::new();
        for win in &mut *all_wins {
            if win.winner_id == self.id {
                affected_indexes.push(i);
            } else {
                i += 1;
            }
        }

        for i in affected_indexes {
            all_wins.remove(i);
        }

        let mut i = 1;
        for win in all_wins {
            win.id = i;
            i += 1;
        }
    }

    pub fn get_wins(&self, all_wins: &Vec<Win>) -> Vec<Win> {
        let wins: Vec<&Win> = all_wins.iter().filter(|win| win.id == self.id).collect();
        wins.iter()
            .map(|win| {
                Win::new(
                    win.winner_id,
                    win.loser_id,
                    all_wins.len().try_into().unwrap(),
                )
            })
            .collect()
    }
    pub fn get_wins_ids(&self, all_wins: &Vec<Win>) -> Vec<i32> {
        let won_against: Vec<i32> = self
            .get_wins(all_wins)
            .into_iter()
            .map(|win| win.loser_id)
            .collect();
        won_against
    }

    pub fn clear_deleted_loss(&mut self, rem_loser_id: &i32, all_wins: &mut Vec<Win>) {
        let won_against = self.get_wins_ids(all_wins);

        let mut affected_indexes: Vec<i32> = vec![];
        for i in 0..all_wins.len() {
            if all_wins[i].loser_id == *rem_loser_id {
                affected_indexes.push(i.try_into().unwrap());
            }
        }
        let mut y = 0;
        for i in affected_indexes {
            all_wins.remove((i - y).try_into().unwrap());
            y += 1;
        }
    }

    pub fn id_to_entry(entries: Vec<Entry>, entry_id: i32) -> Entry {
        entries
            .clone()
            .into_iter()
            .find(|entry| entry.id == entry_id)
            .unwrap()
    }
    pub fn entries_vec_to_id(entries: &Vec<Entry>) -> Vec<i32> {
        entries
            .into_iter()
            .clone()
            .map(|entry| entry.id)
            .collect::<Vec<i32>>()
    }

    pub fn check_if_favorite(
        &mut self,
        all_sheet_entries: &Vec<Entry>,
        all_wins: &Vec<Win>,
    ) -> bool {
        let won_against: Vec<i32> = self
            .get_wins(all_wins)
            .into_iter()
            .map(|win| win.loser_id)
            .collect();

        let filtered_entries: Vec<&Entry> = all_sheet_entries
            .into_iter()
            .filter(|entry| !entry.favorited)
            .collect();

        if filtered_entries
            .into_iter()
            .all(|entry| won_against.contains(&entry.id))
        {
            true
        } else {
            false
        }
    }
}

impl Sheet {
    pub fn new(all_sheets: &Vec<Sheet>, name: &str, color: &str, note: &str) -> Sheet {
        let next_index = all_sheets.len() as i32 + 1;
        Sheet {
            id: next_index,
            name: name.into(),
            color: color.into(),
            note: note.into(),
        }
    }

    pub fn get_entries(&self, entries: Vec<Entry>) -> Vec<Entry> {
        let filtered = entries
            .clone()
            .into_iter()
            .filter(|entry| entry.sheet_id == self.id)
            .collect::<Vec<Entry>>()
            .to_vec();
        filtered
    }

    pub fn get_sheet_by_id(sheets: Vec<Sheet>, sheet_id: i32) -> Sheet {
        sheets
            .clone()
            .into_iter()
            .find(|sheet| sheet.id == sheet_id)
            .unwrap()
    }

    pub fn get_entries_by_sheet_id(entries: Vec<Entry>, sheet_id: i32) -> Vec<Entry> {
        let filtered = entries
            .clone()
            .into_iter()
            .filter(|entry| entry.sheet_id == sheet_id)
            .collect::<Vec<Entry>>()
            .to_vec();
        filtered
    }

    pub fn clear_all_favorites(&mut self, entries: Vec<Entry>, all_winners: &mut Vec<Win>) {
        let all_sheet_entries = self.get_entries(entries);
        for mut entry in all_sheet_entries {
            entry.clear_wins(all_winners);
        }
    }

    pub fn handle_choices(
        winners: &mut Vec<Entry>,
        losers: &Vec<Entry>,
        all_winners: &mut Vec<Win>,
    ) {
        //have this actually update the choices later, rn it just sets the first element to be picked
        let loser_ids = Entry::entries_vec_to_id(losers);
        let mut i = 0;
        for winner in winners {
            let win_len: i32 = all_winners.len().try_into().unwrap();

            let mut winmap = loser_ids
                .iter()
                .map(|loser_id| {
                    let win = Win::new(winner.id, *loser_id, win_len + i);
                    i += 1;
                    win
                })
                .collect();
            all_winners.append(&mut winmap);
        }
    }

    pub fn display_choices(
        &mut self,
        random_entries: &mut Vec<Entry>,
        all_sheet_entries: &Vec<Entry>,
        all_winners: &mut Vec<Win>,
    ) -> Vec<Entry> {
        //another fn that assigns won against
        let random_clone = random_entries.clone().to_owned();
        random_clone
            .into_iter()
            .map(|mut entry| {
                let new_entry = Entry {
                    favorited: entry.check_if_favorite(all_sheet_entries, all_winners),
                    ..entry
                };
                new_entry
            })
            .collect()
    }
    pub fn get_remaining_ids(&self, all_entries: Vec<Entry>) -> Vec<i32> {
        let sheet_entries = self.get_entries(all_entries);
        let unfaved: Vec<Entry> = sheet_entries.into_iter().filter(|e| !e.favorited).collect();
        unfaved.iter().map(|e| e.id).collect()
    }

    pub fn picker(&mut self, entries: &Vec<Entry>, sheets: Vec<Sheet>, all_winners: &mut Vec<Win>) {
        let mut rng = thread_rng();
        let mut filtered_entries = self.get_entries(entries.clone());
        while filtered_entries.len() != 0 {
            let mut random_entries = filtered_entries.into_iter().choose_multiple(&mut rng, 20);

            let picked_entries = self.display_choices(&mut random_entries, entries, all_winners);

            let cleaned = picked_entries
                .into_iter()
                .filter(|entry| !entry.favorited)
                .collect();

            filtered_entries = cleaned;
        }
    }
}

//fn save_db(all_entries: Vec<Entry>, all_sheets: Vec<Sheet>) {}

fn handleround(db: &mut Database) {
    let sheets_iter = db.all_sheets.chunks(3);
    let bar = ProgressBar::new(10);
    let blue = Style::new().blue();
    let red = Style::new().red();
    let green = Style::new().green();
    let mut picked_sheets: Vec<&Sheet> = vec![];
    let mut picked_colors: Vec<usize> = vec![];
    let mut colors = all::<AvailableColors>().collect::<Vec<_>>();

    for sheet_vec in sheets_iter {
        let selection = MultiSelect::with_theme(&ColorfulTheme::default())
            .with_prompt("Pick your sheet (use space)")
            .max_length(3)
            .items(sheet_vec)
            .interact()
            .unwrap();
        for i in selection {
            picked_sheets.push(&db.all_sheets[i])
        }
    }

    let selection = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick your sheet (use space)")
        .max_length(3)
        .items(&colors)
        .interact()
        .unwrap();

    for sheet in picked_sheets {
        let color = match &sheet.color as &str {
            "green" => &green,
            "red" => &red,
            "blue" => &blue,
            _ => &blue,
        };
        println!("{} {}", color.apply_to(sheet), Emoji("⭐", "*"));
    }
    for color in colors {
        let fmt_color = Style::new().color256(color.clone() as u8);

        println!("{}", fmt_color.apply_to(color.clone()));
    }
}

trait InteractiveCreate {
    fn interactive_create_root(&self) {
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
    }
}

trait InteractiveEdit {
    fn interactive_edit(&self) {
        //todo
    }
}

trait InteractiveDelete {
    fn interactive_delete(&self) {
        //todo
    }
}

impl InteractiveCreate for Entry {}

fn main_menu(db: &mut Database) {}
fn handle_create(db: &mut Database) {}

fn main() {
    let connection = &mut establish_connection();
    let mut db = load_db(connection);

    db.all_sheets
        .push(Sheet::new(&db.all_sheets, "Games", "red", ""));
    db.all_sheets
        .push(Sheet::new(&db.all_sheets, "Books", "blue", ""));
    db.all_sheets
        .push(Sheet::new(&db.all_sheets, "Projects", "green", ""));
    db.all_sheets
        .push(Sheet::new(&db.all_sheets, "Study", "red", ""));

    db.all_sheets
        .push(Sheet::new(&db.all_sheets, "Games", "red", ""));
    db.all_sheets
        .push(Sheet::new(&db.all_sheets, "Books", "blue", ""));
    db.all_sheets
        .push(Sheet::new(&db.all_sheets, "Projects", "green", ""));
    db.all_sheets
        .push(Sheet::new(&db.all_sheets, "Study", "red", ""));

    handleround(&mut db)
    //handle_create(&mut db);

    // `choice` is a Vec<usize> containing the chosen indices

    //
}
