use super::status::StatusEffect;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Combatant {
    pub name: String,
    pub initiative: i32,
    pub hp_current: i32,
    pub hp_max: i32,
    pub armor_class: i32,
    pub is_player: bool,
    pub status_effects: Vec<StatusEffect>,
}

impl Combatant {
    pub fn new(
        name: String,
        initiative: i32,
        hp_max: i32,
        armor_class: i32,
        is_player: bool,
    ) -> Self {
        Self {
            name,
            initiative,
            hp_current: hp_max,
            hp_max,
            armor_class,
            is_player,
            status_effects: Vec::new(),
        }
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.hp_current = (self.hp_current - damage).max(0);
    }

    pub fn heal(&mut self, amount: i32) {
        self.hp_current = (self.hp_current + amount).min(self.hp_max);
    }

    pub fn add_status_effect(&mut self, effect: StatusEffect) {
        self.status_effects.push(effect);
    }

    pub fn remove_status_effect(&mut self, index: usize) {
        if index < self.status_effects.len() {
            self.status_effects.remove(index);
        }
    }

    pub fn decrement_status_effects(&mut self) {
        for effect in &mut self.status_effects {
            effect.decrement_duration();
        }
        self.status_effects.retain(|effect| !effect.is_expired());
    }

    pub fn is_unconscious(&self) -> bool {
        self.hp_current <= 0
    }

    pub fn hp_percentage(&self) -> f32 {
        if self.hp_max == 0 {
            0.0
        } else {
            (self.hp_current as f32 / self.hp_max as f32) * 100.0
        }
    }
}
