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
    sheet.picker(&mut db.all_entries);
}

fn main_menu(db: &mut Database) {}
fn handle_create(db: &mut Database) {}

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

fn picker_setup(entries: &mut Vec<Entry>) {
    entries.sort_by(|a, b| {
        if a.get_lost_len() > b.get_lost_len() {
            Ordering::Less
        } else if a.get_lost_len() == b.get_lost_len() {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    });

    let mut unique_losses: Vec<usize> = entries.iter_mut().map(|e| e.get_lost_len()).collect();

    unique_losses.dedup();

    let mut grouped_entries: Vec<Vec<usize>> = vec![];

    for loss_len in unique_losses {
        let mut grouped_entry_ids: Vec<usize> = vec![];
        let len1 = entries.len();
        for (i, entry) in entries.iter().enumerate(){
            if entry.get_lost_len() == loss_len{
            grouped_entry_ids.push(i);
            }
        }
        grouped_entries.push(grouped_entry_ids);
    }
    for i in 0..grouped_entries.len(){
        let mut entr_vec: Vec<&mut Entry>;
        for y in  0..grouped_entries[i].len(){
            entr_vec.push(&mut entries[grouped_entries[i][y]])
        }
        let mut entr = entr_vec.as_slice();
        picker(&mut entr);
    }
}

fn picker(entries: &mut [&mut Entry]) {
    //if the entries are too long, chunk it and recurse
    if entries.len() > 10 {
        let mut chunks = entries.chunks_mut(10);
        for chunk in &mut chunks {
            picker(chunk)
        }
    } else {
        let selection: Vec<usize> = mult_menu_creation(entries, "entries");
        let winner_ids: Vec<i32> = selection.into_iter().map(|s| entries[s].id).collect();
        let mut losers: Vec<&mut &mut Entry> = entries
            .iter_mut()
            .filter(|e| !winner_ids.contains(&e.id))
            .collect();
        for i in 0..losers.len() {
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
