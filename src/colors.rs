use console::Style;
use enum_iterator::Sequence;
use rand_derive2::RandGen;
use std::fmt::{self, Display};
#[derive(Debug, PartialEq, Sequence, Clone, RandGen)]
pub enum AvailableColors {
    Pink = 218,
    Yellow = 222,
    Lavender = 182,
    Orange = 173,
    Ruddy = 167,
    Bluish = 146,
    Brown = 138,
    Magenta = 132,
    Green = 108,
    Sky = 105,
    Storm = 103,
    Purple = 97,
    Plum = 96,
    NeonViolet = 91,
    Ruby = 89,
    Red = 196,
}

impl Display for AvailableColors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AvailableColors::Pink => {
                let style = Style::new().color256(AvailableColors::Pink as u8);
                write!(f, "{}", style.apply_to("Pink"))
            }
            AvailableColors::Yellow => {
                let style = Style::new().color256(AvailableColors::Yellow as u8);
                write!(f, "{}", style.apply_to("Yellow"))
            }

            AvailableColors::Lavender => {
                let style = Style::new().color256(AvailableColors::Lavender as u8);
                write!(f, "{}", style.apply_to("Lavender"))
            }

            AvailableColors::Orange => {
                let style = Style::new().color256(AvailableColors::Orange as u8);
                write!(f, "{}", style.apply_to("Orange"))
            }

            AvailableColors::Ruddy => {
                let style = Style::new().color256(AvailableColors::Ruddy as u8);
                write!(f, "{}", style.apply_to("Ruddy"))
            }

            AvailableColors::Bluish => {
                let style = Style::new().color256(AvailableColors::Bluish as u8);
                write!(f, "{}", style.apply_to("Bluish"))
            }

            AvailableColors::Brown => {
                let style = Style::new().color256(AvailableColors::Brown as u8);
                write!(f, "{}", style.apply_to("Brown"))
            }

            AvailableColors::Magenta => {
                let style = Style::new().color256(AvailableColors::Magenta as u8);
                write!(f, "{}", style.apply_to("Magenta"))
            }

            AvailableColors::Green => {
                let style = Style::new().color256(AvailableColors::Green as u8);
                write!(f, "{}", style.apply_to("Green"))
            }

            AvailableColors::Sky => {
                let style = Style::new().color256(AvailableColors::Sky as u8);
                write!(f, "{}", style.apply_to("Sky"))
            }

            AvailableColors::Storm => {
                let style = Style::new().color256(AvailableColors::Storm as u8);
                write!(f, "{}", style.apply_to("Storm"))
            }

            AvailableColors::Purple => {
                let style = Style::new().color256(AvailableColors::Purple as u8);
                write!(f, "{}", style.apply_to("Purple"))
            }

            AvailableColors::Plum => {
                let style = Style::new().color256(AvailableColors::Plum as u8);
                write!(f, "{}", style.apply_to("Plum"))
            }

            AvailableColors::NeonViolet => {
                let style = Style::new().color256(AvailableColors::NeonViolet as u8);
                write!(f, "{}", style.apply_to("NeonViolet"))
            }

            AvailableColors::Ruby => {
                let style = Style::new().color256(AvailableColors::Ruby as u8);
                write!(f, "{}", style.apply_to("Ruby"))
            }

            AvailableColors::Red => {
                let style = Style::new().color256(AvailableColors::Red as u8);
                write!(f, "{}", style.apply_to("Red"))
            }
        }
    }
}
impl AvailableColors {
    pub fn random() -> AvailableColors {
        rand::random()
    }
}
