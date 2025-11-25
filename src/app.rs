use crate::combat::CombatEncounter;
use crate::models::{Combatant, ConcentrationInfo, ConditionType, DeathSaveOutcome, StatusEffect};

#[derive(Debug, Clone, PartialEq)]
pub enum InputMode {
    Normal,
    AddingCombatant(AddCombatantState),
    DealingDamage(SelectionState),
    Healing(SelectionState),
    AddingStatus(SelectionState),
    SelectingCondition(ConditionSelectionState),
    RollingDeathSave(SelectionState),
    ConcentrationTarget(SelectionState),
    ApplyingConcentration(AddConcentrationState),
    ConcentrationCheck(ConcentrationCheckState),
    ClearingConcentration(SelectionState),
    ClearActionSelection(ClearAction),
    ClearingStatus(SelectionState),
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

#[derive(Debug, Clone, PartialEq)]
pub struct AddConcentrationState {
    pub combatant_index: usize,
    pub step: usize, // 0: spell name, 1: con mod
    pub spell_name: String,
    pub con_mod: String,
}

impl Default for AddConcentrationState {
    fn default() -> Self {
        Self {
            combatant_index: 0,
            step: 0,
            spell_name: String::new(),
            con_mod: String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConcentrationCheckState {
    pub combatant_index: usize,
    pub dc: i32,
    pub input: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClearAction {
    Concentration,
    StatusEffects,
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

    pub fn start_concentration_target(&mut self) {
        if self.encounter.combatants.is_empty() {
            self.set_message("No combatants to set concentration on!".to_string());
            return;
        }
        self.input_mode = InputMode::ConcentrationTarget(SelectionState::default());
        self.clear_message();
    }

    pub fn start_clear_choice(&mut self) {
        self.input_mode = InputMode::ClearActionSelection(ClearAction::Concentration);
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
        let had_concentration = combatant.concentration.clone();
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
        if combatant.is_unconscious() {
            combatant.clear_concentration();
        } else if let Some(info) = had_concentration {
            let dc = std::cmp::max(10, damage / 2);
            self.input_mode = InputMode::ConcentrationCheck(ConcentrationCheckState {
                combatant_index: index,
                dc,
                input: String::new(),
            });
            self.set_message(format!(
                "{} took damage while concentrating on {}. Roll CON save (DC {}).",
                name, info.spell_name, dc
            ));
            return Ok(());
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
            // healing to positive HP keeps concentration as-is
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

    pub fn complete_apply_concentration(
        &mut self,
        state: AddConcentrationState,
    ) -> Result<(), String> {
        if state.combatant_index >= self.encounter.combatants.len() {
            return Err("Invalid combatant index".to_string());
        }

        let con_mod = state
            .con_mod
            .parse::<i32>()
            .map_err(|_| "Invalid constitution modifier".to_string())?;

        if state.spell_name.trim().is_empty() {
            return Err("Spell name cannot be empty".to_string());
        }

        let info = ConcentrationInfo::new(state.spell_name.clone(), con_mod);
        let combatant = &mut self.encounter.combatants[state.combatant_index];
        let name = combatant.name.clone();
        combatant.set_concentration(info);
        self.input_mode = InputMode::Normal;
        self.set_message(format!(
            "{} starts concentrating on {}.",
            name, state.spell_name
        ));
        Ok(())
    }

    pub fn complete_concentration_check(
        &mut self,
        state: ConcentrationCheckState,
        roll_total: i32,
    ) -> Result<(), String> {
        if state.combatant_index >= self.encounter.combatants.len() {
            return Err("Invalid combatant index".to_string());
        }

        let combatant = &mut self.encounter.combatants[state.combatant_index];
        let Some(info) = combatant.concentration.clone() else {
            return Err("Combatant is not concentrating".to_string());
        };

        let name = combatant.name.clone();
        self.input_mode = InputMode::Normal;

        if roll_total >= state.dc {
            self.set_message(format!(
                "{} maintains concentration on {} (roll {} vs DC {}).",
                name, info.spell_name, roll_total, state.dc
            ));
        } else {
            combatant.clear_concentration();
            self.set_message(format!(
                "{} fails concentration on {} (roll {} vs DC {}).",
                name, info.spell_name, roll_total, state.dc
            ));
        }

        Ok(())
    }

    pub fn complete_clear_concentration(&mut self, index: usize) -> Result<(), String> {
        if index >= self.encounter.combatants.len() {
            return Err("Invalid combatant index".to_string());
        }

        let combatant = &mut self.encounter.combatants[index];
        let name = combatant.name.clone();
        if combatant.concentration.is_some() {
            combatant.clear_concentration();
            self.set_message(format!("{} stops concentrating.", name));
        } else {
            self.set_message(format!("{} has no concentration to clear.", name));
        }
        self.input_mode = InputMode::Normal;
        Ok(())
    }

    pub fn complete_clear_status_effects(&mut self, index: usize) -> Result<(), String> {
        if index >= self.encounter.combatants.len() {
            return Err("Invalid combatant index".to_string());
        }

        let combatant = &mut self.encounter.combatants[index];
        let name = combatant.name.clone();
        if combatant.status_effects.is_empty() {
            self.set_message(format!("{} has no status effects to clear.", name));
        } else {
            combatant.status_effects.clear();
            self.set_message(format!("Cleared all status effects from {}.", name));
        }
        self.input_mode = InputMode::Normal;
        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
