struct Thing {
    id: i32,
    sheet_id: i32,
    name: String,
    color: String,
    eliminated_by: i32,
    note: String,
}

struct Sheet {
    id: i32,
    name: String,
    color: String,
    note: String,
}

impl Thing {
    pub fn get_sheet(&self, sheets: Vec<Sheet>) -> Sheet {
        let sheet_id = &self.sheet_id;
        sheets.get(sheet_id).expect("valid sheet id").to_owned()
    }
}

fn main() {
    let mut Sheets: Vec<Sheet>;
}
