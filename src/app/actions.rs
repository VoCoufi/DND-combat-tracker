use super::core::App;
use super::persistence::*;
use super::state::*;
use crate::models::{
    Combatant, CombatantTemplate, ConcentrationInfo, ConditionType, DeathSaveOutcome, LogEntry,
    StatusEffect,
};
use std::time::{SystemTime, UNIX_EPOCH};

impl App {
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
        self.push_log(format!(
            "Added {} (HP {}, AC {}, Init {}, {})",
            state.name,
            hp,
            ac,
            initiative,
            if is_player { "PC" } else { "NPC" }
        ));
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
            self.set_message(base.clone());
        }
        self.push_log(base);
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
        let msg = format!("{} healed {} HP (HP: {})", name, amount, hp);
        self.set_message(msg.clone());
        self.push_log(msg);
        Ok(())
    }

    pub fn complete_grant_temp_hp(&mut self, index: usize, amount: i32) -> Result<(), String> {
        if index >= self.encounter.combatants.len() {
            return Err("Invalid combatant index".to_string());
        }
        if amount < 0 {
            return Err("Temp HP must be non-negative".to_string());
        }
        let combatant = &mut self.encounter.combatants[index];
        let name = combatant.name.clone();
        combatant.grant_temp_hp(amount);
        self.input_mode = InputMode::Normal;
        let msg = format!("{} gains {} temp HP", name, amount);
        self.set_message(msg.clone());
        self.push_log(msg);
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
        self.push_log(format!(
            "{} gains {} for {}",
            name,
            condition.as_str(),
            if duration >= 0 {
                format!("{} rounds", duration)
            } else {
                "indefinite".to_string()
            }
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
        self.push_log(format!("Removed combatant: {}", name));
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
                let msg = format!("{} rolled a 20 and regains consciousness at 1 HP!", name);
                self.push_log(msg.clone());
                msg
            }
            DeathSaveOutcome::Stabilized => {
                let msg = format!(
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
                );
                self.push_log(msg.clone());
                msg
            }
            DeathSaveOutcome::Died => {
                let msg = format!("{} failed too many death saves and has died.", name);
                self.push_log(msg.clone());
                msg
            }
            DeathSaveOutcome::Ongoing => {
                let msg = if let Some(ds) = &combatant.death_saves {
                    format!(
                        "{} death save result recorded (S{}/F{})",
                        name, ds.successes, ds.failures
                    )
                } else {
                    format!("{} death save recorded.", name)
                };
                self.push_log(msg.clone());
                msg
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
        self.push_log(format!(
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

    pub fn complete_clear_status_effect(
        &mut self,
        combatant_index: usize,
        status_index: Option<usize>,
    ) -> Result<(), String> {
        if combatant_index >= self.encounter.combatants.len() {
            return Err("Invalid combatant index".to_string());
        }

        let combatant = &mut self.encounter.combatants[combatant_index];
        let name = combatant.name.clone();
        match status_index {
            Some(idx) => {
                if idx < combatant.status_effects.len() {
                    let removed = combatant.status_effects.remove(idx);
                    self.set_message(format!(
                        "Removed {} from {}.",
                        removed.condition.as_str(),
                        name
                    ));
                } else {
                    return Err("Invalid status selection".to_string());
                }
            }
            None => {
                if combatant.status_effects.is_empty() {
                    self.set_message(format!("{} has no status effects to clear.", name));
                } else {
                    combatant.status_effects.clear();
                    self.set_message(format!("Cleared all status effects from {}.", name));
                }
            }
        };
        self.input_mode = InputMode::Normal;
        Ok(())
    }

    pub fn add_combatant_from_template(&mut self, template_index: usize) -> Result<(), String> {
        if template_index >= self.templates.len() {
            return Err("Invalid template selection".to_string());
        }
        let tpl = self.templates[template_index].clone();
        let mut state = AddCombatantState::default();
        state.name = tpl.name.clone();
        state.hp = tpl.hp_max.to_string();
        state.ac = tpl.armor_class.to_string();
        state.is_player = if tpl.is_player {
            "y".to_string()
        } else {
            "n".to_string()
        };
        state.step = 1; // next prompt will be initiative
        self.input_mode = InputMode::AddingCombatant(state);
        self.set_message(format!("Set initiative for template: {}", tpl.name));
        Ok(())
    }

    pub fn save_template_from_combatant(&mut self, combatant_index: usize) -> Result<(), String> {
        if combatant_index >= self.encounter.combatants.len() {
            return Err("Invalid combatant index".to_string());
        }
        let c = &self.encounter.combatants[combatant_index];
        let tpl =
            CombatantTemplate::from_stats(c.name.clone(), c.hp_max, c.armor_class, c.is_player);

        if let Some(existing) = self
            .templates
            .iter_mut()
            .find(|t| t.name.to_lowercase() == tpl.name.to_lowercase())
        {
            *existing = tpl.clone();
        } else {
            self.templates.push(tpl.clone());
        }

        if let Err(err) = save_templates(&self.templates) {
            self.set_message(format!(
                "Saved template in memory but failed to write file: {}",
                err
            ));
        } else {
            self.set_message(format!("Saved template: {}", tpl.name));
        }
        self.input_mode = InputMode::Normal;
        Ok(())
    }

    fn push_log(&mut self, message: String) {
        let entry = LogEntry::new(self.encounter.round_number, message);
        self.log.push(entry);
        if self.log.len() > 200 {
            let overflow = self.log.len() - 200;
            self.log.drain(0..overflow);
        }
    }

    pub fn complete_save_encounter(&mut self, filename: String) -> Result<(), String> {
        if filename.trim().is_empty() {
            return Err("Filename cannot be empty".to_string());
        }

        // Validate filename (alphanumeric, underscore, hyphen only)
        if !filename
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        {
            return Err("Filename can only contain letters, numbers, underscore, and hyphen".to_string());
        }

        match save_encounter(&self.encounter, &self.log, &filename) {
            Ok(()) => {
                self.set_message(format!("Successfully saved encounter: {}", filename));
                self.input_mode = InputMode::Normal;
                Ok(())
            }
            Err(err) => {
                log::error!("Failed to save encounter '{}': {}", filename, err);
                self.set_message(format!("Failed to save encounter: {}", err));
                self.input_mode = InputMode::Normal;
                Err(err)
            }
        }
    }

    pub fn complete_load_encounter(&mut self, filename: String) -> Result<(), String> {
        match load_encounter(&filename) {
            Ok(saved) => {
                self.encounter = saved.encounter;
                self.log = saved.log;
                self.set_message(format!("Successfully loaded encounter: {}", filename));
                self.input_mode = InputMode::Normal;
                Ok(())
            }
            Err(err) => {
                log::error!("Failed to load encounter '{}': {}", filename, err);
                self.set_message(format!("Failed to load encounter: {}", err));
                self.input_mode = InputMode::Normal;
                Err(err)
            }
        }
    }

    pub fn list_saved_encounters(&self) -> Vec<String> {
        list_encounter_files().unwrap_or_else(|_| Vec::new())
    }

    pub fn complete_save_library(&mut self, mut state: SaveLibraryState) -> Result<(), String> {
        // Validate name
        if state.name.trim().is_empty() {
            return Err("Name cannot be empty".to_string());
        }

        // Validate filename characters
        if !state.name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        {
            return Err(
                "Name can only contain letters, numbers, underscore, and hyphen".to_string()
            );
        }

        // Validate description
        if state.description.trim().is_empty() {
            return Err("Description cannot be empty".to_string());
        }

        // Trim inputs
        state.name = state.name.trim().to_string();
        state.description = state.description.trim().to_string();
        state.difficulty = state.difficulty.trim().to_string();

        // Check if already exists
        if library_template_exists(&state.name) {
            let name = state.name.clone();
            self.input_mode = InputMode::ConfirmingLibraryOverwrite(state);
            self.set_message(format!(
                "Library entry '{}' already exists. Overwrite? (y/n)",
                name
            ));
            return Ok(());
        }

        self.save_library_template_internal(state)
    }

    fn save_library_template_internal(&mut self, state: SaveLibraryState) -> Result<(), String> {
        // Convert current combatants to library combatants (fresh state)
        let library_combatants: Vec<LibraryCombatant> = self
            .encounter
            .combatants
            .iter()
            .map(|c| LibraryCombatant {
                name: c.name.clone(),
                hp_max: c.hp_max,
                armor_class: c.armor_class,
                is_player: c.is_player,
            })
            .collect();

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let template = EncounterTemplate {
            name: state.name.clone(),
            description: state.description.clone(),
            difficulty: state.difficulty.clone(),
            combatants: library_combatants,
            created_at: timestamp,
        };

        match save_library_template(&template, &state.name) {
            Ok(()) => {
                self.set_message(format!(
                    "Successfully saved to library: {}",
                    state.name
                ));
                self.input_mode = InputMode::Normal;
                Ok(())
            }
            Err(err) => {
                log::error!("Failed to save library template '{}': {}", state.name, err);
                self.set_message(format!("Failed to save library template: {}", err));
                self.input_mode = InputMode::Normal;
                Err(err)
            }
        }
    }

    pub fn confirm_overwrite_library(&mut self, state: SaveLibraryState) -> Result<(), String> {
        self.save_library_template_internal(state)
    }

    pub fn cancel_library_overwrite(&mut self) {
        self.input_mode = InputMode::Normal;
        self.set_message("Save cancelled".to_string());
    }

    pub fn select_library_template(&mut self, filename: String) -> Result<(), String> {
        // Check if encounter is empty
        if self.encounter.combatants.is_empty() {
            // Load directly without confirmation
            self.start_library_initiative_input(filename)
        } else {
            // Need confirmation
            self.input_mode = InputMode::ConfirmingLibraryLoad(filename);
            self.set_message(
                "This will clear the current encounter. Continue? (y/n)".to_string(),
            );
            Ok(())
        }
    }

    fn start_library_initiative_input(&mut self, filename: String) -> Result<(), String> {
        match load_library_template(&filename) {
            Ok(template) => {
                // Initialize state with all combatants
                let combatants_with_init: Vec<(LibraryCombatant, String)> = template
                    .combatants
                    .iter()
                    .map(|c| (c.clone(), String::new()))
                    .collect();

                self.input_mode = InputMode::SettingLibraryInitiatives(LoadLibraryState {
                    template,
                    combatants_with_init,
                    current_index: 0,
                });
                Ok(())
            }
            Err(err) => {
                log::error!("Failed to load library template '{}': {}", filename, err);
                self.set_message(format!("Failed to load library template: {}", err));
                self.input_mode = InputMode::Normal;
                Err(err)
            }
        }
    }

    pub fn confirm_load_library(&mut self, filename: String) -> Result<(), String> {
        self.start_library_initiative_input(filename)
    }

    pub fn cancel_library_load(&mut self) {
        self.input_mode = InputMode::Normal;
        self.set_message("Load cancelled".to_string());
    }

    pub fn complete_library_initiative(&mut self, initiative: String) -> Result<(), String> {
        let state = match &self.input_mode {
            InputMode::SettingLibraryInitiatives(s) => s.clone(),
            _ => return Err("Not in library initiative setting mode".to_string()),
        };

        // Parse initiative
        let init_value = initiative.parse::<i32>().map_err(|_| {
            self.set_message("Invalid initiative value".to_string());
            "Invalid initiative value".to_string()
        })?;

        // Update the current combatant's initiative
        let mut new_state = state.clone();
        new_state.combatants_with_init[state.current_index].1 = init_value.to_string();

        // Move to next combatant or finalize
        if state.current_index + 1 < state.combatants_with_init.len() {
            new_state.current_index += 1;
            self.input_mode = InputMode::SettingLibraryInitiatives(new_state);
            Ok(())
        } else {
            // All initiatives collected, load the encounter
            self.finalize_library_load(new_state)
        }
    }

    fn finalize_library_load(&mut self, state: LoadLibraryState) -> Result<(), String> {
        // Clear current encounter
        self.encounter.combatants.clear();
        self.encounter.current_turn_index = 0;
        self.encounter.round_number = 1;
        self.log.clear();

        // Create fresh combatants with entered initiatives
        for (lib_combatant, init_str) in state.combatants_with_init {
            let initiative = init_str.parse::<i32>().unwrap_or(10); // Fallback to 10
            let combatant = Combatant::new(
                lib_combatant.name,
                initiative,
                lib_combatant.hp_max,
                lib_combatant.armor_class,
                lib_combatant.is_player,
            );
            self.encounter.add_combatant(combatant);
        }

        self.set_message(format!(
            "Loaded encounter from library: {}",
            state.template.name
        ));
        self.push_log(format!(
            "Loaded encounter '{}' from library",
            state.template.name
        ));
        self.input_mode = InputMode::Normal;
        Ok(())
    }

    pub fn list_library_templates(&self) -> Vec<String> {
        list_library_files().unwrap_or_else(|_| Vec::new())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::combat::CombatEncounter;
    use std::path::Path;

    fn add_basic_combatant(app: &mut App, name: &str) {
        let state = AddCombatantState {
            step: 4,
            name: name.to_string(),
            initiative: "10".to_string(),
            hp: "20".to_string(),
            ac: "15".to_string(),
            is_player: "n".to_string(),
        };
        app.complete_add_combatant(state).unwrap();
    }

    #[test]
    fn granting_temp_hp_updates_combatant_and_logs() {
        let mut app = App::new();
        add_basic_combatant(&mut app, "Orc");
        app.complete_grant_temp_hp(0, 7).unwrap();
        assert_eq!(app.encounter.combatants[0].temp_hp, 7);
        assert!(app.message.as_ref().unwrap().contains("gains 7 temp HP"));
        assert!(app.log.last().unwrap().message.contains("gains 7 temp HP"));
    }

    #[test]
    fn log_is_capped_at_200_entries() {
        let mut app = App::new();
        for i in 0..205 {
            app.push_log(format!("entry {}", i));
        }
        assert_eq!(app.log.len(), 200);
        assert_eq!(app.log.first().unwrap().message, "entry 5");
    }

    #[test]
    fn damage_triggers_concentration_check() {
        let mut app = App::new();
        add_basic_combatant(&mut app, "Mage");
        app.encounter.combatants[0]
            .set_concentration(ConcentrationInfo::new("Haste".to_string(), 3));
        app.complete_deal_damage(0, 12).unwrap();
        match &app.input_mode {
            InputMode::ConcentrationCheck(state) => {
                assert_eq!(state.dc, 10); // max(10, damage/2)
                assert!(app.message.as_ref().unwrap().contains("Roll CON save"));
            }
            _ => panic!("Expected ConcentrationCheck mode"),
        }
        assert!(app.encounter.combatants[0].concentration.is_some());
    }

    // Save/Load Encounters Tests

    #[test]
    fn saved_encounter_serialization_roundtrip() {
        let mut encounter = CombatEncounter::new();
        let combatant = Combatant::new("Goblin".to_string(), 10, 20, 15, false);
        encounter.add_combatant(combatant);

        let mut log = Vec::new();
        log.push(LogEntry::new(1, "Test log entry".to_string()));

        let saved = SavedEncounter {
            encounter,
            log: log.clone(),
            saved_at: 1234567890,
            version: env!("CARGO_PKG_VERSION").to_string(),
        };

        // Serialize to JSON
        let json = serde_json::to_string(&saved).unwrap();

        // Deserialize back
        let loaded: SavedEncounter = serde_json::from_str(&json).unwrap();

        assert_eq!(loaded.encounter.combatants.len(), 1);
        assert_eq!(loaded.encounter.combatants[0].name, "Goblin");
        assert_eq!(loaded.log.len(), 1);
        assert_eq!(loaded.log[0].message, "Test log entry");
        assert_eq!(loaded.saved_at, 1234567890);
    }

    #[test]
    fn start_saving_encounter_enters_correct_mode() {
        let mut app = App::new();
        add_basic_combatant(&mut app, "Orc");

        app.start_saving_encounter();

        match &app.input_mode {
            InputMode::SavingEncounter(state) => {
                assert_eq!(state.input, "");
            }
            _ => panic!("Expected SavingEncounter mode"),
        }
    }

    #[test]
    fn start_loading_encounter_enters_correct_mode() {
        let mut app = App::new();

        app.start_loading_encounter();

        match &app.input_mode {
            InputMode::LoadingEncounter(state) => {
                assert_eq!(state.selected_index, 0);
                assert_eq!(state.input, "");
            }
            _ => panic!("Expected LoadingEncounter mode"),
        }
    }

    #[test]
    fn complete_save_encounter_validates_empty_filename() {
        let mut app = App::new();
        add_basic_combatant(&mut app, "Goblin");

        let result = app.complete_save_encounter("".to_string());

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Filename cannot be empty"));
    }

    #[test]
    fn complete_save_encounter_validates_invalid_characters() {
        let mut app = App::new();
        add_basic_combatant(&mut app, "Goblin");

        let result = app.complete_save_encounter("invalid/filename".to_string());

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("letters, numbers"));
    }

    #[test]
    fn complete_save_and_load_encounter_full_cycle() {
        use std::fs;
        use std::time::{SystemTime, UNIX_EPOCH};

        let mut app = App::new();
        add_basic_combatant(&mut app, "Dragon");
        app.encounter.combatants[0].hp_current = 50;
        app.encounter.round_number = 3;

        // Clear log from combatant creation and add specific test entry
        app.log.clear();
        app.push_log("Dragon breathes fire".to_string());

        // Generate unique filename for test
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let test_filename = format!("test_encounter_{}", timestamp);

        // Save encounter
        let save_result = app.complete_save_encounter(test_filename.clone());
        assert!(save_result.is_ok(), "Save failed: {:?}", save_result);

        // Verify file exists
        let file_path = format!("{}/{}.json", encounters_dir(), test_filename);
        assert!(Path::new(&file_path).exists());

        // Modify app state
        app.encounter.combatants[0].hp_current = 30;
        app.encounter.round_number = 5;
        app.log.clear();

        // Load encounter
        let load_result = app.complete_load_encounter(test_filename.clone());
        assert!(load_result.is_ok(), "Load failed: {:?}", load_result);

        // Verify state was restored
        assert_eq!(app.encounter.combatants.len(), 1);
        assert_eq!(app.encounter.combatants[0].name, "Dragon");
        assert_eq!(app.encounter.combatants[0].hp_current, 50);
        assert_eq!(app.encounter.round_number, 3);
        assert_eq!(app.log.len(), 1);
        assert_eq!(app.log[0].message, "Dragon breathes fire");

        // Cleanup
        let _ = fs::remove_file(&file_path);
    }

    #[test]
    fn complete_load_encounter_fails_for_missing_file() {
        let mut app = App::new();

        let result = app.complete_load_encounter("nonexistent_file_12345".to_string());

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found"));
    }

    #[test]
    fn list_saved_encounters_returns_sorted_list() {
        use std::fs;
        use std::time::{SystemTime, UNIX_EPOCH};

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut app = App::new();
        add_basic_combatant(&mut app, "Test");

        // Create multiple test encounters
        let filenames = vec![
            format!("ztest_{}", timestamp),
            format!("atest_{}", timestamp),
            format!("mtest_{}", timestamp),
        ];

        for filename in &filenames {
            app.complete_save_encounter(filename.clone()).unwrap();
        }

        let list = app.list_saved_encounters();

        // Verify alphabetical sorting
        let test_files: Vec<&String> = list.iter()
            .filter(|name| name.contains(&timestamp.to_string()))
            .collect();

        assert!(test_files.len() >= 3);

        // Check that our test files are in alphabetical order
        let mut sorted_test_files = test_files.clone();
        sorted_test_files.sort();
        assert_eq!(test_files, sorted_test_files);

        // Cleanup
        for filename in &filenames {
            let path = format!("{}/{}.json", encounters_dir(), filename);
            let _ = fs::remove_file(&path);
        }
    }

    // Note: Testing the case where the directory doesn't exist is difficult
    // in a multi-threaded test environment as other tests may create it.
    // The `list_encounter_files()` function returns an empty vec if the
    // directory doesn't exist, which is tested implicitly when the directory
    // is first created by other tests.

    #[test]
    fn save_encounter_includes_all_combatant_state() {
        use std::fs;
        use std::time::{SystemTime, UNIX_EPOCH};

        let mut app = App::new();
        add_basic_combatant(&mut app, "Wizard");

        // Add complex state
        app.encounter.combatants[0].hp_current = 15;
        app.encounter.combatants[0].temp_hp = 5;
        app.encounter.combatants[0].add_status_effect(
            StatusEffect::new(ConditionType::Poisoned, 10, None)
        );
        app.encounter.combatants[0].set_concentration(
            ConcentrationInfo::new("Shield".to_string(), 3)
        );

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let test_filename = format!("test_state_{}", timestamp);

        // Save and load
        app.complete_save_encounter(test_filename.clone()).unwrap();

        let mut new_app = App::new();
        new_app.complete_load_encounter(test_filename.clone()).unwrap();

        // Verify all state
        let combatant = &new_app.encounter.combatants[0];
        assert_eq!(combatant.name, "Wizard");
        assert_eq!(combatant.hp_current, 15);
        assert_eq!(combatant.temp_hp, 5);
        assert_eq!(combatant.status_effects.len(), 1);
        assert_eq!(combatant.status_effects[0].condition, ConditionType::Poisoned);
        assert!(combatant.concentration.is_some());
        assert_eq!(combatant.concentration.as_ref().unwrap().spell_name, "Shield");

        // Cleanup
        let path = format!("{}/{}.json", encounters_dir(), test_filename);
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn save_encounter_returns_to_normal_mode_on_success() {
        use std::fs;
        use std::time::{SystemTime, UNIX_EPOCH};

        let mut app = App::new();
        add_basic_combatant(&mut app, "Orc");

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let test_filename = format!("test_mode_{}", timestamp);

        app.complete_save_encounter(test_filename.clone()).unwrap();

        assert!(matches!(app.input_mode, InputMode::Normal));
        assert!(app.message.as_ref().unwrap().contains("saved"));

        // Cleanup
        let path = format!("{}/{}.json", encounters_dir(), test_filename);
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn load_encounter_returns_to_normal_mode_on_success() {
        use std::fs;
        use std::time::{SystemTime, UNIX_EPOCH};

        let mut app = App::new();
        add_basic_combatant(&mut app, "Goblin");

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let test_filename = format!("test_load_mode_{}", timestamp);

        app.complete_save_encounter(test_filename.clone()).unwrap();
        app.complete_load_encounter(test_filename.clone()).unwrap();

        assert!(matches!(app.input_mode, InputMode::Normal));
        assert!(app.message.as_ref().unwrap().contains("loaded"));

        // Cleanup
        let path = format!("{}/{}.json", encounters_dir(), test_filename);
        let _ = fs::remove_file(&path);
    }

    // Library feature tests

    #[test]
    fn library_template_serialization_roundtrip() {
        use std::time::{SystemTime, UNIX_EPOCH};

        let combatants = vec![
            LibraryCombatant {
                name: "Orc".to_string(),
                hp_max: 15,
                armor_class: 13,
                is_player: false,
            },
            LibraryCombatant {
                name: "Goblin".to_string(),
                hp_max: 7,
                armor_class: 15,
                is_player: false,
            },
        ];

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let template = EncounterTemplate {
            name: "Orc Ambush".to_string(),
            description: "A surprise attack from orcs".to_string(),
            difficulty: "Medium".to_string(),
            combatants: combatants.clone(),
            created_at: timestamp,
        };

        let json = serde_json::to_string(&template).unwrap();
        let deserialized: EncounterTemplate = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.name, "Orc Ambush");
        assert_eq!(deserialized.description, "A surprise attack from orcs");
        assert_eq!(deserialized.difficulty, "Medium");
        assert_eq!(deserialized.combatants.len(), 2);
        assert_eq!(deserialized.combatants[0].name, "Orc");
        assert_eq!(deserialized.combatants[0].hp_max, 15);
        assert_eq!(deserialized.combatants[1].name, "Goblin");
    }

    #[test]
    fn start_saving_library_validates_non_empty_encounter() {
        let mut app = App::new();
        app.start_saving_library();

        // Should set error message and remain in Normal mode
        assert!(matches!(app.input_mode, InputMode::Normal));
        assert_eq!(
            app.message.as_ref().unwrap(),
            "Cannot save empty encounter to library"
        );
    }

    #[test]
    fn complete_save_library_validates_empty_name() {
        let state = SaveLibraryState {
            step: 2,
            name: "".to_string(),
            description: "Test description".to_string(),
            difficulty: "Easy".to_string(),
        };

        let mut app = App::new();
        let result = app.complete_save_library(state);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Name cannot be empty");
    }

    #[test]
    fn complete_save_library_validates_invalid_characters() {
        let state = SaveLibraryState {
            step: 2,
            name: "invalid/name".to_string(),
            description: "Test description".to_string(),
            difficulty: "Easy".to_string(),
        };

        let mut app = App::new();
        let result = app.complete_save_library(state);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("can only contain"));
    }

    #[test]
    fn complete_save_library_validates_empty_description() {
        let state = SaveLibraryState {
            step: 2,
            name: "valid_name".to_string(),
            description: "".to_string(),
            difficulty: "Easy".to_string(),
        };

        let mut app = App::new();
        let result = app.complete_save_library(state);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Description cannot be empty");
    }

    #[test]
    fn complete_save_and_load_library_full_cycle() {
        use std::fs;
        use std::time::{SystemTime, UNIX_EPOCH};

        let mut app = App::new();

        // Add combatants with various states
        add_basic_combatant(&mut app, "Dragon");
        app.encounter.combatants[0].hp_current = 50; // Partial damage
        app.encounter.combatants[0].temp_hp = 10;

        add_basic_combatant(&mut app, "Kobold");
        app.encounter.combatants[1].hp_current = 7;

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let test_name = format!("test_library_{}", timestamp);

        // Save to library
        let state = SaveLibraryState {
            step: 2,
            name: test_name.clone(),
            description: "Test encounter with dragon and kobold".to_string(),
            difficulty: "Hard".to_string(),
        };

        let save_result = app.complete_save_library(state);
        assert!(save_result.is_ok(), "Save failed: {:?}", save_result);

        // Verify file exists
        let file_path = format!("{}/{}.json", library_dir(), test_name);
        assert!(Path::new(&file_path).exists());

        // Load the template
        let loaded_template = load_library_template(&test_name);
        assert!(loaded_template.is_ok());

        let template = loaded_template.unwrap();
        assert_eq!(template.name, test_name);
        assert_eq!(template.description, "Test encounter with dragon and kobold");
        assert_eq!(template.difficulty, "Hard");
        assert_eq!(template.combatants.len(), 2);

        // Verify combatant data is fresh (no runtime state)
        assert_eq!(template.combatants[0].name, "Dragon");
        assert_eq!(template.combatants[0].hp_max, 20);
        assert_eq!(template.combatants[0].armor_class, 15);

        assert_eq!(template.combatants[1].name, "Kobold");
        assert_eq!(template.combatants[1].hp_max, 20);

        // Cleanup
        let _ = fs::remove_file(&file_path);
    }

    #[test]
    fn library_combatant_strips_runtime_state() {
        use std::fs;
        use std::time::{SystemTime, UNIX_EPOCH};

        let mut app = App::new();

        // Add combatant with various runtime state
        add_basic_combatant(&mut app, "Orc");
        app.encounter.combatants[0].hp_current = 5; // Damaged
        app.encounter.combatants[0].temp_hp = 8;
        app.encounter.combatants[0].concentration = Some(crate::models::ConcentrationInfo {
            spell_name: "Bless".to_string(),
            constitution_modifier: 2,
        });
        app.encounter.combatants[0].status_effects.push(
            crate::models::StatusEffect::new(
                crate::models::ConditionType::Poisoned,
                3,
                None,
            )
        );

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let test_name = format!("test_strip_{}", timestamp);

        let state = SaveLibraryState {
            step: 2,
            name: test_name.clone(),
            description: "Test state stripping".to_string(),
            difficulty: "".to_string(),
        };

        app.complete_save_library(state).unwrap();

        // Load and verify state was stripped
        let template = load_library_template(&test_name).unwrap();
        let lib_combatant = &template.combatants[0];

        // Only basic stats should be present
        assert_eq!(lib_combatant.name, "Orc");
        assert_eq!(lib_combatant.hp_max, 20);
        assert_eq!(lib_combatant.armor_class, 15);
        assert!(!lib_combatant.is_player);

        // Note: LibraryCombatant doesn't have hp_current, temp_hp,
        // concentration, status_effects, or initiative fields at all

        // Cleanup
        let path = format!("{}/{}.json", library_dir(), test_name);
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn library_template_exists_check() {
        use std::fs;
        use std::time::{SystemTime, UNIX_EPOCH};

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let test_name = format!("test_exists_{}", timestamp);

        // Should not exist initially
        assert!(!library_template_exists(&test_name));

        // Create template
        let mut app = App::new();
        add_basic_combatant(&mut app, "Goblin");

        let state = SaveLibraryState {
            step: 2,
            name: test_name.clone(),
            description: "Existence test".to_string(),
            difficulty: "".to_string(),
        };

        app.complete_save_library(state).unwrap();

        // Should exist now
        assert!(library_template_exists(&test_name));

        // Cleanup
        let path = format!("{}/{}.json", library_dir(), test_name);
        let _ = fs::remove_file(&path);

        // Should not exist after cleanup
        assert!(!library_template_exists(&test_name));
    }

    #[test]
    fn list_library_templates_returns_sorted_list() {
        use std::fs;
        use std::time::{SystemTime, UNIX_EPOCH};

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let names = vec![
            format!("test_list_z_{}", timestamp),
            format!("test_list_a_{}", timestamp),
            format!("test_list_m_{}", timestamp),
        ];

        // Create templates
        let mut app = App::new();
        add_basic_combatant(&mut app, "Test");

        for name in &names {
            let state = SaveLibraryState {
                step: 2,
                name: name.clone(),
                description: "List test".to_string(),
                difficulty: "".to_string(),
            };
            app.complete_save_library(state).unwrap();
        }

        // Get list
        let list = list_library_files().unwrap();

        // Verify all test files are present
        for name in &names {
            assert!(list.contains(name), "List should contain {}", name);
        }

        // Verify list is sorted
        let test_entries: Vec<_> = list.iter()
            .filter(|n| n.starts_with("test_list_"))
            .collect();
        let mut sorted = test_entries.clone();
        sorted.sort();
        assert_eq!(test_entries, sorted);

        // Cleanup
        for name in &names {
            let path = format!("{}/{}.json", library_dir(), name);
            let _ = fs::remove_file(&path);
        }
    }

    #[test]
    fn complete_save_library_detects_duplicate_and_prompts_confirmation() {
        use std::fs;
        use std::time::{SystemTime, UNIX_EPOCH};

        let mut app = App::new();
        add_basic_combatant(&mut app, "Troll");

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let test_name = format!("test_overwrite_{}", timestamp);

        // Save first time
        let state1 = SaveLibraryState {
            step: 2,
            name: test_name.clone(),
            description: "First version".to_string(),
            difficulty: "Easy".to_string(),
        };

        let result1 = app.complete_save_library(state1);
        assert!(result1.is_ok());
        assert!(matches!(app.input_mode, InputMode::Normal));

        // Try to save again with same name
        let state2 = SaveLibraryState {
            step: 2,
            name: test_name.clone(),
            description: "Second version".to_string(),
            difficulty: "Hard".to_string(),
        };

        let result2 = app.complete_save_library(state2);
        assert!(result2.is_ok());

        // Should be in confirmation mode
        assert!(matches!(app.input_mode, InputMode::ConfirmingLibraryOverwrite(_)));
        assert!(app.message.as_ref().unwrap().contains("already exists"));
        assert!(app.message.as_ref().unwrap().contains("Overwrite?"));

        // Cleanup
        let path = format!("{}/{}.json", library_dir(), test_name);
        let _ = fs::remove_file(&path);
    }

    #[test]
    fn finalize_library_load_creates_fresh_combatants() {
        use std::fs;
        use std::time::{SystemTime, UNIX_EPOCH};

        let mut app = App::new();

        // Create and save a template
        add_basic_combatant(&mut app, "Orc");
        add_basic_combatant(&mut app, "Goblin");

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let test_name = format!("test_fresh_{}", timestamp);

        let state = SaveLibraryState {
            step: 2,
            name: test_name.clone(),
            description: "Fresh combatants test".to_string(),
            difficulty: "Medium".to_string(),
        };

        app.complete_save_library(state).unwrap();

        // Load template
        let template = load_library_template(&test_name).unwrap();

        // Create LoadLibraryState with initiatives
        let combatants_with_init: Vec<(LibraryCombatant, String)> = template
            .combatants
            .iter()
            .enumerate()
            .map(|(i, c)| (c.clone(), (15 + i).to_string()))
            .collect();

        let load_state = LoadLibraryState {
            template: template.clone(),
            combatants_with_init,
            current_index: 0,
        };

        // Clear encounter and finalize load
        app.encounter.combatants.clear();
        app.finalize_library_load(load_state).unwrap();

        // Verify combatants are fresh
        assert_eq!(app.encounter.combatants.len(), 2);

        // Combatants are sorted by initiative (highest first)
        // Goblin has initiative 16, Orc has initiative 15
        let goblin = &app.encounter.combatants[0];
        assert_eq!(goblin.name, "Goblin");
        assert_eq!(goblin.hp_current, goblin.hp_max); // Full HP
        assert_eq!(goblin.temp_hp, 0);
        assert_eq!(goblin.initiative, 16);
        assert!(goblin.concentration.is_none());
        assert!(goblin.status_effects.is_empty());

        let orc = &app.encounter.combatants[1];
        assert_eq!(orc.name, "Orc");
        assert_eq!(orc.hp_current, orc.hp_max); // Full HP
        assert_eq!(orc.initiative, 15);

        // Cleanup
        let path = format!("{}/{}.json", library_dir(), test_name);
        let _ = fs::remove_file(&path);
    }
}
