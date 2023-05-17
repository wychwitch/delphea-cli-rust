use console::Style;
use enum_iterator::Sequence;
use rand_derive2::RandGen;
use std::fmt::{self, Display};
#[derive(Debug, PartialEq, Sequence, Clone, RandGen)]
pub enum AvailableColors {
    BlackSystem = 0,
    MaroonSystem = 1,
    GreenSystem = 2,
    OliveSystem = 3,
    NavySystem = 4,
    PurpleSystem = 5,
    TealSystem = 6,
    SilverSystem = 7,
    GreySystem = 8,
    RedSystem = 9,
    LimeSystem = 10,
    YellowSystem = 11,
    BlueSystem = 12,
    FuchsiaSystem = 13,
    AquaSystem = 14,
    Pink = 218,
    Orange = 173,
    Ruddy = 167,
    Brown = 138,
    Magenta = 132,
    Sky = 105,
    Storm = 103,
    Purple = 97,
    NeonViolet = 91,
    Ruby = 89,
}

impl Display for AvailableColors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AvailableColors::Pink => {
                let style = Style::new().color256(AvailableColors::Pink as u8);
                write!(f, "{}", style.apply_to("Pink"))
            }

            AvailableColors::Orange => {
                let style = Style::new().color256(AvailableColors::Orange as u8);
                write!(f, "{}", style.apply_to("Orange"))
            }

            AvailableColors::Ruddy => {
                let style = Style::new().color256(AvailableColors::Ruddy as u8);
                write!(f, "{}", style.apply_to("Ruddy"))
            }

            AvailableColors::Brown => {
                let style = Style::new().color256(AvailableColors::Brown as u8);
                write!(f, "{}", style.apply_to("Brown"))
            }

            AvailableColors::Magenta => {
                let style = Style::new().color256(AvailableColors::Magenta as u8);
                write!(f, "{}", style.apply_to("Magenta"))
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

            AvailableColors::NeonViolet => {
                let style = Style::new().color256(AvailableColors::NeonViolet as u8);
                write!(f, "{}", style.apply_to("NeonViolet"))
            }

            AvailableColors::Ruby => {
                let style = Style::new().color256(AvailableColors::Ruby as u8);
                write!(f, "{}", style.apply_to("Ruby"))
            }

            AvailableColors::BlackSystem => {
                let style = Style::new().color256(AvailableColors::BlackSystem as u8);
                write!(f, "{}", style.apply_to("Black (System)"))
            }

            AvailableColors::MaroonSystem => {
                let style = Style::new().color256(AvailableColors::MaroonSystem as u8);
                write!(f, "{}", style.apply_to("Maroon (System)"))
            }

            AvailableColors::GreenSystem => {
                let style = Style::new().color256(AvailableColors::GreenSystem as u8);
                write!(f, "{}", style.apply_to("Green (System)"))
            }

            AvailableColors::OliveSystem => {
                let style = Style::new().color256(AvailableColors::OliveSystem as u8);
                write!(f, "{}", style.apply_to("Olive (System)"))
            }

            AvailableColors::NavySystem => {
                let style = Style::new().color256(AvailableColors::NavySystem as u8);
                write!(f, "{}", style.apply_to("Navy (System)"))
            }

            AvailableColors::PurpleSystem => {
                let style = Style::new().color256(AvailableColors::PurpleSystem as u8);
                write!(f, "{}", style.apply_to("Purple (System)"))
            }

            AvailableColors::TealSystem => {
                let style = Style::new().color256(AvailableColors::TealSystem as u8);
                write!(f, "{}", style.apply_to("Teal (System)"))
            }

            AvailableColors::SilverSystem => {
                let style = Style::new().color256(AvailableColors::SilverSystem as u8);
                write!(f, "{}", style.apply_to("Silver (System)"))
            }

            AvailableColors::GreySystem => {
                let style = Style::new().color256(AvailableColors::GreySystem as u8);
                write!(f, "{}", style.apply_to("Grey (System)"))
            }

            AvailableColors::RedSystem => {
                let style = Style::new().color256(AvailableColors::RedSystem as u8);
                write!(f, "{}", style.apply_to("Red (System)"))
            }

            AvailableColors::LimeSystem => {
                let style = Style::new().color256(AvailableColors::LimeSystem as u8);
                write!(f, "{}", style.apply_to("Lime (System)"))
            }

            AvailableColors::YellowSystem => {
                let style = Style::new().color256(AvailableColors::YellowSystem as u8);
                write!(f, "{}", style.apply_to("Yellow (System)"))
            }

            AvailableColors::BlueSystem => {
                let style = Style::new().color256(AvailableColors::BlueSystem as u8);
                write!(f, "{}", style.apply_to("Blue (System)"))
            }
            AvailableColors::FuchsiaSystem => {
                let style = Style::new().color256(AvailableColors::FuchsiaSystem as u8);
                write!(f, "{}", style.apply_to("Fuchsia (System)"))
            }

            AvailableColors::AquaSystem => {
                let style = Style::new().color256(AvailableColors::AquaSystem as u8);
                write!(f, "{}", style.apply_to("Aqua (System)"))
            }
        }
    }
}
impl AvailableColors {
    pub fn random() -> AvailableColors {
        rand::random()
    }
}
