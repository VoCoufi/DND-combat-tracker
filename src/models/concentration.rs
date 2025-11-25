use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConcentrationInfo {
    pub spell_name: String,
    pub duration_remaining: i32,
    pub constitution_modifier: i32,
}

impl ConcentrationInfo {
    pub fn new(spell_name: String, duration_remaining: i32, constitution_modifier: i32) -> Self {
        Self {
            spell_name,
            duration_remaining,
            constitution_modifier,
        }
    }
}
