use crate::db::Id;

#[derive(Debug, Clone)]
pub struct Method {
    pub id: Id,
    pub title: String,
    // unit of measurement
    pub unit: String,
    pub min_reference_values: TypeUnit,
    pub max_reference_values: TypeUnit,
}

#[derive(Debug, Clone)]
pub enum TypeUnit {
    String(String),
    Number(f32),
    Boolean(bool),
}
