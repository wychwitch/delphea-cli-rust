mod colors;
mod database;
mod entries;
mod menus;
mod sheets;

use database::Database;

fn setup_round(mut db: Database) {
    let sheet_idx = db.pick_sheet_idx();
    let mut sheet = &mut db.all_sheets[sheet_idx];
    sheet.entries = Database::picker_loop(sheet.entries.to_owned());
    db.all_sheets[sheet_idx] = sheet.clone();
    db.save();
}

fn main() {
    let db: Database = Database::load_db();
    setup_round(db);
}
