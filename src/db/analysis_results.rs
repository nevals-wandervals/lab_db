use crate::db::{Id, method::TypeUnit};

#[derive(Debug, Clone)]
pub struct AnalysisResult {
    method: Id,
    value: TypeUnit,
}

impl AnalysisResult {
    pub fn new(value: TypeUnit, method: Id) -> Self {
        Self { method, value }
    }

    pub fn get_value(&self) -> &TypeUnit {
        &self.value
    }

    pub fn get_method(&self) -> Id {
        self.method
    }

    pub fn set_value(&mut self, new_value: TypeUnit) {
        self.value = new_value;
    }
}
