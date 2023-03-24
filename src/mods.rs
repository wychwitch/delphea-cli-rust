use console::Style;
use dialoguer::{theme::ColorfulTheme, theme::SimpleTheme, Input, MultiSelect, Select, Validator};
use enum_iterator::{all, Sequence};
use rand::{seq::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};
use std::{
    borrow::BorrowMut,
    fmt::{self, Display},
};
