mod colors;
mod database;
mod entries;
mod menus;
mod sheets;

use menus::{confirm, create_select};

use database::Database;

// [ ] -
// 0.1.0 TODO
// [ ] - View Entries
// [x] - Delete Entry
// [x] - Delete Sheet
// [ ] - Rank Entries selection
// [ ] - Rank entries confirmation
// 1.0.0 TODO
// [ ] - Gracefully handle errors
// [ ] - Sheet of sheets
// [ ] - export list

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
        "Delete Sheet",
        "Delete Entry",
        "Quit",
    ];
    let selection_i = create_select(&choices, msg);

    match selection_i {
        0 => println!("Viewing sheet! not"),
        1 => setup_ranking(db, sheet_i),
        2 => db.create_entry(sheet_i),
        3 => println!("deletb"),
        4 => db.delete_sheet(),
        5 => db.delete_entry(sheet_i),
        _ => println!("cruel angel thesis"),
    }
}

fn edit_sheet(mut db: Database) {
    print!("Doh!");
    main_menu(db);
}

fn delete_sheet(mut db: Database) {
    let sheet_idx = db.pick_sheet_idx();
    let sheet_name = &db.all_sheets[sheet_idx].name;
    match confirm(&format!("Are you sure you want to delete {}", sheet_name)) {
        Ok(choice) => match choice {
            true => {
                db.all_sheets.swap_remove(sheet_idx);
                db.save();
                println!("Sheet deleted!")
            }
            false => main_menu(db),
        },
        Err(_) => main_menu(db),
    }
}

fn main_menu(mut db: Database) {
    let msg = "???";
    let choices = vec![
        "Select sheet",
        "Create Sheet",
        "edit sheet",
        "delete sheet",
        "Quit",
    ];
    let selection_i = create_select(&choices, msg);
    match selection_i {
        0 => select_sheet(db),
        1 => create_sheet(db),
        2 => edit_sheet(db),
        3 => delete_sheet(db),
        _ => println!("cruel angel thesis"),
    }
}

fn main() {
    let db: Database = Database::load();
    //dbg!(db);
    main_menu(db);
}
