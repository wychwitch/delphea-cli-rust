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
    let mut quit_bool: bool = false;
    while !is_processed && !quit_bool {
        (quit_bool, (survivors, losers)) = picker(survivors, losers);
        (is_processed, (survivors, losers, ranked)) =
            check_for_finished_round(survivors, losers, ranked);
    }
    merge_entry_vecs(&mut survivors, &mut losers, &mut ranked)
}

fn register_winners(winner_ids: &mut Vec<i32>, mut losers: Vec<Entry>) -> Vec<Entry> {
    for loser in losers.as_mut_slice() {
        loser.lost_against.append(winner_ids);
    }
    losers
}

fn check_for_finished_round(
    mut survivors: Vec<Entry>,
    mut losers: Vec<Entry>,
    mut ranked: Vec<Entry>,
) -> (bool, (Vec<Entry>, Vec<Entry>, Vec<Entry>)) {
    if survivors.len() == 1 {
        dbg!("IN FINISHED ROUND");
        let highest_rank = ranked.len() + 1;
        let mut ranked_winner = survivors[0].clone();
        ranked_winner.rank = highest_rank;
        let mut released_entries: Vec<Entry> = losers
            .clone()
            .into_iter()
            .filter(|e| e.lost_against.contains(&ranked_winner.id))
            .collect();
        for mut entry in &mut released_entries {
            let index: usize = entry
                .lost_against
                .iter()
                .position(|id| id == &ranked_winner.id)
                .unwrap();
            entry.lost_against.remove(index);
        }
        let new_losers: Vec<Entry> = losers
            .into_iter()
            .filter(|e| e.get_lost_len() != 0)
            .collect();
        let mut edited_released = released_entries.clone();
        dbg!(released_entries.clone());
        survivors.append(&mut edited_released);
        ranked.push(ranked_winner);
        (false, (survivors, new_losers, ranked))
    } else {
        (false, (survivors, losers, ranked))
    }
}

fn picker(survivors: Vec<Entry>, mut losers: Vec<Entry>) -> (bool, (Vec<Entry>, Vec<Entry>)) {
    //if the entries are too long, chunk it and recurse
    let mut quit_bool = false;
    if survivors.len() > 10 {
        let mut processed_survivors: Vec<Entry> = Vec::new();
        let mut processed_losers: Vec<Entry> = losers.to_owned();
        let mut v_chunked: Vec<Vec<Entry>> = survivors.chunks(10).map(|x| x.to_vec()).collect();

        dbg!(v_chunked.len());
        for i in 0..v_chunked.len() {
            let mut v_chunk = &mut v_chunked[i];
            let (quit_bool, (mut returned_survivors, mut returned_losers)) =
                picker(v_chunk.to_owned(), processed_losers.to_owned());
            processed_losers.append(&mut returned_losers);
            processed_survivors.append(&mut returned_survivors);
            if quit_bool {
                for y in i..v_chunked.len() {
                    let mut v_chunk = &mut v_chunked[y];
                    processed_survivors.append(&mut v_chunk);
                }
                break;
            }
        }
        (quit_bool, (processed_survivors, processed_losers))
    } else {
        let selection: Vec<usize> = mult_menu_creation(survivors.as_slice(), "entries");
        let selected_survivors: Vec<Entry> = selection
            .into_iter()
            .map(|s| survivors[s].clone())
            .collect();
        let mut found_losers: Vec<Entry> = survivors
            .into_iter()
            .filter(|e| !selected_survivors.iter().any(|w| w.id == e.id))
            .collect();
        let mut winner_ids: Vec<i32> = selected_survivors
            .clone()
            .into_iter()
            .map(|w| w.id)
            .collect();
        let found_losers = register_winners(&mut winner_ids, found_losers);

        (false, (selected_survivors, found_losers))
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
