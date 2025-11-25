use crate::app::{
    AddConcentrationState, App, ClearAction, ConcentrationCheckState, ConditionSelectionState,
    InputMode, SelectionState,
};
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_key_event(app: &mut App, key: KeyEvent) {
    match app.input_mode.clone() {
        InputMode::Normal => handle_normal_mode(app, key),
        InputMode::AddingCombatant(_) => handle_add_combatant_mode(app, key),
        InputMode::DealingDamage(_) => handle_selection_mode(app, key, |app, idx, input| {
            if let Ok(damage) = input.parse::<i32>() {
                let _ = app.complete_deal_damage(idx, damage);
            } else {
                app.set_message("Invalid damage value!".to_string());
                app.input_mode = InputMode::Normal;
            }
        }),
        InputMode::Healing(_) => handle_selection_mode(app, key, |app, idx, input| {
            if let Ok(amount) = input.parse::<i32>() {
                let _ = app.complete_heal(idx, amount);
            } else {
                app.set_message("Invalid heal value!".to_string());
                app.input_mode = InputMode::Normal;
            }
        }),
        InputMode::AddingStatus(_) => handle_status_selection_mode(app, key),
        InputMode::SelectingCondition(state) => handle_condition_selection_mode(app, key, state),
        InputMode::RollingDeathSave(_) => handle_selection_mode(app, key, |app, idx, input| {
            if let Ok(roll) = input.parse::<i32>() {
                let _ = app.complete_death_save_roll(idx, roll);
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
            let _ = app.complete_clear_concentration(idx);
        }),
        InputMode::ClearActionSelection(choice) => handle_clear_choice_mode(app, key, choice),
        InputMode::ClearingStatus(_) => handle_selection_mode(app, key, |app, idx, _| {
            let _ = app.complete_clear_status_effects(idx);
        }),
        InputMode::Removing(_) => handle_removing_mode(app, key),
    }
}

fn handle_normal_mode(app: &mut App, key: KeyEvent) {
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
        KeyCode::Char('r') => app.start_removing(),
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

    app.input_mode = match app.input_mode {
        InputMode::DealingDamage(_) => InputMode::DealingDamage(new_state),
        InputMode::Healing(_) => InputMode::Healing(new_state),
        InputMode::AddingStatus(_) => InputMode::AddingStatus(new_state),
        InputMode::RollingDeathSave(_) => InputMode::RollingDeathSave(new_state),
        InputMode::ConcentrationTarget(_) => InputMode::ConcentrationTarget(new_state),
        InputMode::ClearingConcentration(_) => InputMode::ClearingConcentration(new_state),
        InputMode::ClearingStatus(_) => InputMode::ClearingStatus(new_state),
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
            let _ = app.complete_add_status(combatant_index, condition, duration);
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
                let _ = app.complete_concentration_check(state.clone(), total);
            } else {
                app.set_message("Invalid roll total".to_string());
                app.input_mode = InputMode::Normal;
            }
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
                let _ = app.complete_remove(selected_index);
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
