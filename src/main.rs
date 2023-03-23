mod colors;
mod database;
mod entries;
mod menus;
mod sheets;

use menus::create_select;

use database::Database;

fn setup_round(mut db: Database) {
    let sheet_idx = db.pick_sheet_idx();
    let mut sheet = &mut db.all_sheets[sheet_idx];
    sheet.entries = Database::picker_loop(sheet.entries.to_owned());
    db.all_sheets[sheet_idx] = sheet.clone();
    db.save();
}

fn create_sheet(mut db: Database) {
    db.create_sheet();
    main_menu(db);
}

fn main_menu(mut db: Database) {
    let msg = "???";
    let choices = vec!["Select sheet", "Create Sheet", "Quit"];
    let selection_i = create_select(&choices, msg);
    match selection_i {
        0 => setup_round(db),
        1 => create_sheet(db),
        2 => println!("fuc"),
        _ => println!("cruel angel thesis"),
    }
}

fn main() {
    let db: Database = Database::load_db();
    //dbg!(db);
    main_menu(db);
}
