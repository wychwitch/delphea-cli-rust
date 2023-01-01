pub mod models;
pub mod schema;
use self::models::*;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenvy::dotenv;
use rand::{seq::IteratorRandom, thread_rng};
use std::env;
use std::{io, vec};
use tui::{backend::CrosstermBackend, Terminal};

use crate::models::Entry;
use crate::models::Sheet;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn load_db() -> Database {
    use self::schema::entries::dsl::*;

    use self::schema::sheets::dsl::*;

    let connection = &mut establish_connection();

    let all_sheets = sheets
        .load::<Sheet>(connection)
        .expect("Error loading sheets");

    let all_entries = entries
        .load::<Entry>(connection)
        .expect("Error loading sheets");

    Database {
        entries: all_entries,
        sheets: all_sheets,
    }
}

struct Database {
    entries: Vec<Entry>,
    sheets: Vec<Sheet>,
}

impl Entry {
    pub fn new(entries: Vec<Entry>, sheet_id: i32, name: &str, color: &str, note: &str) -> Entry {
        Entry {
            id: entries.len() as i32,
            sheet_id,
            name: name.to_string(),
            color: color.to_uppercase(),
            note: note.to_string(),
            won_against: vec![],
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

    pub fn track_wins(&mut self, losers: &Vec<Entry>) {
        let mut picked: Vec<i32> = losers.into_iter().map(|loser| loser.id).collect();
        self.won_against.append(&mut picked);
    }

    pub fn clear_wins(&mut self) {
        self.won_against = vec![];
    }

    pub fn clear_deleted_loss(&mut self, rem_loser_id: &i32) {
        let mut save: Vec<i32> = self
            .won_against
            .clone()
            .into_iter()
            .filter(|loser_id| loser_id != rem_loser_id)
            .collect();
        self.won_against.append(&mut save);
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

    pub fn check_if_favorite(&mut self, all_sheet_entries: &Vec<Entry>) -> bool {
        let filtered_entries: Vec<&Entry> = all_sheet_entries
            .into_iter()
            .filter(|entry| !entry.favorited)
            .collect();
        if filtered_entries
            .into_iter()
            .all(|entry| self.won_against.contains(&entry.id))
        {
            true
        } else {
            false
        }
    }
}

impl Sheet {
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

    pub fn clear_all_favorites(&mut self, entries: Vec<Entry>) {
        let all_sheet_entries = self.get_entries(entries);
        for mut entry in all_sheet_entries {
            entry.clear_wins();
        }
    }

    pub fn handle_choices(winners: &mut Vec<Entry>, losers: &Vec<Entry>) {
        //have this actually update the choices later, rn it just sets the first element to be picked
        let loser_ids = Entry::entries_vec_to_id(losers);
        for entry in winners {
            entry.won_against.append(&mut loser_ids.clone())
        }
    }

    pub fn display_choices(
        &mut self,
        random_entries: &mut Vec<Entry>,
        all_sheet_entries: &Vec<Entry>,
    ) -> Vec<Entry> {
        //another fn that assigns won against
        let random_clone = random_entries.clone().to_owned();
        random_clone
            .into_iter()
            .map(|mut entry| {
                let new_entry = Entry {
                    favorited: entry.check_if_favorite(all_sheet_entries),
                    ..entry
                };
                new_entry
            })
            .collect()
    }
    pub fn picker(&mut self, entries: &Vec<Entry>) {
        let mut rng = thread_rng();
        let mut filtered_entries = self.get_entries(entries.clone());
        while filtered_entries.len() != 0 {
            let mut random_entries = filtered_entries.into_iter().choose_multiple(&mut rng, 20);

            let picked_entries = self.display_choices(&mut random_entries, entries);

            let cleaned = picked_entries
                .into_iter()
                .filter(|entry| !entry.favorited)
                .collect();

            filtered_entries = cleaned;
        }
    }
}

enum DbTypes {
    Sheet,
    Entry,
}

enum DbReturnTypes {
    Sheets(Vec<Sheet>),
    Entries(Vec<Entry>),
}

//fn save_db(all_entries: Vec<Entry>, all_sheets: Vec<Sheet>) {}

fn tui_testing() {}

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    Ok(())
}
