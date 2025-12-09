use super::persistence::*;
use super::state::*;
use crate::combat::CombatEncounter;
use crate::models::{CombatantTemplate, LogEntry};

pub struct App {
    pub encounter: CombatEncounter,
    pub input_mode: InputMode,
    pub should_quit: bool,
    pub message: Option<String>,
    pub templates: Vec<CombatantTemplate>,
    pub log: Vec<LogEntry>,
}

impl App {
    pub fn new() -> Self {
        let (templates, message) = match load_templates() {
            Ok(t) => (t, None),
            Err(e) => {
                log::error!("Template load error: {}", e);
                (Vec::new(), Some(format!("Warning: {}", e)))
            }
        };

        Self {
            encounter: CombatEncounter::new(),
            input_mode: InputMode::Normal,
            should_quit: false,
            message,
            templates,
            log: Vec::new(),
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

    pub fn open_action_menu(&mut self) {
        self.input_mode = InputMode::ActionMenu(0);
        self.clear_message();
    }

    pub fn open_combatant_menu(&mut self) {
        self.input_mode = InputMode::CombatantMenu(0);
        self.clear_message();
    }

    pub fn start_granting_temp_hp(&mut self) {
        if self.encounter.combatants.is_empty() {
            self.set_message("No combatants to grant temp HP!".to_string());
            return;
        }
        self.input_mode = InputMode::GrantingTempHp(SelectionState::default());
        self.clear_message();
    }

    pub fn start_selecting_template(&mut self) {
        if self.templates.is_empty() {
            self.set_message("No templates available".to_string());
            return;
        }
        self.input_mode = InputMode::SelectingTemplate(SelectionState::default());
        self.clear_message();
    }

    pub fn start_saving_template(&mut self) {
        if self.encounter.combatants.is_empty() {
            self.set_message("No combatants to save as template".to_string());
            return;
        }
        self.input_mode = InputMode::SavingTemplate(SelectionState::default());
        self.clear_message();
    }

    pub fn start_saving_encounter(&mut self) {
        self.input_mode = InputMode::SavingEncounter(SaveEncounterState::default());
    }

    pub fn start_loading_encounter(&mut self) {
        self.input_mode = InputMode::LoadingEncounter(SelectionState::default());
    }

    pub fn start_saving_library(&mut self) {
        if self.encounter.combatants.is_empty() {
            self.set_message("Cannot save empty encounter to library".to_string());
            return;
        }
        self.input_mode = InputMode::SavingLibrary(SaveLibraryState::default());
    }

    pub fn start_loading_library(&mut self) {
        self.input_mode = InputMode::LoadingLibrary(SelectionState::default());
    }

    pub fn cancel_input(&mut self) {
        self.input_mode = InputMode::Normal;
        self.clear_message();
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
