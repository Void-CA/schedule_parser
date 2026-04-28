use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Major {
    IGI,
    IMS,
    IME,
    ICE,
    IEE,
    IEM,
    LAF,
    LCM,
}

impl Major {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "IGI" => Some(Self::IGI),
            "IMS" => Some(Self::IMS),
            "IME" => Some(Self::IME),
            "ICE" => Some(Self::ICE),
            "IEE" => Some(Self::IEE),
            "IEM" => Some(Self::IEM),
            "LAF" => Some(Self::LAF),
            "LCM" => Some(Self::LCM),
            _ => None,
        }
    }
}

impl fmt::Display for Major {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Major::IGI => write!(f, "Ingeniería en Gestion Industrial"),
            Major::IMS => write!(f, "Ingeniería en Mecatronica y Sistemas de Control"),
            Major::IME => write!(f, "Ingeniería en Mecanica con Enfasis en Energias Renovables"),
            Major::ICE => write!(f, "Ingeniería en Cibernetica Electronica"),
            Major::IEE => write!(f, "Ingeniería en Eléctrica con Enfasis en Eficiencia Energetica"),
            Major::IEM => write!(f, "Ingeniería Electromedica"),
            Major::LAF => write!(f, "Licenciatura en Administración con Enfasis en Finanzas"),
            Major::LCM => write!(f, "Licenciatura en Comercio y Mercadeo"),
        }
    }
}