use crate::combat::CombatEncounter;
use crate::models::{Combatant, ConditionType, DeathSaveOutcome, StatusEffect};

#[derive(Debug, Clone, PartialEq)]
pub enum InputMode {
    Normal,
    AddingCombatant(AddCombatantState),
    DealingDamage(SelectionState),
    Healing(SelectionState),
    AddingStatus(SelectionState),
    SelectingCondition(ConditionSelectionState),
    RollingDeathSave(SelectionState),
    Removing(SelectionState),
}

#[derive(Debug, Clone, PartialEq)]
pub struct AddCombatantState {
    pub step: usize, // 0: name, 1: initiative, 2: hp, 3: ac, 4: is_player
    pub name: String,
    pub initiative: String,
    pub hp: String,
    pub ac: String,
    pub is_player: String,
}

impl Default for AddCombatantState {
    fn default() -> Self {
        Self {
            step: 0,
            name: String::new(),
            initiative: String::new(),
            hp: String::new(),
            ac: String::new(),
            is_player: String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelectionState {
    pub selected_index: usize,
    pub input: String,
}

impl Default for SelectionState {
    fn default() -> Self {
        Self {
            selected_index: 0,
            input: String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConditionSelectionState {
    pub combatant_index: usize,
    pub input: String,
}

pub struct App {
    pub encounter: CombatEncounter,
    pub input_mode: InputMode,
    pub should_quit: bool,
    pub message: Option<String>,
}

impl App {
    pub fn new() -> Self {
        Self {
            encounter: CombatEncounter::new(),
            input_mode: InputMode::Normal,
            should_quit: false,
            message: None,
        }
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn set_message(&mut self, msg: String) {
        self.message = Some(msg);
    }

    pub fn clear_message(&mut self) {
        self.message = None;
    }

    pub fn start_adding_combatant(&mut self) {
        self.input_mode = InputMode::AddingCombatant(AddCombatantState::default());
        self.clear_message();
    }

    pub fn start_dealing_damage(&mut self) {
        if self.encounter.combatants.is_empty() {
            self.set_message("No combatants to damage!".to_string());
            return;
        }
        self.input_mode = InputMode::DealingDamage(SelectionState::default());
        self.clear_message();
    }

    pub fn start_healing(&mut self) {
        if self.encounter.combatants.is_empty() {
            self.set_message("No combatants to heal!".to_string());
            return;
        }
        self.input_mode = InputMode::Healing(SelectionState::default());
        self.clear_message();
    }

    pub fn start_adding_status(&mut self) {
        if self.encounter.combatants.is_empty() {
            self.set_message("No combatants to add status to!".to_string());
            return;
        }
        self.input_mode = InputMode::AddingStatus(SelectionState::default());
        self.clear_message();
    }

    pub fn start_removing(&mut self) {
        if self.encounter.combatants.is_empty() {
            self.set_message("No combatants to remove!".to_string());
            return;
        }
        self.input_mode = InputMode::Removing(SelectionState::default());
        self.clear_message();
    }

    pub fn start_rolling_death_save(&mut self) {
        if self
            .encounter
            .combatants
            .iter()
            .all(|c| c.death_saves.is_none() || c.hp_current > 0)
        {
            self.set_message("No combatants need death saves!".to_string());
            return;
        }
        self.input_mode = InputMode::RollingDeathSave(SelectionState::default());
        self.clear_message();
    }

    pub fn cancel_input(&mut self) {
        self.input_mode = InputMode::Normal;
        self.clear_message();
    }

    pub fn complete_add_combatant(&mut self, state: AddCombatantState) -> Result<(), String> {
        let initiative = state
            .initiative
            .parse::<i32>()
            .map_err(|_| "Invalid initiative value")?;
        let hp = state.hp.parse::<i32>().map_err(|_| "Invalid HP value")?;
        let ac = state.ac.parse::<i32>().map_err(|_| "Invalid AC value")?;
        let is_player =
            state.is_player.to_lowercase() == "y" || state.is_player.to_lowercase() == "yes";

        if state.name.is_empty() {
            return Err("Name cannot be empty".to_string());
        }

        let combatant = Combatant::new(state.name.clone(), initiative, hp, ac, is_player);
        self.encounter.add_combatant(combatant);
        self.input_mode = InputMode::Normal;
        self.set_message(format!("Added combatant: {}", state.name));
        Ok(())
    }

    pub fn complete_deal_damage(&mut self, index: usize, damage: i32) -> Result<(), String> {
        if index >= self.encounter.combatants.len() {
            return Err("Invalid combatant index".to_string());
        }

        let combatant = &mut self.encounter.combatants[index];
        let was_unconscious = combatant.is_unconscious();
        combatant.take_damage(damage);
        let name = combatant.name.clone();
        let hp = combatant.hp_current;
        let mut extra_message: Option<String> = None;

        if combatant.is_player {
            if !was_unconscious && combatant.is_unconscious() {
                combatant.ensure_death_saves();
                extra_message = Some(format!("{} is down and starts making death saves.", name));
            } else if was_unconscious && combatant.is_unconscious() {
                match combatant.fail_death_save_from_damage() {
                    DeathSaveOutcome::Died => {
                        extra_message = Some(format!("{} takes damage at 0 HP and dies.", name));
                    }
                    DeathSaveOutcome::Ongoing => {
                        if let Some(ds) = &combatant.death_saves {
                            extra_message = Some(format!(
                                "{} takes damage at 0 HP (Death Saves F{}/S{})",
                                name, ds.failures, ds.successes
                            ));
                        }
                    }
                    _ => {}
                }
            }
        }

        self.input_mode = InputMode::Normal;
        let base = format!("{} took {} damage (HP: {})", name, damage, hp);
        if let Some(extra) = extra_message {
            self.set_message(format!("{} | {}", base, extra));
        } else {
            self.set_message(base);
        }
        Ok(())
    }

    pub fn complete_heal(&mut self, index: usize, amount: i32) -> Result<(), String> {
        if index >= self.encounter.combatants.len() {
            return Err("Invalid combatant index".to_string());
        }

        let combatant = &mut self.encounter.combatants[index];
        combatant.heal(amount);
        let name = combatant.name.clone();
        let hp = combatant.hp_current;

        if combatant.hp_current > 0 {
            combatant.clear_death_saves();
        }

        self.input_mode = InputMode::Normal;
        self.set_message(format!("{} healed {} HP (HP: {})", name, amount, hp));
        Ok(())
    }

    pub fn complete_add_status(
        &mut self,
        combatant_index: usize,
        condition: ConditionType,
        duration: i32,
    ) -> Result<(), String> {
        if combatant_index >= self.encounter.combatants.len() {
            return Err("Invalid combatant index".to_string());
        }

        let effect = StatusEffect::new(condition, duration, None);
        let combatant = &mut self.encounter.combatants[combatant_index];
        combatant.add_status_effect(effect);
        let name = combatant.name.clone();

        self.input_mode = InputMode::Normal;
        self.set_message(format!(
            "Added {} to {} for {} rounds",
            condition.as_str(),
            name,
            duration
        ));
        Ok(())
    }

    pub fn complete_remove(&mut self, index: usize) -> Result<(), String> {
        if index >= self.encounter.combatants.len() {
            return Err("Invalid combatant index".to_string());
        }

        let name = self.encounter.combatants[index].name.clone();
        self.encounter.remove_combatant(index);

        self.input_mode = InputMode::Normal;
        self.set_message(format!("Removed combatant: {}", name));
        Ok(())
    }

    pub fn complete_death_save_roll(&mut self, index: usize, roll: i32) -> Result<(), String> {
        if index >= self.encounter.combatants.len() {
            return Err("Invalid combatant index".to_string());
        }

        let combatant = &mut self.encounter.combatants[index];
        if !combatant.is_player {
            return Err("Only player characters roll death saves".to_string());
        }
        if combatant.hp_current > 0 {
            return Err("Combatant is not at 0 HP".to_string());
        }
        if combatant.is_dead() {
            return Err("Combatant is already dead".to_string());
        }

        let name = combatant.name.clone();
        let outcome = combatant.apply_death_save_roll(roll);
        self.input_mode = InputMode::Normal;

        let message = match outcome {
            DeathSaveOutcome::Revived => {
                format!("{} rolled a 20 and regains consciousness at 1 HP!", name)
            }
            DeathSaveOutcome::Stabilized => format!(
                "{} succeeds the death save and is now stable (S{}/F{})",
                name,
                combatant
                    .death_saves
                    .as_ref()
                    .map(|d| d.successes)
                    .unwrap_or(3),
                combatant
                    .death_saves
                    .as_ref()
                    .map(|d| d.failures)
                    .unwrap_or(0)
            ),
            DeathSaveOutcome::Died => format!("{} failed too many death saves and has died.", name),
            DeathSaveOutcome::Ongoing => {
                if let Some(ds) = &combatant.death_saves {
                    format!(
                        "{} death save result recorded (S{}/F{})",
                        name, ds.successes, ds.failures
                    )
                } else {
                    format!("{} death save recorded.", name)
                }
            }
        };

        self.set_message(message);
        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
