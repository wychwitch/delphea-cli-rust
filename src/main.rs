pub mod mods;

use dialoguer::{theme::ColorfulTheme, Input, MultiSelect, Select};
use mods::{AvailableColors, Database, Entry, Sheet};
use rand::thread_rng;
use std::cmp::Ordering;
use std::fs::File;

use std::vec;

fn handle_round(mut db: Database) {
    let sheet_idx = db.pick_sheet_idx();
    let mut sheet = &mut db.all_sheets[sheet_idx];
    sheet.debug_add_entries(&mut db.all_entries);
    let indices: Vec<usize> = sheet.get_entry_indices(&db.all_entries);
    db.all_entries = picker_setup(db.all_entries, indices);
}

fn main_menu(db: &mut Database) {}
fn handle_create(db: Database) {}

fn load_db() -> Database {
    let db = match File::open("~/.delphea.json") {
        Ok(file) => {
            let db: Database =
                serde_json::from_reader(file).expect("error while reading or parsing");
            db
        }
        Err(_) => Database {
            all_entries: vec![],
            all_sheets: vec![],
        },
    };
    db
}

fn picker_setup(sheet_entries: Vec<Entry>) -> Vec<Entry> {
    //fix this so it returns ALL entries or at least overwrites the sheet entries- actually yes do that
    let mut sheet_slices = sheet_entries.as_mut_slice();
    picker(sheet_slices)
}

fn picker(entries: Vec<Entry>) -> Vec<Entry> {
    //if the entries are too long, chunk it and recurse
    if entries.len() > 10 {
        let mut processed_entries: Vec<Entry>;
        let mut v_chunked: Vec<Vec<Entry>> = v.chunks(10).map(|x| x.to_vec()).collect();
        for chunk in &mut v_chunked {
            processed_entries.append(&mut picker(chunk.to_owned()));
        }
        processed_entries
    } else {
        let selection: Vec<usize> = mult_menu_creation(entries, "entries");
        let winner_ids: Vec<i32> = selection.into_iter().map(|s| entries[s].id).collect();
        let mut losers: Vec<&mut &mut Entry> = entries
            .iter_mut()
            .filter(|e| !winner_ids.contains(&e.id))
            .collect();
        for i in 0..losers.len() {
            //todo fix this so it builds the right kind of vec
            let mut winner_vec = winner_ids.clone();
            let mut loser = &mut losers[i];
            loser.lost_against.append(&mut winner_vec);
        }
    }
}

fn mult_menu_creation<T: std::fmt::Display>(choices: &[T], msg: &str) -> Vec<usize> {
    let selection_i = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Pick your {msg} (use space)"))
        .items(&choices)
        .interact()
        .unwrap();

    selection_i
}

fn main() {
    let mut db: Database = load_db();

    db.all_sheets.push(Sheet::new(
        &db.all_sheets,
        "Games",
        AvailableColors::Green as u8,
        "note!",
    ));
    db.all_sheets.push(Sheet::new(
        &db.all_sheets,
        "Books",
        AvailableColors::Green as u8,
        "",
    ));
    db.all_sheets.push(Sheet::new(
        &db.all_sheets,
        "Projects",
        AvailableColors::Green as u8,
        "",
    ));
    db.all_sheets.push(Sheet::new(
        &db.all_sheets,
        "Study",
        AvailableColors::Pink as u8,
        "",
    ));

    db.all_sheets.push(Sheet::new(
        &db.all_sheets,
        "Games",
        AvailableColors::Lavender as u8,
        "",
    ));
    db.all_sheets.push(Sheet::new(
        &db.all_sheets,
        "Books",
        AvailableColors::Magenta as u8,
        "",
    ));
    db.all_sheets.push(Sheet::new(
        &db.all_sheets,
        "Projects",
        AvailableColors::Green as u8,
        "",
    ));
    db.all_sheets.push(Sheet::new(
        &db.all_sheets,
        "Study",
        AvailableColors::Orange as u8,
        "",
    ));

    handle_round(db)
}
