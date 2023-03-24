use dialoguer::{theme::ColorfulTheme, theme::SimpleTheme, Input, MultiSelect, Select};

pub fn create_validated_multi_select<T: std::fmt::Display + std::fmt::Debug>(
    choices: &[T],
    msg: &str,
) -> Result<Vec<usize>, String> {
    let selection_i = create_mult_select(choices, msg);
    validate_selection(selection_i, choices.len())
}

pub fn create_mult_select<T: std::fmt::Display + std::fmt::Debug>(
    choices: &[T],
    msg: &str,
) -> Option<Vec<usize>> {
    let selection_i = MultiSelect::with_theme(&SimpleTheme)
        .with_prompt(format!("Pick your {msg} (use space)"))
        .items(&choices)
        .interact_opt()
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
pub fn create_select<T: std::fmt::Display>(choices: &Vec<T>, msg: &str) -> usize {
    let selection_i: usize = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("Pick your {msg} (use space)"))
        .items(&choices)
        .interact()
        .unwrap();

    selection_i
}
