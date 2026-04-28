#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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