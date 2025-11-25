use super::{ConcentrationInfo, DeathSaveOutcome, DeathSaves, status::StatusEffect};
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
    pub death_saves: Option<DeathSaves>,
    pub concentration: Option<ConcentrationInfo>,
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
            death_saves: None,
            concentration: None,
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

    pub fn is_dead(&self) -> bool {
        self.death_saves
            .as_ref()
            .map(|d| d.failures >= 3)
            .unwrap_or(false)
    }

    pub fn hp_percentage(&self) -> f32 {
        if self.hp_max == 0 {
            0.0
        } else {
            (self.hp_current as f32 / self.hp_max as f32) * 100.0
        }
    }

    pub fn ensure_death_saves(&mut self) {
        if self.is_player && self.death_saves.is_none() {
            self.death_saves = Some(DeathSaves::default());
        }
    }

    pub fn clear_death_saves(&mut self) {
        self.death_saves = None;
    }

    pub fn apply_death_save_roll(&mut self, roll: i32) -> DeathSaveOutcome {
        if !self.is_player || self.hp_current > 0 {
            return DeathSaveOutcome::Ongoing;
        }

        self.ensure_death_saves();

        if roll == 20 {
            self.hp_current = 1;
            self.death_saves = None;
            return DeathSaveOutcome::Revived;
        }

        if let Some(saves) = &mut self.death_saves {
            if roll == 1 {
                saves.add_failure(2)
            } else if roll >= 10 {
                saves.add_success()
            } else {
                saves.add_failure(1)
            }
        } else {
            DeathSaveOutcome::Ongoing
        }
    }

    pub fn fail_death_save_from_damage(&mut self) -> DeathSaveOutcome {
        if !self.is_player {
            return DeathSaveOutcome::Ongoing;
        }

        self.ensure_death_saves();
        if let Some(saves) = &mut self.death_saves {
            saves.add_failure(1)
        } else {
            DeathSaveOutcome::Ongoing
        }
    }

    pub fn clear_concentration(&mut self) {
        self.concentration = None;
    }

    pub fn set_concentration(&mut self, info: ConcentrationInfo) {
        self.concentration = Some(info);
    }
}
