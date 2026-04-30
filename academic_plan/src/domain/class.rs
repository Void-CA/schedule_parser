// Estructura de dominio final
#[derive(Debug, Clone)]
pub struct Class {
    pub code: String,
    pub name: String,
    pub theoretical_hours: u32,
    pub practical_hours: u32,
    pub independent_hours: u32,
    pub total_hours: u32,
    pub credits: u32,
    pub prerequisites: String,
    pub precedents: String,
}