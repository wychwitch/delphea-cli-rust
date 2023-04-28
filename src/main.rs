mod colors;
mod database;
mod debuginit;
mod entries;
mod menus;
mod sheets;

use menus::{confirm, create_select};

use database::Database;

// [ ] -
// 0.1.0 TODO
// [x] - View Entries
// [x] - Delete Entry
// [x] - Delete Sheet
// [x] - Rank Entries selection
// [x] - Rank entries confirmation
// [ ] - fix out display bug
// [ ] - fix adding then ranking single item bug
// 1.0.0 TODO
// [ ] - Gracefully handle errors
// [ ] - Sheet of sheets
// [ ] - export list

fn setup_ranking(mut db: Database, sheet_i: usize) {
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
                db.all_sheets[sheet_i] = sheet.clone();
            }
            2 => {
                sheet.clear_all_ranked();
                sheet.entries = Database::picker_loop(sheet.entries.to_owned());
                db.all_sheets[sheet_i] = sheet.clone();
            }
            _ => (),
        }
    } else {
        sheet.entries = Database::picker_loop(sheet.entries.to_owned());
        db.all_sheets[sheet_i] = sheet.clone();
    }
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
    let mut quit = false;
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
        0 => {
            db.all_sheets[sheet_i].view_entries();
            print!("how dis");
            sheet_menu(db, sheet_i);
        }
        1 => setup_ranking(db, sheet_i),
        2 => db.create_entry(sheet_i),
        3 => db.delete_sheet(),
        4 => db.delete_entry(sheet_i),
        _ => println!("cruel angel thesis"),
    }
    print!("owie");
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
