use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatantTemplate {
    pub name: String,
    pub hp_max: i32,
    pub armor_class: i32,
    pub initiative: i32,
    pub is_player: bool,
}

impl CombatantTemplate {
    pub fn from_stats(
        name: String,
        initiative: i32,
        hp_max: i32,
        armor_class: i32,
        is_player: bool,
    ) -> Self {
        Self {
            name,
            hp_max,
            armor_class,
            initiative,
            is_player,
        }
    }
}
