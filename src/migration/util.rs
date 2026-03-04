use inquire::Select;
use std::fmt;

pub enum PromtYesNoRemember {
    Yes,
    No,
    NoRemember,
}

impl fmt::Display for PromtYesNoRemember {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Yes => f.write_str("Yes"),
            Self::No => f.write_str("No"),
            Self::NoRemember => f.write_str("No (don't ask again)"),
        }
    }
}

impl PromtYesNoRemember {
    pub fn promt(message: &str) -> inquire::Select<'_, Self> {
        let options = vec![Self::Yes, Self::No, Self::NoRemember];
        Select::new(message, options)
    }
}
