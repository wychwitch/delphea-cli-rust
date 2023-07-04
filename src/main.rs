mod colors;
mod database;
mod debuginit;
mod entries;
mod menus;
mod sheets;
use clap::{Command, Parser, Subcommand};
use database::Database;
use menus::{confirm, create_select};
use std::env;

// [ ] -
// 0.1.0 TODO
// [x] - View Entries
// [x] - Delete Entry
// [x] - Delete Sheet
// [x] - Rank Entries selection
// [x] - Rank entries confirmation
// [x] - fix out display bug
// [x] - fix adding then ranking single item bug
// [x] - remove unused dependencies
// [ ] - Gracefully handle errors
// 0.2.0 TODO
// [ ] - Todo type of Sheets
// [ ] - import as list
// [ ] - Sheet of sheets
// [ ] - export list
// 1.0.0 TODO
// [ ] - complete seperating lib from cli application

fn setup_ranking(db: &mut Database, sheet_i: usize) {
    let mut sheet = &mut db.all_sheets[sheet_i];
    if sheet.check_if_all_ranked() {
        let confirm = menus::confirm(
            "Looks like this sheet is already fuly ranked. Do you want to rerank everything?",
        )
        .unwrap();
        if confirm {
            sheet.clear_all_ranked();
            sheet.entries = Database::picker_loop(sheet.entries.to_owned());
            db.all_sheets[sheet_i] = sheet.clone();
        }
    } else if !sheet.check_if_all_unranked() {
        let choices = vec!["quit", "finish ranking", "rerank everything"];
        let choice = menus::create_select(
            &choices,
            "Looks like this was partially ranked. What do you want to do?",
        );
        match choice {
            1 => {
                sheet.entries = Database::picker_loop(sheet.entries.to_owned());
                sheet.view_entries();
                db.all_sheets[sheet_i] = sheet.clone();
            }
            2 => {
                sheet.clear_all_ranked();
                sheet.entries = Database::picker_loop(sheet.entries.to_owned());
                sheet.view_entries();
                db.all_sheets[sheet_i] = sheet.clone();
            }
            _ => (),
        }
    } else {
        sheet.entries = Database::picker_loop(sheet.entries.to_owned());
        db.all_sheets[sheet_i] = sheet.clone();
    }
    match db.save() {
        Ok(_) => (),
        Err(e) => print!("{e}"),
    }
}

fn create_sheet(mut db: Database) {
    db.create_sheet();
    main_menu(db);
}

fn select_sheet(db: Database) {
    let sheet_i = db.pick_sheet_idx();
    sheet_menu(db, sheet_i);
}

fn sheet_menu(mut db: Database, sheet_i: usize) {
    let sheet = &db.all_sheets[sheet_i];
    let msg = "an option";
    let choices = vec![
        format!("View {sheet} sheet").to_owned(),
        format!("Rank {sheet} sheet").to_owned(),
        "Add an Entry".to_owned(),
        "Edit an Entry".to_owned(),
        format!("Edit {sheet} sheet").to_owned(),
        format!("Delete {sheet} sheet").to_owned(),
        "Delete an Entry".to_owned(),
        "Quit".to_owned(),
    ];
    let selection_i = create_select(&choices, msg);

    match selection_i {
        0 => {
            db.all_sheets[sheet_i].view_entries();
            sheet_menu(db, sheet_i);
        }
        1 => {
            setup_ranking(&mut db, sheet_i);
            sheet_menu(db, sheet_i);
        }
        2 => {
            db.create_entry(sheet_i);
            sheet_menu(db, sheet_i);
        }
        3 => {
            db.edit_entry(sheet_i);
            sheet_menu(db, sheet_i);
        }
        4 => {
            edit_sheet(&mut db, sheet_i);
            sheet_menu(db, sheet_i);
        }
        5 => {
            db.delete_sheet();
            sheet_menu(db, sheet_i);
        }
        6 => {
            db.delete_entry(sheet_i);
            sheet_menu(db, sheet_i);
        }
        _ => main_menu(db),
    }
}

fn delete_sheet(mut db: Database) {
    let sheet_idx = db.pick_sheet_idx();
    let sheet_name = &db.all_sheets[sheet_idx].name;
    match confirm(&format!("Are you sure you want to delete {}", sheet_name)) {
        Ok(choice) => match choice {
            true => {
                db.all_sheets.swap_remove(sheet_idx);
                match db.save() {
                    Ok(_) => (),
                    Err(e) => print!("{e}"),
                }
                println!("Sheet deleted!")
            }
            false => main_menu(db),
        },
        Err(_) => main_menu(db),
    }
}
pub fn edit_sheet(db: &mut Database, sheet_i: usize) {
    db.interactive_edit_sheet(sheet_i);
    match db.save() {
        Ok(_) => (),
        Err(e) => print!("{e}"),
    }
}

fn main_menu(db: Database) {
    let msg = "option";
    let choices = vec!["Select sheet", "Create Sheet", "delete sheet", "Quit"];
    let selection_i = create_select(&choices, msg);
    match selection_i {
        0 => select_sheet(db),
        1 => create_sheet(db),
        2 => delete_sheet(db),
        _ => println!("byebye"),
    }
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help = false)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Add {
        /// The Sheet to add the entry to
        #[arg(short, long, value_name = "SHEET")]
        sheet: Option<String>,

        /// The entry name
        #[arg(short, long, value_name = "ENTRY")]
        entry: String,
    },
    Import {
        /// The Sheet to add the entry to
        #[arg(short, long, value_name = "SHEET")]
        sheet: Option<String>,

        /// Path to file to import
        #[arg(short, long, value_name = "PATH")]
        path: Option<String>,
    },
}

fn cli_commands(command: Commands, db: &mut Database) {
    match command {
        Commands::Add { sheet, entry } => {
            let sheet_i = match sheet.as_deref() {
                Some(sheet) => db
                    .all_sheets
                    .iter()
                    .position(|s| s.name.to_lowercase() == sheet.to_lowercase()),
                None => match env::var("DELPHEA_SHEET") {
                    Ok(sheet) => db
                        .all_sheets
                        .iter()
                        .position(|s| s.name.to_lowercase() == sheet.to_lowercase()),
                    Err(_) => None,
                },
            };
            match sheet_i {
                Some(sheet_i) => {
                    db.create_entry_cli(sheet_i, &entry);
                    let name = &db.all_sheets[sheet_i];
                    println!("Added {entry} to the {name} sheet!");
                }
                None => println!("Sheet not found."),
            }
        }
        Commands::Import { sheet, path } => {
            println!("Import command!");
        }
    }
}

fn main() {
    let cli = Cli::parse();
    let mut db: Database = Database::load();
    match cli.command {
        Some(command) => cli_commands(command, &mut db),
        None => main_menu(db),
    }
}
