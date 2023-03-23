use crate::entries::Entry;
use crate::menus::{create_select, create_validated_multi_select};
use crate::sheets::Sheet;
use home::home_dir;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Error, Write};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    pub all_sheets: Vec<Sheet>,
}

impl Database {
    pub fn pick_sheet_idx(&self) -> usize {
        let sheet_id = create_select(&self.all_sheets, "sheet");
        sheet_id
    }
    pub fn save(&self) -> Result<(), Error> {
        let home = home_dir().expect("could not find home dir");
        let save_path = PathBuf::from(".local/share/delphea/delphea_db.json".to_string());
        let path = home.join(save_path);

        let db_json = serde_json::to_string(self).unwrap();
        let mut output = File::create(path)?;
        write!(output, "{}", db_json);
        Ok(())
    }

    pub fn load() -> Database {
        let home = home_dir().expect("could not find home dir");
        let save_path = PathBuf::from(".local/share/delphea/delphea_db.json".to_string());
        let path = home.join(save_path);
        let db = match File::open(path) {
            Ok(file) => {
                let db: Database =
                    serde_json::from_reader(file).expect("error while reading or parsing");
                db
            }
            Err(err) => {
                dbg!(err);
                Database { all_sheets: vec![] }
            }
        };
        db
    }

    pub fn create_sheet(&mut self) {
        let sheet = Sheet::interactive_create(self.all_sheets.len());
        self.all_sheets.push(sheet);
        self.save();
    }
    pub fn picker_loop(mut sheet_entries: Vec<Entry>) -> Vec<Entry> {
        let (mut survivors, mut losers, mut ranked) = categorize_entries(sheet_entries);
        let mut is_processed = false;
        let mut quit_bool: bool = false;
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
            (processed_survivors, processed_losers, ranked) =
                Self::check_for_finished_round(returned_survivors, processed_losers, ranked);
            is_processed = processed_survivors.len() == 0;
        }
        println!("DONE!!");
        merge_entry_vecs(&mut survivors, &mut losers, &mut ranked)
    }
    pub fn check_for_finished_round(
        mut survivors: Vec<Entry>,
        mut losers: Vec<Entry>,
        mut ranked: Vec<Entry>,
    ) -> (Vec<Entry>, Vec<Entry>, Vec<Entry>) {
        let mut returned_survivors: Vec<Entry>;
        let mut returned_losers: Vec<Entry>;
        let mut returned_ranked: Vec<Entry>;
        if survivors.len() == 1 {
            let mut winner = survivors.pop().unwrap();
            (returned_survivors, returned_losers, returned_ranked) =
                process_winner(winner, losers, ranked);
            while returned_survivors.len() == 1 {
                winner = returned_survivors.pop().unwrap();
                (returned_survivors, returned_losers, returned_ranked) =
                    process_winner(winner, returned_losers, returned_ranked);
                // panic!("jumped into here!");
            }
            //returned_survivors = dbg!(returned_survivors);
            returned_ranked = dbg!(returned_ranked);
            (returned_survivors, returned_losers, returned_ranked)
        } else {
            (survivors, losers, ranked)
        }
    }
}

pub fn picker(survivors: Vec<Entry>) -> (bool, (Vec<Entry>, Vec<Entry>)) {
    let mut quit_bool = false;
    let selection_result: Result<Vec<usize>, String> =
        create_validated_multi_select(survivors.as_slice(), "entries");
    let selection = match selection_result {
        Ok(selec) => selec,
        Err(msg) => panic!("{}", msg),
    };
    let selected_survivors: Vec<Entry> = selection
        .into_iter()
        .map(|s| survivors[s].clone())
        .collect();
    let mut found_losers: Vec<Entry> = survivors
        .into_iter()
        .filter(|e| !selected_survivors.iter().any(|w| w.id == e.id))
        .collect();
    let mut winner_ids: Vec<usize> = selected_survivors
        .clone()
        .into_iter()
        .map(|w| w.id)
        .collect();
    let found_losers = register_winners(winner_ids, found_losers);

    (false, (selected_survivors, found_losers))
}
pub fn process_winner(
    mut ranked_winner: Entry,
    mut losers: Vec<Entry>,
    mut ranked: Vec<Entry>,
) -> (Vec<Entry>, Vec<Entry>, Vec<Entry>) {
    let highest_rank = ranked.len() + 1;
    ranked_winner.rank = highest_rank;
    ranked_winner.lost_against = vec![];

    for mut entry in &mut losers {
        entry.clear_winner(ranked_winner.id)
    }

    let released_entries: Vec<Entry> = losers
        .clone()
        .into_iter()
        .filter(|e| e.get_lost_len() == 0)
        .collect();

    ranked.push(ranked_winner);
    //dbg!(losers.clone());
    //THIS IS HWE PROBLEMDASXJGHDSHJGSD
    let new_losers: Vec<Entry> = losers
        .clone()
        .into_iter()
        .filter(|e| e.get_lost_len() != 0)
        .collect();
    //dbg!(new_losers.clone());
    //dbg!(ranked.clone());
    (released_entries.clone(), new_losers, ranked)
}
pub fn register_winners(winner_ids: Vec<usize>, mut losers: Vec<Entry>) -> Vec<Entry> {
    for loser in losers.as_mut_slice() {
        let mut cloned_ids = winner_ids.clone();
        loser.lost_against.append(&mut cloned_ids);
    }
    losers
}
///
/// Takes a list of entries and split it into 3 categories
pub fn categorize_entries(entries: Vec<Entry>) -> (Vec<Entry>, Vec<Entry>, Vec<Entry>) {
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

pub fn merge_entry_vecs(
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
