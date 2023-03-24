mod colors;
mod database;
mod entries;
mod menus;
mod sheets;

use menus::create_select;

use database::Database;

fn setup_ranking(mut db: Database, sheet_i: usize) {
    let mut sheet = &mut db.all_sheets[sheet_i];
    sheet.entries = Database::picker_loop(sheet.entries.to_owned());
    db.all_sheets[sheet_i] = sheet.clone();
    db.save();
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
    let msg = "an option";
    let choices = vec![
        "View Sheet",
        "Rank Sheet",
        "Add Entry",
        "Delete  Sheet",
        "Delete Entry",
        "Quit",
    ];
    let selection_i = create_select(&choices, msg);

    match selection_i {
        0 => println!("Viewing sheet! not"),
        1 => setup_ranking(db, sheet_i),
        2 => db.create_entry(sheet_i),
        3 => println!("deletb"),
        4 => println!("deletb"),
        5 => println!("wowie"),
        _ => println!("cruel angel thesis"),
    }
}

fn main_menu(mut db: Database) {
    let msg = "???";
    let choices = vec!["Select sheet", "Create Sheet", "Quit"];
    let selection_i = create_select(&choices, msg);
    match selection_i {
        0 => select_sheet(db),
        1 => create_sheet(db),
        2 => println!("fuc"),
        _ => println!("cruel angel thesis"),
    }
}

fn main() {
    let db: Database = Database::load();
    //dbg!(db);
    main_menu(db);
}
