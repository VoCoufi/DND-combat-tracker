use crate::app::{
    AddConcentrationState, App, ClearAction, ConcentrationCheckState, ConditionSelectionState,
    InputMode, LoadLibraryState, SaveEncounterState, SaveLibraryState, SelectionState,
    StatusSelectionState,
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle_key_event(app: &mut App, key: KeyEvent) {
    match app.input_mode.clone() {
        InputMode::Normal => handle_normal_mode(app, key),
        InputMode::AddingCombatant(_) => handle_add_combatant_mode(app, key),
        InputMode::DealingDamage(_) => handle_selection_mode(app, key, |app, idx, input| {
            if let Ok(damage) = input.parse::<i32>() {
                if let Err(e) = app.complete_deal_damage(idx, damage) {
                    app.set_message(e);
                }
            } else {
                app.set_message("Invalid damage value!".to_string());
                app.input_mode = InputMode::Normal;
            }
        }),
        InputMode::Healing(_) => handle_selection_mode(app, key, |app, idx, input| {
            if let Ok(amount) = input.parse::<i32>() {
                if let Err(e) = app.complete_heal(idx, amount) {
                    app.set_message(e);
                }
            } else {
                app.set_message("Invalid heal value!".to_string());
                app.input_mode = InputMode::Normal;
            }
        }),
        InputMode::AddingStatus(_) => handle_status_selection_mode(app, key),
        InputMode::SelectingCondition(state) => handle_condition_selection_mode(app, key, state),
        InputMode::RollingDeathSave(_) => handle_selection_mode(app, key, |app, idx, input| {
            if let Ok(roll) = input.parse::<i32>() {
                if let Err(e) = app.complete_death_save_roll(idx, roll) {
                    app.set_message(e);
                }
            } else {
                app.set_message("Invalid roll value!".to_string());
                app.input_mode = InputMode::Normal;
            }
        }),
        InputMode::ConcentrationTarget(_) => handle_selection_mode(app, key, |app, idx, _| {
            app.input_mode = InputMode::ApplyingConcentration(AddConcentrationState {
                combatant_index: idx,
                ..Default::default()
            });
        }),
        InputMode::ApplyingConcentration(state) => handle_add_concentration_mode(app, key, state),
        InputMode::ConcentrationCheck(state) => handle_concentration_check_mode(app, key, state),
        InputMode::ClearingConcentration(_) => handle_selection_mode(app, key, |app, idx, _| {
            if let Err(e) = app.complete_clear_concentration(idx) {
                app.set_message(e);
            }
        }),
        InputMode::ClearActionSelection(choice) => handle_clear_choice_mode(app, key, choice),
        InputMode::ClearingStatus(_) => handle_selection_mode(app, key, |app, idx, _| {
            if let Some(combatant) = app.encounter.combatants.get(idx) {
                if combatant.status_effects.is_empty() {
                    if let Err(e) = app.complete_clear_status_effect(idx, None) {
                        app.set_message(e);
                    }
                } else if combatant.status_effects.len() == 1 {
                    if let Err(e) = app.complete_clear_status_effect(idx, Some(0)) {
                        app.set_message(e);
                    }
                } else {
                    app.input_mode = InputMode::SelectingStatusToClear(StatusSelectionState {
                        combatant_index: idx,
                        selected_status_index: 0,
                    });
                }
            } else {
                app.set_message("Invalid combatant selection".to_string());
                app.input_mode = InputMode::Normal;
            }
        }),
        InputMode::GrantingTempHp(_) => handle_selection_mode(app, key, |app, idx, input| {
            if let Ok(amount) = input.parse::<i32>() {
                if let Err(e) = app.complete_grant_temp_hp(idx, amount) {
                    app.set_message(e);
                }
            } else {
                app.set_message("Invalid temp HP value!".to_string());
                app.input_mode = InputMode::Normal;
            }
        }),
        InputMode::SelectingStatusToClear(state) => handle_status_clear_selection(app, key, state),
        InputMode::SelectingTemplate(state) => handle_template_selection_mode(app, key, state),
        InputMode::SavingTemplate(_) => handle_selection_mode(app, key, |app, idx, _| {
            if let Err(e) = app.save_template_from_combatant(idx) {
                app.set_message(e);
            }
        }),
        InputMode::ActionMenu(selected) => handle_action_menu_mode(app, key, selected),
        InputMode::CombatantMenu(selected) => handle_combatant_menu_mode(app, key, selected),
        InputMode::QuickReference(selected) => handle_quick_reference_mode(app, key, selected),
        InputMode::Removing(_) => handle_removing_mode(app, key),
        InputMode::SavingEncounter(state) => handle_save_encounter_mode(app, key, state),
        InputMode::LoadingEncounter(state) => handle_load_encounter_mode(app, key, state),
        InputMode::SavingLibrary(state) => handle_save_library_mode(app, key, state),
        InputMode::LoadingLibrary(state) => handle_loading_library_mode(app, key, state),
        InputMode::SettingLibraryInitiatives(state) => {
            handle_setting_library_initiatives_mode(app, key, state)
        }
        InputMode::ConfirmingLibraryOverwrite(state) => {
            handle_confirm_library_overwrite(app, key, state)
        }
        InputMode::ConfirmingLibraryLoad(filename) => {
            handle_confirm_library_load(app, key, filename)
        }
    }
}

fn handle_normal_mode(app: &mut App, key: KeyEvent) {
    // Check for Ctrl key combinations first
    if key.modifiers.contains(KeyModifiers::CONTROL) {
        match key.code {
            KeyCode::Char('s') | KeyCode::Char('S') => {
                app.start_saving_encounter();
                return;
            }
            KeyCode::Char('o') | KeyCode::Char('O') => {
                app.start_loading_encounter();
                return;
            }
            _ => {}
        }
    }

    // Regular key bindings
    match key.code {
        KeyCode::Char('q') => app.quit(),
        KeyCode::Char('n') => {
            app.encounter.next_turn();
            app.clear_message();
        }
        KeyCode::Char('a') => app.start_adding_combatant(),
        KeyCode::Char('d') => app.start_dealing_damage(),
        KeyCode::Char('h') => app.start_healing(),
        KeyCode::Char('s') => app.start_adding_status(),
        KeyCode::Char('v') => app.start_rolling_death_save(),
        KeyCode::Char('c') => app.start_concentration_target(),
        KeyCode::Char('x') => app.start_clear_choice(),
        KeyCode::Char('m') => app.open_action_menu(),
        KeyCode::Char('b') => app.open_combatant_menu(),
        KeyCode::Char('?') => app.input_mode = InputMode::QuickReference(0),
        _ => {}
    }
}

fn handle_add_combatant_mode(app: &mut App, key: KeyEvent) {
    if let InputMode::AddingCombatant(mut state) = app.input_mode.clone() {
        match key.code {
            KeyCode::Esc => app.cancel_input(),
            KeyCode::Enter => {
                if state.step < 4 {
                    state.step += 1;
                    app.input_mode = InputMode::AddingCombatant(state);
                } else {
                    if let Err(e) = app.complete_add_combatant(state) {
                        app.set_message(e);
                        app.input_mode = InputMode::Normal;
                    }
                }
            }
            KeyCode::Backspace => {
                match state.step {
                    0 => {
                        state.name.pop();
                    }
                    1 => {
                        state.initiative.pop();
                    }
                    2 => {
                        state.hp.pop();
                    }
                    3 => {
                        state.ac.pop();
                    }
                    4 => {
                        state.is_player.pop();
                    }
                    _ => {}
                }
                app.input_mode = InputMode::AddingCombatant(state);
            }
            KeyCode::Char(c) => {
                match state.step {
                    0 => state.name.push(c),
                    1 => {
                        if c.is_ascii_digit() || c == '-' {
                            state.initiative.push(c);
                        }
                    }
                    2 => {
                        if c.is_ascii_digit() {
                            state.hp.push(c);
                        }
                    }
                    3 => {
                        if c.is_ascii_digit() {
                            state.ac.push(c);
                        }
                    }
                    4 => {
                        if c == 'y' || c == 'n' || c == 'Y' || c == 'N' {
                            state.is_player.clear();
                            state.is_player.push(c);
                        }
                    }
                    _ => {}
                }
                app.input_mode = InputMode::AddingCombatant(state);
            }
            _ => {}
        }
    }
}

fn handle_selection_mode<F>(app: &mut App, key: KeyEvent, on_confirm: F)
where
    F: FnOnce(&mut App, usize, String),
{
    let (selected_index, mut input, allow_empty_confirm) = match &app.input_mode {
        InputMode::DealingDamage(state) => (state.selected_index, state.input.clone(), false),
        InputMode::Healing(state) => (state.selected_index, state.input.clone(), false),
        InputMode::RollingDeathSave(state) => (state.selected_index, state.input.clone(), false),
        InputMode::ConcentrationTarget(state) => (state.selected_index, state.input.clone(), true),
        InputMode::ClearingConcentration(state) => {
            (state.selected_index, state.input.clone(), true)
        }
        InputMode::ClearingStatus(state) => (state.selected_index, state.input.clone(), true),
        InputMode::SavingTemplate(state) => (state.selected_index, state.input.clone(), true),
        InputMode::GrantingTempHp(state) => (state.selected_index, state.input.clone(), false),
        InputMode::SelectingStatusToClear(_) => return,
        InputMode::ActionMenu(_) | InputMode::CombatantMenu(_) | InputMode::QuickReference(_) => {
            return;
        }
        _ => return,
    };

    match key.code {
        KeyCode::Esc => app.cancel_input(),
        KeyCode::Up => {
            let new_index = if selected_index > 0 {
                selected_index - 1
            } else {
                app.encounter.combatants.len().saturating_sub(1)
            };
            update_selection_state(app, new_index, input);
        }
        KeyCode::Down => {
            let new_index = if selected_index < app.encounter.combatants.len().saturating_sub(1) {
                selected_index + 1
            } else {
                0
            };
            update_selection_state(app, new_index, input);
        }
        KeyCode::Enter => {
            if allow_empty_confirm || !input.is_empty() {
                on_confirm(app, selected_index, input);
            }
        }
        KeyCode::Backspace => {
            input.pop();
            update_selection_state(app, selected_index, input);
        }
        KeyCode::Char(c) => {
            if c.is_ascii_digit() {
                input.push(c);
                update_selection_state(app, selected_index, input);
            }
        }
        _ => {}
    }
}

fn update_selection_state(app: &mut App, index: usize, input: String) {
    let new_state = SelectionState {
        selected_index: index,
        input,
    };

    app.input_mode = match app.input_mode.clone() {
        InputMode::DealingDamage(_) => InputMode::DealingDamage(new_state),
        InputMode::Healing(_) => InputMode::Healing(new_state),
        InputMode::AddingStatus(_) => InputMode::AddingStatus(new_state),
        InputMode::RollingDeathSave(_) => InputMode::RollingDeathSave(new_state),
        InputMode::ConcentrationTarget(_) => InputMode::ConcentrationTarget(new_state),
        InputMode::ClearingConcentration(_) => InputMode::ClearingConcentration(new_state),
        InputMode::ClearingStatus(_) => InputMode::ClearingStatus(new_state),
        InputMode::SelectingStatusToClear(state) => {
            InputMode::SelectingStatusToClear(StatusSelectionState {
                combatant_index: state.combatant_index,
                selected_status_index: new_state.selected_index,
            })
        }
        InputMode::SelectingTemplate(_) => InputMode::SelectingTemplate(new_state),
        InputMode::SavingTemplate(_) => InputMode::SavingTemplate(new_state),
        InputMode::GrantingTempHp(_) => InputMode::GrantingTempHp(new_state),
        InputMode::Removing(_) => InputMode::Removing(new_state),
        _ => app.input_mode.clone(),
    };
}

fn handle_status_selection_mode(app: &mut App, key: KeyEvent) {
    if let InputMode::AddingStatus(state) = &app.input_mode {
        let selected_index = state.selected_index;

        match key.code {
            KeyCode::Esc => app.cancel_input(),
            KeyCode::Up => {
                let new_index = if selected_index > 0 {
                    selected_index - 1
                } else {
                    app.encounter.combatants.len().saturating_sub(1)
                };
                update_selection_state(app, new_index, String::new());
            }
            KeyCode::Down => {
                let new_index = if selected_index < app.encounter.combatants.len().saturating_sub(1)
                {
                    selected_index + 1
                } else {
                    0
                };
                update_selection_state(app, new_index, String::new());
            }
            KeyCode::Enter => {
                app.input_mode = InputMode::SelectingCondition(ConditionSelectionState {
                    combatant_index: selected_index,
                    input: String::new(),
                });
            }
            _ => {}
        }
    }
}

fn handle_condition_selection_mode(app: &mut App, key: KeyEvent, state: ConditionSelectionState) {
    let mut input = state.input;
    let combatant_index = state.combatant_index;

    match key.code {
        KeyCode::Esc => app.cancel_input(),
        KeyCode::Backspace => {
            input.pop();
            app.input_mode = InputMode::SelectingCondition(ConditionSelectionState {
                combatant_index,
                input,
            });
        }
        KeyCode::Char(c) => {
            if c.is_ascii_digit() || c == ' ' {
                // Allow only one space separator
                if c != ' ' || !input.contains(' ') {
                    input.push(c);
                    app.input_mode = InputMode::SelectingCondition(ConditionSelectionState {
                        combatant_index,
                        input,
                    });
                }
            }
        }
        KeyCode::Enter => {
            let parts: Vec<&str> = input.split_whitespace().collect();
            if parts.len() != 2 {
                app.set_message("Enter condition number and duration (e.g., 1 3)".to_string());
                return;
            }

            let condition_idx = match parts[0].parse::<usize>() {
                Ok(idx) if idx >= 1 && idx <= crate::models::ConditionType::all().len() => idx - 1,
                _ => {
                    app.set_message("Invalid condition number".to_string());
                    return;
                }
            };

            let duration = match parts[1].parse::<i32>() {
                Ok(d) if d >= 0 => d,
                _ => {
                    app.set_message("Duration must be a non-negative number".to_string());
                    return;
                }
            };

            let condition = crate::models::ConditionType::all()[condition_idx];
            if let Err(e) = app.complete_add_status(combatant_index, condition, duration) {
                app.set_message(e);
            }
        }
        _ => {}
    }
}

fn handle_add_concentration_mode(app: &mut App, key: KeyEvent, state: AddConcentrationState) {
    let mut state = state;
    match key.code {
        KeyCode::Esc => app.cancel_input(),
        KeyCode::Enter => {
            if state.step < 1 {
                state.step += 1;
                app.input_mode = InputMode::ApplyingConcentration(state);
            } else if let Err(e) = app.complete_apply_concentration(state) {
                app.set_message(e);
                app.input_mode = InputMode::Normal;
            }
        }
        KeyCode::Backspace => {
            match state.step {
                0 => {
                    state.spell_name.pop();
                }
                1 => {
                    state.con_mod.pop();
                }
                _ => {}
            }
            app.input_mode = InputMode::ApplyingConcentration(state);
        }
        KeyCode::Char(c) => {
            match state.step {
                0 => state.spell_name.push(c),
                1 => {
                    if c.is_ascii_digit() || c == '-' {
                        state.con_mod.push(c);
                    }
                }
                _ => {}
            }
            app.input_mode = InputMode::ApplyingConcentration(state);
        }
        _ => {}
    }
}

fn handle_concentration_check_mode(app: &mut App, key: KeyEvent, state: ConcentrationCheckState) {
    let mut input = state.input.clone();
    match key.code {
        KeyCode::Esc => app.cancel_input(),
        KeyCode::Backspace => {
            input.pop();
            app.input_mode =
                InputMode::ConcentrationCheck(ConcentrationCheckState { input, ..state });
        }
        KeyCode::Char(c) => {
            if c.is_ascii_digit() {
                input.push(c);
                app.input_mode =
                    InputMode::ConcentrationCheck(ConcentrationCheckState { input, ..state });
            }
        }
        KeyCode::Enter => {
            if let Ok(total) = input.parse::<i32>() {
                if let Err(e) = app.complete_concentration_check(state.clone(), total) {
                    app.set_message(e);
                }
            } else {
                app.set_message("Invalid roll total".to_string());
                app.input_mode = InputMode::Normal;
            }
        }
        _ => {}
    }
}

fn handle_quick_reference_mode(app: &mut App, key: KeyEvent, selected_index: usize) {
    let total = crate::models::ConditionType::all().len();
    match key.code {
        KeyCode::Esc | KeyCode::Char('q') => app.cancel_input(),
        KeyCode::Up => {
            let new_idx = if selected_index > 0 {
                selected_index - 1
            } else {
                total.saturating_sub(1)
            };
            app.input_mode = InputMode::QuickReference(new_idx);
        }
        KeyCode::Down => {
            let new_idx = if selected_index + 1 < total {
                selected_index + 1
            } else {
                0
            };
            app.input_mode = InputMode::QuickReference(new_idx);
        }
        _ => {}
    }
}

fn handle_removing_mode(app: &mut App, key: KeyEvent) {
    if let InputMode::Removing(state) = &app.input_mode {
        let selected_index = state.selected_index;

        match key.code {
            KeyCode::Esc => app.cancel_input(),
            KeyCode::Up => {
                let new_index = if selected_index > 0 {
                    selected_index - 1
                } else {
                    app.encounter.combatants.len().saturating_sub(1)
                };
                update_selection_state(app, new_index, String::new());
            }
            KeyCode::Down => {
                let new_index = if selected_index < app.encounter.combatants.len().saturating_sub(1)
                {
                    selected_index + 1
                } else {
                    0
                };
                update_selection_state(app, new_index, String::new());
            }
            KeyCode::Enter => {
                if let Err(e) = app.complete_remove(selected_index) {
                    app.set_message(e);
                }
            }
            _ => {}
        }
    }
}

fn handle_clear_choice_mode(app: &mut App, key: KeyEvent, choice: ClearAction) {
    let mut choice = choice;
    match key.code {
        KeyCode::Esc => app.cancel_input(),
        KeyCode::Up | KeyCode::Down => {
            choice = match choice {
                ClearAction::Concentration => ClearAction::StatusEffects,
                ClearAction::StatusEffects => ClearAction::Concentration,
            };
            app.input_mode = InputMode::ClearActionSelection(choice);
        }
        KeyCode::Enter => match choice {
            ClearAction::Concentration => {
                app.input_mode = InputMode::ClearingConcentration(SelectionState::default())
            }
            ClearAction::StatusEffects => {
                app.input_mode = InputMode::ClearingStatus(SelectionState::default())
            }
        },
        _ => {}
    }
}

#[derive(Clone, Copy)]
enum ActionMenuItem {
    Damage,
    Heal,
    AddStatus,
    DeathSave,
    Concentration,
    ClearMenu,
    TempHp,
}

fn action_menu_items() -> Vec<(ActionMenuItem, &'static str)> {
    vec![
        (ActionMenuItem::Damage, "Deal Damage"),
        (ActionMenuItem::Heal, "Heal"),
        (ActionMenuItem::AddStatus, "Add Status Effect"),
        (ActionMenuItem::DeathSave, "Roll Death Save"),
        (ActionMenuItem::Concentration, "Set Concentration"),
        (ActionMenuItem::ClearMenu, "Clear Concentration/Status"),
        (ActionMenuItem::TempHp, "Grant Temp HP"),
    ]
}

fn handle_action_menu_mode(app: &mut App, key: KeyEvent, selected_index: usize) {
    let items = action_menu_items();
    match key.code {
        KeyCode::Esc => app.cancel_input(),
        KeyCode::Up => {
            let new_idx = if selected_index > 0 {
                selected_index - 1
            } else {
                items.len().saturating_sub(1)
            };
            app.input_mode = InputMode::ActionMenu(new_idx);
        }
        KeyCode::Down => {
            let new_idx = if selected_index + 1 < items.len() {
                selected_index + 1
            } else {
                0
            };
            app.input_mode = InputMode::ActionMenu(new_idx);
        }
        KeyCode::Enter => {
            if let Some((action, _)) = items.get(selected_index) {
                match action {
                    ActionMenuItem::Damage => app.start_dealing_damage(),
                    ActionMenuItem::Heal => app.start_healing(),
                    ActionMenuItem::AddStatus => app.start_adding_status(),
                    ActionMenuItem::DeathSave => app.start_rolling_death_save(),
                    ActionMenuItem::Concentration => app.start_concentration_target(),
                    ActionMenuItem::ClearMenu => app.start_clear_choice(),
                    ActionMenuItem::TempHp => app.start_granting_temp_hp(),
                }
            }
        }
        _ => {}
    }
}

#[derive(Clone, Copy)]
enum CombatantMenuItem {
    AddCombatant,
    RemoveCombatant,
    LoadTemplate,
    SaveTemplate,
    LoadLibrary,
    SaveLibrary,
}

fn combatant_menu_items() -> Vec<(CombatantMenuItem, &'static str)> {
    vec![
        (CombatantMenuItem::AddCombatant, "Add Combatant"),
        (CombatantMenuItem::RemoveCombatant, "Remove Combatant"),
        (CombatantMenuItem::LoadTemplate, "Add from Template"),
        (CombatantMenuItem::SaveTemplate, "Save as Template"),
        (CombatantMenuItem::LoadLibrary, "Load Encounter Library"),
        (CombatantMenuItem::SaveLibrary, "Save to Encounter Library"),
    ]
}

fn handle_combatant_menu_mode(app: &mut App, key: KeyEvent, selected_index: usize) {
    let items = combatant_menu_items();
    match key.code {
        KeyCode::Esc => app.cancel_input(),
        KeyCode::Up => {
            let new_idx = if selected_index > 0 {
                selected_index - 1
            } else {
                items.len().saturating_sub(1)
            };
            app.input_mode = InputMode::CombatantMenu(new_idx);
        }
        KeyCode::Down => {
            let new_idx = if selected_index + 1 < items.len() {
                selected_index + 1
            } else {
                0
            };
            app.input_mode = InputMode::CombatantMenu(new_idx);
        }
        KeyCode::Enter => {
            if let Some((action, _)) = items.get(selected_index) {
                match action {
                    CombatantMenuItem::AddCombatant => app.start_adding_combatant(),
                    CombatantMenuItem::RemoveCombatant => app.start_removing(),
                    CombatantMenuItem::LoadTemplate => app.start_selecting_template(),
                    CombatantMenuItem::SaveTemplate => app.start_saving_template(),
                    CombatantMenuItem::LoadLibrary => app.start_loading_library(),
                    CombatantMenuItem::SaveLibrary => app.start_saving_library(),
                }
            }
        }
        _ => {}
    }
}
fn handle_status_clear_selection(app: &mut App, key: KeyEvent, state: StatusSelectionState) {
    let mut selected = state.selected_status_index;
    let combatant_index = state.combatant_index;

    match key.code {
        KeyCode::Esc => app.cancel_input(),
        KeyCode::Up => {
            if selected > 0 {
                selected -= 1;
            } else {
                if let Some(combatant) = app.encounter.combatants.get(combatant_index) {
                    selected = combatant.status_effects.len().saturating_sub(1);
                }
            }
            app.input_mode = InputMode::SelectingStatusToClear(StatusSelectionState {
                combatant_index,
                selected_status_index: selected,
            });
        }
        KeyCode::Down => {
            if let Some(combatant) = app.encounter.combatants.get(combatant_index) {
                if selected + 1 < combatant.status_effects.len() {
                    selected += 1;
                } else {
                    selected = 0;
                }
            }
            app.input_mode = InputMode::SelectingStatusToClear(StatusSelectionState {
                combatant_index,
                selected_status_index: selected,
            });
        }
        KeyCode::Enter => {
            if let Err(e) = app.complete_clear_status_effect(combatant_index, Some(selected)) {
                app.set_message(e);
            }
        }
        _ => {}
    }
}

fn handle_template_selection_mode(app: &mut App, key: KeyEvent, state: SelectionState) {
    let mut selected_index = state.selected_index;
    let mut input = state.input.clone();
    let filtered: Vec<usize> = app
        .templates
        .iter()
        .enumerate()
        .filter(|(_, t)| t.name.to_lowercase().contains(&input.to_lowercase()))
        .map(|(idx, _)| idx)
        .collect();

    match key.code {
        KeyCode::Esc => app.cancel_input(),
        KeyCode::Up => {
            if !filtered.is_empty() {
                if selected_index > 0 {
                    selected_index -= 1;
                } else {
                    selected_index = filtered.len().saturating_sub(1);
                }
            }
            update_selection_state(app, selected_index, input);
        }
        KeyCode::Down => {
            if !filtered.is_empty() {
                if selected_index + 1 < filtered.len() {
                    selected_index += 1;
                } else {
                    selected_index = 0;
                }
            }
            update_selection_state(app, selected_index, input);
        }
        KeyCode::Enter => {
            if let Some(&tpl_idx) = filtered.get(selected_index) {
                let _ = app.add_combatant_from_template(tpl_idx);
            } else {
                app.set_message("No matching template".to_string());
                app.input_mode = InputMode::Normal;
            }
        }
        KeyCode::Backspace => {
            input.pop();
            update_selection_state(app, selected_index, input);
        }
        KeyCode::Char(c) => {
            input.push(c);
            // Reset selection to 0 on new filter
            update_selection_state(app, 0, input);
        }
        _ => {}
    }
}

fn handle_save_encounter_mode(app: &mut App, key: KeyEvent, state: SaveEncounterState) {
    let mut input = state.input.clone();

    match key.code {
        KeyCode::Esc => app.cancel_input(),
        KeyCode::Enter => {
            if let Err(e) = app.complete_save_encounter(input) {
                app.set_message(e);
            }
        }
        KeyCode::Backspace => {
            input.pop();
            app.input_mode = InputMode::SavingEncounter(SaveEncounterState { input });
        }
        KeyCode::Char(c) if c.is_alphanumeric() || c == '_' || c == '-' => {
            input.push(c);
            app.input_mode = InputMode::SavingEncounter(SaveEncounterState { input });
        }
        _ => {}
    }
}

fn handle_load_encounter_mode(app: &mut App, key: KeyEvent, state: SelectionState) {
    let mut selected_index = state.selected_index;
    let mut input = state.input.clone();

    // Get list of saved encounters
    let saved_encounters = app.list_saved_encounters();

    // Filter by search input
    let filtered: Vec<&String> = saved_encounters
        .iter()
        .filter(|name| name.to_lowercase().contains(&input.to_lowercase()))
        .collect();

    if filtered.is_empty() && input.is_empty() {
        // No saved encounters exist
        app.set_message("No saved encounters found. Press Ctrl+S to save current encounter.".to_string());
        app.input_mode = InputMode::Normal;
        return;
    }

    match key.code {
        KeyCode::Esc => app.cancel_input(),
        KeyCode::Up => {
            if !filtered.is_empty() {
                selected_index = if selected_index > 0 {
                    selected_index - 1
                } else {
                    filtered.len() - 1
                };
                app.input_mode = InputMode::LoadingEncounter(SelectionState {
                    selected_index,
                    input,
                });
            }
        }
        KeyCode::Down => {
            if !filtered.is_empty() {
                selected_index = if selected_index < filtered.len() - 1 {
                    selected_index + 1
                } else {
                    0
                };
                app.input_mode = InputMode::LoadingEncounter(SelectionState {
                    selected_index,
                    input,
                });
            }
        }
        KeyCode::Enter => {
            if let Some(filename) = filtered.get(selected_index) {
                let _ = app.complete_load_encounter((*filename).clone());
            } else if !filtered.is_empty() {
                app.set_message("No encounter selected".to_string());
                app.input_mode = InputMode::Normal;
            }
        }
        KeyCode::Backspace => {
            input.pop();
            app.input_mode = InputMode::LoadingEncounter(SelectionState {
                selected_index: 0,
                input,
            });
        }
        KeyCode::Char(c) => {
            input.push(c);
            app.input_mode = InputMode::LoadingEncounter(SelectionState {
                selected_index: 0,
                input,
            });
        }
        _ => {}
    }
}

// Library input handlers

fn handle_save_library_mode(app: &mut App, key: KeyEvent, state: SaveLibraryState) {
    let mut new_state = state.clone();

    match key.code {
        KeyCode::Esc => app.cancel_input(),
        KeyCode::Enter => {
            // Move to next step or complete
            if new_state.step == 0 {
                // Name entered, move to description
                if new_state.name.is_empty() {
                    app.set_message("Name cannot be empty".to_string());
                    return;
                }
                new_state.step = 1;
                app.input_mode = InputMode::SavingLibrary(new_state);
            } else if new_state.step == 1 {
                // Description entered, move to difficulty
                if new_state.description.is_empty() {
                    app.set_message("Description cannot be empty".to_string());
                    return;
                }
                new_state.step = 2;
                app.input_mode = InputMode::SavingLibrary(new_state);
            } else if new_state.step == 2 {
                // Difficulty entered (optional), complete save
                if let Err(e) = app.complete_save_library(new_state) {
                    app.set_message(e);
                }
            }
        }
        KeyCode::Backspace => {
            match new_state.step {
                0 => { new_state.name.pop(); }
                1 => { new_state.description.pop(); }
                2 => { new_state.difficulty.pop(); }
                _ => {}
            };
            app.input_mode = InputMode::SavingLibrary(new_state);
        }
        KeyCode::Char(c) => {
            match new_state.step {
                0 if c.is_alphanumeric() || c == '_' || c == '-' => {
                    new_state.name.push(c);
                }
                1 => {
                    new_state.description.push(c);
                }
                2 => {
                    new_state.difficulty.push(c);
                }
                _ => {}
            }
            app.input_mode = InputMode::SavingLibrary(new_state);
        }
        _ => {}
    }
}

fn handle_loading_library_mode(app: &mut App, key: KeyEvent, state: SelectionState) {
    let mut selected_index = state.selected_index;
    let mut input = state.input.clone();

    // Get list of library templates
    let library_templates = app.list_library_templates();

    // Filter by search input
    let filtered: Vec<&String> = library_templates
        .iter()
        .filter(|name| name.to_lowercase().contains(&input.to_lowercase()))
        .collect();

    if filtered.is_empty() && input.is_empty() {
        // No library templates exist
        app.set_message(
            "No library templates found. Save current encounter to library first.".to_string(),
        );
        app.input_mode = InputMode::Normal;
        return;
    }

    match key.code {
        KeyCode::Esc => app.cancel_input(),
        KeyCode::Up => {
            if !filtered.is_empty() {
                selected_index = if selected_index > 0 {
                    selected_index - 1
                } else {
                    filtered.len() - 1
                };
                app.input_mode = InputMode::LoadingLibrary(SelectionState {
                    selected_index,
                    input,
                });
            }
        }
        KeyCode::Down => {
            if !filtered.is_empty() {
                selected_index = if selected_index < filtered.len() - 1 {
                    selected_index + 1
                } else {
                    0
                };
                app.input_mode = InputMode::LoadingLibrary(SelectionState {
                    selected_index,
                    input,
                });
            }
        }
        KeyCode::Enter => {
            if let Some(filename) = filtered.get(selected_index) {
                let _ = app.select_library_template((*filename).clone());
            } else if !filtered.is_empty() {
                app.set_message("No template selected".to_string());
                app.input_mode = InputMode::Normal;
            }
        }
        KeyCode::Backspace => {
            input.pop();
            app.input_mode = InputMode::LoadingLibrary(SelectionState {
                selected_index: 0,
                input,
            });
        }
        KeyCode::Char(c) => {
            input.push(c);
            app.input_mode = InputMode::LoadingLibrary(SelectionState {
                selected_index: 0,
                input,
            });
        }
        _ => {}
    }
}

fn handle_setting_library_initiatives_mode(
    app: &mut App,
    key: KeyEvent,
    state: LoadLibraryState,
) {
    let _current_combatant = &state.combatants_with_init[state.current_index].0;
    let mut input = state.combatants_with_init[state.current_index].1.clone();

    match key.code {
        KeyCode::Esc => app.cancel_input(),
        KeyCode::Enter => {
            if let Err(e) = app.complete_library_initiative(input) {
                app.set_message(e);
            }
        }
        KeyCode::Backspace => {
            input.pop();
            let mut new_state = state.clone();
            new_state.combatants_with_init[state.current_index].1 = input;
            app.input_mode = InputMode::SettingLibraryInitiatives(new_state);
        }
        KeyCode::Char(c) if c.is_numeric() || (c == '-' && input.is_empty()) => {
            input.push(c);
            let mut new_state = state.clone();
            new_state.combatants_with_init[state.current_index].1 = input;
            app.input_mode = InputMode::SettingLibraryInitiatives(new_state);
        }
        _ => {}
    }
}

fn handle_confirm_library_overwrite(app: &mut App, key: KeyEvent, state: SaveLibraryState) {
    match key.code {
        KeyCode::Char('y') | KeyCode::Char('Y') => {
            let _ = app.confirm_overwrite_library(state);
        }
        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
            app.cancel_library_overwrite();
        }
        _ => {}
    }
}

fn handle_confirm_library_load(app: &mut App, key: KeyEvent, filename: String) {
    match key.code {
        KeyCode::Char('y') | KeyCode::Char('Y') => {
            let _ = app.confirm_load_library(filename);
        }
        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
            app.cancel_library_load();
        }
        _ => {}
    }
}
