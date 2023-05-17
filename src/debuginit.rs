use crate::colors::AvailableColors;
use crate::database::Database;
use crate::entries::Entry;
use crate::sheets::Sheet;

pub fn _debug_db(mut db: Database) -> Database {
    let mut entry_vec: Vec<Entry> = vec![
        Entry {
            id: 1,
            name: "Pikachu".to_string(),
            color: AvailableColors::random() as u8,
            note: "".to_string(),
            rank: 0,
            lost_against: vec![],
        },
        Entry {
            id: 2,
            name: "Pichu".to_string(),
            color: AvailableColors::random() as u8,
            note: "".to_string(),
            rank: 0,
            lost_against: vec![],
        },
        Entry {
            id: 3,
            name: "Mimikyu".to_string(),
            color: AvailableColors::random() as u8,
            note: "".to_string(),
            rank: 0,
            lost_against: vec![],
        },
        Entry {
            id: 4,
            name: "Drampa".to_string(),
            color: AvailableColors::random() as u8,
            note: "".to_string(),
            rank: 0,
            lost_against: vec![],
        },
        Entry {
            id: 5,
            name: "Kyogre".to_string(),
            color: AvailableColors::random() as u8,
            note: "".to_string(),
            rank: 0,
            lost_against: vec![],
        },
        Entry {
            id: 6,
            name: "Hydregon".to_string(),
            color: AvailableColors::random() as u8,
            note: "".to_string(),
            rank: 0,
            lost_against: vec![],
        },
        Entry {
            id: 7,
            name: "Illimuse".to_string(),
            color: AvailableColors::random() as u8,
            note: "".to_string(),
            rank: 0,
            lost_against: vec![],
        },
        Entry {
            id: 8,
            name: "Gardevoir".to_string(),
            color: AvailableColors::random() as u8,
            note: "".to_string(),
            rank: 0,
            lost_against: vec![],
        },
        Entry {
            id: 9,
            name: "Ralts".to_string(),
            color: AvailableColors::random() as u8,
            note: "".to_string(),
            rank: 0,
            lost_against: vec![],
        },
        Entry {
            id: 10,
            name: "Wailord".to_string(),
            color: AvailableColors::random() as u8,
            note: "".to_string(),
            rank: 0,
            lost_against: vec![],
        },
    ];
    db.all_sheets.push(Sheet::new_debug(
        1,
        "Pokemon",
        AvailableColors::random() as u8,
        "note!",
        &mut entry_vec,
    ));
    db.all_sheets.push(Sheet::new(
        db.all_sheets.len(),
        "Books",
        AvailableColors::random() as u8,
        "",
    ));
    db
}
