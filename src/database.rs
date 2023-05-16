use crate::colors::AvailableColors;
use crate::entries::Entry;
use crate::menus::{confirm, create_select, create_validated_multi_select};
use crate::sheets::Sheet;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use enum_iterator::all;
use home::home_dir;
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, File};
use std::io::{Error, Write};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    pub all_sheets: Vec<Sheet>,
}

impl Database {
    pub fn pick_sheet_idx(&self) -> usize {
        create_select(&self.all_sheets, "sheet")
    }
    pub fn save(&self) -> Result<(), Error> {
        let home = home_dir().expect("could not find home dir");
        let save_path = PathBuf::from(".local/share/delphea/delphea_db.json".to_string());
        let path = home.join(save_path);

        let db_json = serde_json::to_string(self).unwrap();
        create_dir_all(home.join(".local/share/delphea/"))?;
        let mut output = File::create(path)?;
        match write!(output, "{}", db_json) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn load() -> Database {
        let home = home_dir().expect("could not find home dir");
        let save_path = PathBuf::from(".local/share/delphea/delphea_db.json".to_string());
        let path = home.join(save_path);
        match File::open(path) {
            Ok(file) => {
                let db: Database =
                    serde_json::from_reader(file).expect("error while reading or parsing");
                db
            }
            Err(err) => {
                //Database { all_sheets: vec![] }
                println!("error finding sheets: {}", err);
                let mut new_db = Database { all_sheets: vec![] };
                new_db.create_sheet();
                new_db
            }
        }
    }
    pub fn delete_sheet(&mut self) {
        let sheet_idx = self.pick_sheet_idx();
        let sheet_name = &self.all_sheets[sheet_idx].name.to_owned();

        if let Ok(choice) = confirm(&format!("Are you sure you want to delete {}", sheet_name)) {
            match choice {
                true => {
                    self.all_sheets.swap_remove(sheet_idx);
                    match self.save() {
                        Ok(_) => (),
                        Err(e) => print!("{e}"),
                    }
                    println!("Sheet deleted!")
                }
                false => (),
            }
        }
    }
    pub fn delete_entry(&mut self, sheet_idx: usize) {
        self.all_sheets[sheet_idx].delete_entry();
        match self.save() {
            Ok(_) => (),
            Err(e) => print!("{e}"),
        }
    }

    pub fn create_sheet(&mut self) {
        let sheet_len = self.all_sheets.len();
        let (name, color, note) = Sheet::interactive_create_root("Sheet");
        let sheet = Sheet::new(sheet_len, &name, color, &note);
        self.all_sheets.push(sheet);
        match self.save() {
            Ok(_) => (),
            Err(e) => print!("{e}"),
        }
    }

    pub fn create_entry(&mut self, sheet_i: usize) {
        let entry_len = self.all_sheets[sheet_i].entries.len();
        self.all_sheets[sheet_i].interactive_create_entry(entry_len);
        match self.save() {
            Ok(_) => (),
            Err(e) => print!("{e}"),
        }
    }
    pub fn edit_entry(&mut self, sheet_i: usize) {
        let entry_i = create_select(&self.all_sheets[sheet_i].entries, "Select an entry to edit");
        self.all_sheets[sheet_i].interactive_edit_entry("Entry", entry_i);
        match self.save() {
            Ok(_) => (),
            Err(e) => print!("{e}"),
        }
    }
    pub fn interactive_edit_sheet(&mut self, original_sheet_i: usize) {
        let original_sheet = &self.all_sheets[original_sheet_i];
        let colors = all::<AvailableColors>().collect::<Vec<_>>();
        let color_i = if let Some(color_i) = colors.iter().position(|c| {
            let color = c.to_owned();
            color as u8 == original_sheet.color
        }) {
            color_i
        } else {
            0 as usize
        };

        let name: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("Enter this sheet's name"))
            .with_initial_text(original_sheet.name.to_owned())
            .interact()
            .unwrap();
        let color_idx: usize = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Pick a color")
            .items(&colors)
            .default(color_i)
            .interact()
            .unwrap();
        let color = colors[color_idx].clone();
        let note: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Any notes?")
            .allow_empty(true)
            .with_initial_text(original_sheet.note.to_owned())
            .interact()
            .unwrap();
        self.all_sheets[original_sheet_i] = Sheet {
            name,
            color: color as u8,
            id: original_sheet.id,
            note,
            entries: original_sheet.entries.to_owned(),
        }
    }

    pub fn create_entry_cli(&mut self, sheet_i: usize, entry_name: &str) {
        let entry_len = self.all_sheets[sheet_i].entries.len();
        let entry = Entry::new(entry_len, entry_name, AvailableColors::random() as u8, "");

        self.all_sheets[sheet_i].entries.push(entry);
        match self.save() {
            Ok(_) => (),
            Err(e) => print!("{e}"),
        }
    }

    pub fn picker_loop(sheet_entries: Vec<Entry>) -> Vec<Entry> {
        let (survivors, losers, mut ranked) = categorize_entries(sheet_entries);
        let mut is_processed;
        let mut quit_bool: bool = false;
        let mut processed_survivors: Vec<Entry> = survivors;
        let mut processed_losers: Vec<Entry> = losers;
        let processed_ranked: Vec<Entry> = ranked.to_owned();
        (processed_survivors, processed_losers, ranked) =
            Self::check_for_finished_round(processed_survivors, processed_losers, processed_ranked);
        is_processed = processed_survivors.is_empty();

        while !is_processed && !quit_bool {
            let mut returned_survivors: Vec<Entry> = vec![];
            let v_chunked: Vec<Vec<Entry>> =
                processed_survivors.chunks(11).map(|x| x.to_vec()).collect();
            for chunk in v_chunked {
                let mut picked_survivors;
                let mut picked_losers;
                (quit_bool, (picked_survivors, picked_losers)) = picker(chunk);
                processed_losers.append(&mut picked_losers);
                returned_survivors.append(&mut picked_survivors);
            }
            (processed_survivors, processed_losers, ranked) =
                Self::check_for_finished_round(returned_survivors, processed_losers, ranked);
            is_processed = processed_survivors.is_empty();
        }
        println!("DONE!!");
        merge_entry_vecs(&mut processed_survivors, &mut processed_losers, &mut ranked)
    }
    pub fn check_for_finished_round(
        mut survivors: Vec<Entry>,
        losers: Vec<Entry>,
        ranked: Vec<Entry>,
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
            //returned_survivors = dbg!(returned_survivors;
            (returned_survivors, returned_losers, returned_ranked)
        } else {
            (survivors, losers, ranked)
        }
    }
}

pub fn picker(survivors: Vec<Entry>) -> (bool, (Vec<Entry>, Vec<Entry>)) {
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
    let found_losers: Vec<Entry> = survivors
        .into_iter()
        .filter(|e| !selected_survivors.iter().any(|w| w.id == e.id))
        .collect();
    let winner_ids: Vec<usize> = selected_survivors
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

    for entry in &mut losers {
        entry.clear_winner(ranked_winner.id)
    }

    let released_entries: Vec<Entry> = losers
        .clone()
        .into_iter()
        .filter(|e| e.get_lost_len() == 0)
        .collect();

    println!("{} ranked at #{}", ranked_winner.clone(), highest_rank);
    ranked.push(ranked_winner);
    let new_losers: Vec<Entry> = losers
        .clone()
        .into_iter()
        .filter(|e| e.get_lost_len() != 0)
        .collect();
    (released_entries, new_losers, ranked)
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
    dbg!(entries.clone());
    entries
}
