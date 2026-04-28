#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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