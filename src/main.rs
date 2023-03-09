pub mod mods;

use dialoguer::{theme::ColorfulTheme, theme::SimpleTheme, Input, MultiSelect, Select};
use log::debug;
use mods::{AvailableColors, Database, Entry, EntryBag, Sheet};
use rand::thread_rng;
use std::cmp::Ordering;
use std::fs::File;

use std::vec;

fn handle_round(mut db: Database) {
    println!("2");
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

///
/// Takes a list of entries and split it into 3 categories
fn categorize_entries(entries: Vec<Entry>) -> (Vec<Entry>, Vec<Entry>, Vec<Entry>) {
    let mut losers: Vec<Entry> = vec![];
    let mut ranked: Vec<Entry> = vec![];
    let mut survivors: Vec<Entry> = vec![];
    for entry in entries {
        if entry.rank > 0 {
            ranked.push(entry);
        } else if entry.get_lost_len() > 0 {
            losers.push(entry);
        } else {
            survivors.push(entry);
        }
    }
    (survivors, losers, ranked)
}

fn merge_entry_vecs(
    survivors: &mut Vec<Entry>,
    losers: &mut Vec<Entry>,
    ranked: &mut Vec<Entry>,
) -> Vec<Entry> {
    let mut entries: Vec<Entry> = vec![];
    entries.append(ranked);
    entries.append(losers);
    entries.append(survivors);
    entries
}

fn picker_setup(mut sheet_entries: Vec<Entry>) -> Vec<Entry> {
    let (mut survivors, mut losers, mut ranked) = categorize_entries(sheet_entries);
    let mut is_processed = false;
    while !is_processed {
        let mut quit_bool: bool = false;
    }
}

fn check_for_finished_round(
    survivors: Vec<Entry>,
    losers: Vec<Entry>,
    ranked: Vec<Entry>,
) -> (Vec<Entry>, Vec<Entry>, Vec<Entry>) {
    if survivors.len() == 1 {
        let highest_rank = ranked.len() + 1;
        let mut winner = survivors[0];
        let mut released_entries: Vec<Entry> = losers
            .into_iter()
            .filter(|e| e.lost_against.contains(&winner.id))
            .collect();
        for mut entry in released_entries {
            let index: usize = entry
                .lost_against
                .iter()
                .position(|id| id == &winner.id)
                .unwrap();
            entry.lost_against.remove(index);
        }
        let new_losers: Vec<Entry> = losers
            .into_iter()
            .filter(|e| e.get_lost_len() == 0)
            .collect();

        survivors.append(&mut released_entries);
        ranked.push(winner);
        (survivors, new_losers, ranked)
    } else {
        (survivors, losers, ranked)
    }
}

fn remove_ranked_from_losers() {}

fn picker(entries: (bool, Vec<Entry>)) -> (bool, Vec<Entry>) {
    //if the entries are too long, chunk it and recurse
    let (quit_bool, entries) = entries;
    if entries.len() > 10 {
        let mut processed_entries: Vec<Entry> = Vec::new();
        let mut v_chunked: Vec<Vec<Entry>> = entries.chunks(10).map(|x| x.to_vec()).collect();

        dbg!(v_chunked.len());
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

fn mult_menu_creation<T: std::fmt::Display + std::fmt::Debug>(
    choices: &[T],
    msg: &str,
) -> Vec<usize> {
    println!("multmenu creation");
    dbg!(choices);
    let selection_i = MultiSelect::with_theme(&SimpleTheme)
        .with_prompt(format!("Pick your {msg} (use space)"))
        .items(&choices)
        .interact()
        .unwrap();

    selection_i
}

fn debug_db(mut db: Database) -> Database {
    let entry_vec: Vec<Entry> = vec![
        Entry {
            id: 1,
            name: "Pikachu".to_string(),
            color: AvailableColors::Red as u8,
            note: "".to_string(),
            rank: 0,
            lost_against: vec![],
        },
        Entry {
            id: 2,
            name: "Pichu".to_string(),
            color: AvailableColors::Bluish as u8,
            note: "".to_string(),
            rank: 0,
            lost_against: vec![],
        },
        Entry {
            id: 3,
            name: "Mimikyu".to_string(),
            color: AvailableColors::Purple as u8,
            note: "".to_string(),
            rank: 0,
            lost_against: vec![],
        },
        Entry {
            id: 4,
            name: "Drampa".to_string(),
            color: AvailableColors::Pink as u8,
            note: "".to_string(),
            rank: 0,
            lost_against: vec![],
        },
        Entry {
            id: 5,
            name: "Kyogre".to_string(),
            color: AvailableColors::Pink as u8,
            note: "".to_string(),
            rank: 0,
            lost_against: vec![],
        },
        Entry {
            id: 6,
            name: "Hydregon".to_string(),
            color: AvailableColors::Bluish as u8,
            note: "".to_string(),
            rank: 0,
            lost_against: vec![],
        },
        Entry {
            id: 7,
            name: "Illimuse".to_string(),
            color: AvailableColors::Green as u8,
            note: "".to_string(),
            rank: 0,
            lost_against: vec![],
        },
        Entry {
            id: 8,
            name: "Gardevoir".to_string(),
            color: AvailableColors::Red as u8,
            note: "".to_string(),
            rank: 0,
            lost_against: vec![],
        },
        Entry {
            id: 9,
            name: "Ralts".to_string(),
            color: AvailableColors::Purple as u8,
            note: "".to_string(),
            rank: 0,
            lost_against: vec![],
        },
        Entry {
            id: 10,
            name: "Wailord".to_string(),
            color: AvailableColors::Green as u8,
            note: "".to_string(),
            rank: 0,
            lost_against: vec![],
        },
        Entry {
            id: 11,
            name: "Marcago".to_string(),
            color: AvailableColors::Red as u8,
            note: "".to_string(),
            rank: 0,
            lost_against: vec![],
        },
        Entry {
            id: 12,
            name: "Munkidori".to_string(),
            color: AvailableColors::Bluish as u8,
            note: "".to_string(),
            rank: 0,
            lost_against: vec![],
        },
        Entry {
            id: 13,
            name: "Okidogi".to_string(),
            color: AvailableColors::Purple as u8,
            note: "".to_string(),
            rank: 0,
            lost_against: vec![],
        },
        Entry {
            id: 14,
            name: "Palkia".to_string(),
            color: AvailableColors::Pink as u8,
            note: "".to_string(),
            rank: 0,
            lost_against: vec![],
        },
        Entry {
            id: 15,
            name: "Rattacate".to_string(),
            color: AvailableColors::Pink as u8,
            note: "".to_string(),
            rank: 0,
            lost_against: vec![],
        },
        Entry {
            id: 16,
            name: "Dedenne".to_string(),
            color: AvailableColors::Bluish as u8,
            note: "".to_string(),
            rank: 0,
            lost_against: vec![],
        },
        Entry {
            id: 17,
            name: "Absol".to_string(),
            color: AvailableColors::Green as u8,
            note: "".to_string(),
            rank: 0,
            lost_against: vec![],
        },
        Entry {
            id: 18,
            name: "Wormadam".to_string(),
            color: AvailableColors::Red as u8,
            note: "".to_string(),
            rank: 0,
            lost_against: vec![],
        },
        Entry {
            id: 19,
            name: "Togekiss".to_string(),
            color: AvailableColors::Purple as u8,
            note: "".to_string(),
            rank: 0,
            lost_against: vec![],
        },
        Entry {
            id: 20,
            name: "Steelix".to_string(),
            color: AvailableColors::Green as u8,
            note: "".to_string(),
            rank: 0,
            lost_against: vec![],
        },
    ];
    db.all_sheets.push(Sheet::new_debug(
        1,
        "Games",
        AvailableColors::Green as u8,
        "note!",
        &mut entry_vec.to_owned(),
    ));
    db.all_sheets.push(Sheet::new(
        &db.all_sheets,
        "Books",
        AvailableColors::Green as u8,
        "",
    ));
    db
}

fn main() {
    let db: Database = load_db();
    let db = debug_db(db);
    handle_round(db);
    println!("1");
}
