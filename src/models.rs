#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

impl Day {
    pub fn from_abbrev(abbrev: &str) -> Result<Self, String> {
        match abbrev {
            "Lu" => Ok(Self::Monday),
            "Ma" => Ok(Self::Tuesday),
            "Mi" => Ok(Self::Wednesday),
            "Ju" => Ok(Self::Thursday),
            "Vi" => Ok(Self::Friday),
            "Sa" => Ok(Self::Saturday),
            _ => Err(format!("Día no válido: {}", abbrev)),
        }
    }
}

/// Representa la unidad mínima de clase: 50 minutos.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum AcademicBlock {
    /// 08:00 am - 08:50 am
    Morning1,
    /// 08:50 am - 09:40 am
    Morning2,
    /// 10:00 am - 10:50 am
    Morning3,
    /// 10:50 am - 11:40 am
    Morning4,
    /// 01:00 pm - 01:50 pm
    Afternoon1,
    /// 01:50 pm - 02:40 pm
    Afternoon2,
    /// 03:00 pm - 03:50 pm
    Afternoon3,
    /// 03:50 pm - 04:40 pm
    Afternoon4,
}

#[derive(Debug, Clone)]
pub struct Encounter {
    pub major: Major,
    pub group: u8,
    pub subject: String,
    pub professor: String,
    pub room: String,
    pub day: Day,
    pub blocks: Vec<AcademicBlock>,
}


impl AcademicBlock {
    /// Traduce las horas crudas del SIGA a bloques académicos.
    pub fn from_time_range(start: &str, end: &str) -> Result<Vec<AcademicBlock>, String> {
        match (start, end) {
            ("08:00 am", "09:40 am") => Ok(vec![Self::Morning1, Self::Morning2]),
            ("08:50 am", "09:40 am") => Ok(vec![Self::Morning2]),
            ("10:00 am", "11:40 am") => Ok(vec![Self::Morning3, Self::Morning4]),
            ("01:00 pm", "02:40 pm") => Ok(vec![Self::Afternoon1, Self::Afternoon2]),
            ("03:00 pm", "04:40 pm") => Ok(vec![Self::Afternoon3, Self::Afternoon4]),
            ("03:00 pm", "03:50 pm") => Ok(vec![Self::Afternoon3]),
            _ => Err(format!("Horario no reconocido o irregular: {} - {}", start, end)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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
    pub fn from_abbrev(abbrev: &str) -> Result<Self, String> {
        match abbrev {
            "IGI" => Ok(Self::IGI),
            "IMS" => Ok(Self::IMS),
            "IME" => Ok(Self::IME),
            "ICE" => Ok(Self::ICE),
            "IEE" => Ok(Self::IEE),
            "IEM" => Ok(Self::IEM),
            "LAF" => Ok(Self::LAF),
            "LCM" => Ok(Self::LCM),
            _ => Err(format!("Carrera no reconocida: {}", abbrev)),
        }
    }

    /// Retorna el nombre completo de la carrera para la interfaz de usuario
    pub fn full_name(&self) -> &'static str {
        match self {
            Self::IGI => "Ingeniería en Gestión Industrial",
            Self::IMS => "Ingeniería Mecatrónica y en Sistemas de Control",
            Self::IME => "Ingeniería Mecánica con énfasis en Energías Renovables",
            Self::ICE => "Ingeniería Cibernética Electrónica",
            Self::IEE => "Ingeniería Eléctrica con Eficiencia Energética",
            Self::IEM => "Ingeniería Electromédica",
            Self::LAF => "Licenciatura en Administración con énfasis en Finanzas",
            Self::LCM => "Licenciatura en Comercio y Mercadeo",
        }
    }
}