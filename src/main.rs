use rand::{seq::IteratorRandom, thread_rng};

#[derive(Clone)]
struct Entry {
    id: i32,
    sheet_id: i32,
    name: String,
    color: String,
    won_against: Vec<i32>,
    note: String,
    favorited: bool,
}

#[derive(Clone)]
struct Sheet {
    id: i32,
    name: String,
    color: String,
    note: String,
}

impl Entry {
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

    pub fn id_to_entry(entry_id: i32, all_entries: Vec<Entry>) -> Entry {
        all_entries
            .into_iter()
            .find(|entry| entry.id == entry_id)
            .unwrap()
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
    pub fn get_entries(&self, all_entries: &Vec<Entry>) -> Vec<Entry> {
        let filtered = all_entries
            .clone()
            .into_iter()
            .filter(|entry| entry.sheet_id == self.id)
            .collect::<Vec<Entry>>()
            .to_vec();
        filtered
    }
    pub fn clear_all_favorites(&mut self, all_entries: Vec<Entry>) {
        let all_sheet_entries = self.get_entries(&all_entries);
        for mut entry in all_sheet_entries {
            entry.clear_wins();
        }
    }
    pub fn display_choices(
        &mut self,
        random_entries: &Vec<Entry>,
        all_sheet_entries: &Vec<Entry>,
    ) -> Vec<Entry> {
        //something something display choices and get choices
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
    pub fn picker(&mut self, mut all_entries: Vec<Entry>) {
        let mut rng = thread_rng();
        let mut filtered_entries = self.get_entries(&all_entries);
        while filtered_entries.len() != 0 {
            let random_entries = filtered_entries.into_iter().choose_multiple(&mut rng, 20);
            let picked_entries = self.display_choices(&random_entries, &all_entries);

            let cleaned = filtered_entries
                .into_iter()
                .filter(|entry| !entry.favorited)
                .collect();

            filtered_entries = cleaned;
        }
    }
}

fn main() {}
