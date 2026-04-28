use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

impl Day {
    pub fn from_abbrev(s: &str) -> Option<Self> {
        match s {
            "Lu" => Some(Self::Monday),
            "Ma" => Some(Self::Tuesday),
            "Mi" => Some(Self::Wednesday),
            "Ju" => Some(Self::Thursday),
            "Vi" => Some(Self::Friday),
            "Sa" => Some(Self::Saturday),
            _ => None,
        }
    }
}

impl fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Day::Monday => write!(f, "Lunes"),
            Day::Tuesday => write!(f, "Martes"),
            Day::Wednesday => write!(f, "Miércoles"),
            Day::Thursday => write!(f, "Jueves"),
            Day::Friday => write!(f, "Viernes"),
            Day::Saturday => write!(f, "Sábado"),
        }
    }
}