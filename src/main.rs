pub mod mods;

use mods::{AvailableColors, Database, Sheet};

use rand::thread_rng;
use std::fs::File;

use std::vec;

fn handle_round(mut db: Database) {
    let mut rng = thread_rng();
    let sheet_idx = db.pick_sheet_idx();
    let mut sheet = &mut db.all_sheets[sheet_idx];
    sheet.picker(&db.all_entries)
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
