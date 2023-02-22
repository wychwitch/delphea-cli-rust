pub mod mods;

use dialoguer::{theme::ColorfulTheme, Input, MultiSelect, Select};
use mods::{AvailableColors, Database, Entry, EntryBag, Sheet};
use rand::thread_rng;
use std::cmp::Ordering;
use std::fs::File;

use std::vec;

fn handle_round(mut db: Database) {
    let sheet_idx = db.pick_sheet_idx();
    let mut sheet = &mut db.all_sheets[sheet_idx];
    sheet.debug_add_entries(&mut db.all_entries);
    sheet.entries = picker_setup(sheet.entries.to_owned());
    db.all_sheets[sheet_idx] = sheet.clone();
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

fn picker_setup(mut sheet_entries: Vec<Entry>) -> Vec<Entry> {
    let mut entry_bag_list = entry_bag_packer(sheet_entries);
    let mut is_processed = false;
    while !is_processed {
        for mut bag in &mut entry_bag_list[..] {
            (_, bag.entries) = picker((false, bag.entries.to_owned()));
        }
        is_processed = entry_bag_list
            .clone()
            .into_iter()
            .all(|e| e.is_all_ranked());
        if !is_processed {
            sheet_entries = entry_bag_unpacker(entry_bag_list);
            entry_bag_list = entry_bag_packer(sheet_entries);
        }
    }
    entry_bag_unpacker(entry_bag_list)
}

fn entry_bag_packer(entries: Vec<Entry>) -> Vec<EntryBag> {
    let mut entry_bag_list: Vec<EntryBag> = vec![];
    for entry in entries {
        let entry_bag = entry_bag_list
            .clone()
            .into_iter()
            .find(|eb| eb.loss_len == entry.get_lost_len());
        match entry_bag {
            Some(mut eb) => eb.new_entry(entry),
            None => {
                let eb = EntryBag {
                    len: 1,
                    loss_len: entry.get_lost_len(),
                    entries: vec![entry],
                };
                entry_bag_list.push(eb);
            }
        }
    }
    entry_bag_list.sort_by(|a, b| a.loss_len.cmp(&b.loss_len));
    entry_bag_list
}

fn entry_bag_unpacker(mut entry_bag_list: Vec<EntryBag>) -> Vec<Entry> {
    let mut entries = Vec::new();

    for mut bag in entry_bag_list {
        let mut bag_entries = bag.entries;
        entries.append(&mut bag_entries)
    }
    entries
}

fn picker(entries: (bool, Vec<Entry>)) -> (bool, Vec<Entry>) {
    //if the entries are too long, chunk it and recurse
    let (quit_bool, entries) = entries;
    if entries.len() > 10 {
        let mut processed_entries: Vec<Entry> = Vec::new();
        let mut v_chunked: Vec<Vec<Entry>> = entries.chunks(10).map(|x| x.to_vec()).collect();

        for i in 0..v_chunked.len() {
            let v_chunk = &v_chunked[i];
            let (quit_bool, mut entries) = picker((quit_bool, v_chunk.to_owned()));
            processed_entries.append(&mut entries);
            if quit_bool {
                for y in i..v_chunked.len() {
                    let mut v_chunk = &mut v_chunked[y];
                    processed_entries.append(&mut v_chunk);
                }
                break;
            }
        }
        (quit_bool, processed_entries)
    } else {
        let selection: Vec<usize> = mult_menu_creation(entries.as_slice(), "entries");
        let winners: Vec<Entry> = selection.into_iter().map(|s| entries[s].clone()).collect();
        let mut losers: Vec<Entry> = entries
            .into_iter()
            .filter(|e| !winners.iter().any(|w| w.id == e.id))
            .collect();
        for loser in losers.as_mut_slice() {
            let mut winner_vec: Vec<i32> = winners.clone().into_iter().map(|w| w.id).collect();
            loser.lost_against.append(&mut winner_vec);
        }

        let mut collected_entries = winners.clone();
        let mut cloned_losers = losers.clone();
        collected_entries.append(&mut cloned_losers);

        (false, collected_entries)
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
