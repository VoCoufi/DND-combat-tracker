use serde::{Deserialize, Serialize};
use crate::models::Combatant;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatEncounter {
    pub combatants: Vec<Combatant>,
    pub current_turn_index: usize,
    pub round_number: u32,
}

impl CombatEncounter {
    pub fn new() -> Self {
        Self {
            combatants: Vec::new(),
            current_turn_index: 0,
            round_number: 1,
        }
    }

    pub fn add_combatant(&mut self, combatant: Combatant) {
        self.combatants.push(combatant);
        self.sort_by_initiative();
        // Reset turn index if this is the first combatant
        if self.combatants.len() == 1 {
            self.current_turn_index = 0;
        }
    }

    pub fn remove_combatant(&mut self, index: usize) {
        if index < self.combatants.len() {
            self.combatants.remove(index);
            // Adjust current turn index if needed
            if self.current_turn_index >= self.combatants.len() && !self.combatants.is_empty() {
                self.current_turn_index = 0;
            }
        }
    }

    pub fn sort_by_initiative(&mut self) {
        self.combatants.sort_by(|a, b| b.initiative.cmp(&a.initiative));
    }

    pub fn next_turn(&mut self) {
        if self.combatants.is_empty() {
            return;
        }

        // Decrement status effects for current combatant
        if let Some(combatant) = self.combatants.get_mut(self.current_turn_index) {
            combatant.decrement_status_effects();
        }

        // Move to next combatant
        self.current_turn_index += 1;

        // If we've gone through all combatants, increment round and reset index
        if self.current_turn_index >= self.combatants.len() {
            self.current_turn_index = 0;
            self.round_number += 1;
        }
    }

    pub fn previous_turn(&mut self) {
        if self.combatants.is_empty() {
            return;
        }

        if self.current_turn_index == 0 {
            // Go to last combatant and decrement round
            self.current_turn_index = self.combatants.len().saturating_sub(1);
            self.round_number = self.round_number.saturating_sub(1).max(1);
        } else {
            self.current_turn_index -= 1;
        }
    }

    pub fn get_current_combatant(&self) -> Option<&Combatant> {
        self.combatants.get(self.current_turn_index)
    }

    pub fn get_current_combatant_mut(&mut self) -> Option<&mut Combatant> {
        self.combatants.get_mut(self.current_turn_index)
    }
}

impl Default for CombatEncounter {
    fn default() -> Self {
        Self::new()
    }
}
