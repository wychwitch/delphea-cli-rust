use crate::entries::Entry;
use crate::sheets::Sheet;
use dialoguer::{theme::ColorfulTheme, theme::SimpleTheme, Input, MultiSelect, Select};
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    pub all_entries: Vec<Entry>,
    pub all_sheets: Vec<Sheet>,
}

//
//Implementations
//

impl Database {
    pub fn save(&self) {}

    pub fn pick_sheet_idx(&self) -> usize {
        let sheet_id = menu_creation(&self.all_sheets, "sheet");
        sheet_id
    }
}

///
/// helper
///

pub fn menu_creation<T: std::fmt::Display>(choices: &Vec<T>, msg: &str) -> usize {
    let selection_i: usize = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Pick your {msg} (use space)"))
        .items(&choices)
        .interact()
        .unwrap();

    selection_i
}

pub fn validate_selection(
    selection: Option<Vec<usize>>,
    choices_len: usize,
) -> Result<Vec<usize>, String> {
    match selection {
        Some(selec) => match selec.len() != choices_len && selec.len() != 0 {
            true => Ok(selec),
            false => {
                if selec.len() == 0 {
                    return Err("You must select something".to_owned());
                } else {
                    return Err("You need to leave one option unselected!".to_owned());
                }
            }
        },
        None => Err("Canceled".to_owned()),
    }
}

pub fn mult_menu_creation<T: std::fmt::Display + std::fmt::Debug>(
    choices: &[T],
    msg: &str,
) -> Result<Vec<usize>, String> {
    let selection_i = MultiSelect::with_theme(&SimpleTheme)
        .with_prompt(format!("Pick your {msg} (use space)"))
        .items(&choices)
        .interact_opt()
        .unwrap();
    validate_selection(selection_i, choices.len())
}
