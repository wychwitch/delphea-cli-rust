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
    dbg!(survivors.clone());
    dbg!(ranked.clone());
    dbg!(losers.clone());
    let mut processed_survivors: Vec<Entry> = survivors.to_owned();
    let mut processed_losers: Vec<Entry> = losers.to_owned();
    let mut processed_ranked: Vec<Entry> = ranked.to_owned();
    while !is_processed && !quit_bool {
        let mut returned_survivors: Vec<Entry> = vec![];
        let mut returned_losers: Vec<Entry> = vec![];
        let mut v_chunked: Vec<Vec<Entry>> =
            processed_survivors.chunks(10).map(|x| x.to_vec()).collect();
        for chunk in v_chunked {
            let mut picked_survivors;
            let mut picked_losers;
            (quit_bool, (picked_survivors, picked_losers)) = picker(chunk);
            processed_losers.append(&mut picked_losers);
            returned_survivors.append(&mut picked_survivors);
        }
        dbg!(returned_survivors.clone());
        dbg!(processed_losers.clone());
       (processed_survivors, processed_losers, ranked) = check_for_finished_round(returned_survivors, processed_losers, ranked); 
    }
    merge_entry_vecs(&mut survivors, &mut losers, &mut ranked)
}

fn register_winners(winner_ids: Vec<i32>, mut losers: Vec<Entry>) -> Vec<Entry> {
    let mut i = 0;
    for loser in losers.as_mut_slice() {
        let mut cloned_ids = winner_ids.clone();
        loser.lost_against.append(&mut cloned_ids);
        i += 1;
    }
    dbg!(i);
    dbg!(losers.clone());
    losers
}

fn check_for_finished_round(
    mut survivors: Vec<Entry>,
    mut losers: Vec<Entry>,
    mut ranked: Vec<Entry>,
) -> (Vec<Entry>, Vec<Entry>, Vec<Entry>) {
    dbg!(losers.clone());
    if survivors.len() == 1 {
        dbg!("IN Check 1");
        process_winners(survivors[0].clone(), losers, ranked)
        
    } 

    else if survivors.len() == 2 {
        let (returned_survivors, returned_losers, returned_ranked) = process_winners(survivors[0].clone(), losers, ranked);
        process_winners(survivors[1].clone(), returned_losers, returned_ranked)
    }
    else {
        (survivors, losers, ranked)
    }
}


fn process_winners(mut ranked_winner: Entry, losers: Vec<Entry>, mut ranked: Vec<Entry>) -> (Vec<Entry>, Vec<Entry>, Vec<Entry>){
    let highest_rank = ranked.len() + 1;
    ranked_winner.rank = highest_rank;

        let mut effected_entries: Vec<Entry> = losers
            .clone()
            .into_iter()
            .filter(|e| {
                e.lost_against.contains(&ranked_winner.id)
            })
            .collect();

        for entry in &mut effected_entries {
            let index: Option<usize> = entry
                .lost_against
                .iter()
                .position(|id| id == &ranked_winner.id);
            match index {
                Some(i) => {
                    entry.lost_against.remove(i);
                }
                None => (),
            };
        }

        let mut released_entries: Vec<Entry> = effected_entries 
            .clone()
            .into_iter()
            .filter(|e| {
                e.get_lost_len() == 0 && e.id != ranked_winner.id
            })
            .collect();

        ranked.push(ranked_winner);
        //dbg!(losers.clone());
        let new_losers: Vec<Entry> = losers
            .into_iter()
            .filter(|e| e.get_lost_len() != 0)
            .collect();
        //dbg!(released_entries.clone());
        //dbg!(new_losers.clone());
        //dbg!(ranked.clone());
        (released_entries.clone(), new_losers, ranked)
}

fn picker(survivors: Vec<Entry> ) -> (bool, (Vec<Entry>, Vec<Entry>)) {
    //if the entries are too long, chunk it and recurse
    let mut quit_bool = false;
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
    let found_losers = register_winners(winner_ids, found_losers);

    (false, (selected_survivors, found_losers))
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
