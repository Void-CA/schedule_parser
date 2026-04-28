use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Serialize, Deserialize)]
pub enum AcademicBlock {
    Morning1,
    Morning2,
    Morning3,
    Morning4,
    Afternoon1,
    Afternoon2,
    Afternoon3,
    Afternoon4,
}

impl AcademicBlock {
    pub fn id(&self) -> usize {
        match self {
            Self::Morning1 => 0,
            Self::Morning2 => 1,
            Self::Morning3 => 2,
            Self::Morning4 => 3,
            Self::Afternoon1 => 4,
            Self::Afternoon2 => 5,
            Self::Afternoon3 => 6,
            Self::Afternoon4 => 7,
        }
    }

    pub fn from_id(id: usize) -> Option<Self> {
        match id {
            0 => Some(Self::Morning1),
            1 => Some(Self::Morning2),
            2 => Some(Self::Morning3),
            3 => Some(Self::Morning4),
            4 => Some(Self::Afternoon1),
            5 => Some(Self::Afternoon2),
            6 => Some(Self::Afternoon3),
            7 => Some(Self::Afternoon4),
            _ => None,
        }
    }
}