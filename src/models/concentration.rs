use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConcentrationInfo {
    pub spell_name: String,
    pub constitution_modifier: i32,
}

impl ConcentrationInfo {
    pub fn new(spell_name: String, constitution_modifier: i32) -> Self {
        Self {
            spell_name,
            constitution_modifier,
        }
    }
}
