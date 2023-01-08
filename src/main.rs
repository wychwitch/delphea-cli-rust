pub mod models;
use console::Term;
use console::{Emoji, Style};
use dialoguer::{theme::ColorfulTheme, Input, MultiSelect, Select};
use enum_iterator::all;
use indicatif::ProgressBar;
use models::{AvailableColors, Database};
use rand::{seq::IteratorRandom, thread_rng};
use std::thread;
use std::time::Duration;
use std::{env, fmt};
use std::{io, vec};

use crate::models::Entry;
use crate::models::Sheet;

fn handleround(db: &mut Database) {
    let dbgSheet = &mut db.all_sheets[0];
    dbgSheet.picker(&mut db.all_entries)
}

fn main_menu(db: &mut Database) {}
fn handle_create(db: &mut Database) {}

fn load_db() -> Database {
    Database {
        all_entries: vec![],
        all_sheets: vec![],
    }
}

fn main() {
    let mut db: Database = load_db();

    db.all_sheets.push(Sheet::new(
        &db.all_sheets,
        "Games",
        AvailableColors::Green as usize,
        "",
    ));
    db.all_sheets.push(Sheet::new(
        &db.all_sheets,
        "Books",
        AvailableColors::Green as usize,
        "",
    ));
    db.all_sheets.push(Sheet::new(
        &db.all_sheets,
        "Projects",
        AvailableColors::Green as usize,
        "",
    ));
    db.all_sheets.push(Sheet::new(
        &db.all_sheets,
        "Study",
        AvailableColors::Pink as usize,
        "",
    ));

    db.all_sheets.push(Sheet::new(
        &db.all_sheets,
        "Games",
        AvailableColors::Lavender as usize,
        "",
    ));
    db.all_sheets.push(Sheet::new(
        &db.all_sheets,
        "Books",
        AvailableColors::Magenta as usize,
        "",
    ));
    db.all_sheets.push(Sheet::new(
        &db.all_sheets,
        "Projects",
        AvailableColors::Green as usize,
        "",
    ));
    db.all_sheets.push(Sheet::new(
        &db.all_sheets,
        "Study",
        AvailableColors::Orange as usize,
        "",
    ));

    handleround(&mut db)
}
