use crate::app::{App, InputMode, SelectionState};
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_key_event(app: &mut App, key: KeyEvent) {
    match &app.input_mode {
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
        InputMode::SelectingCondition(combatant_index) => {
            handle_condition_selection_mode(app, key, *combatant_index)
        }
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
    let (selected_index, mut input) = match &app.input_mode {
        InputMode::DealingDamage(state) => (state.selected_index, state.input.clone()),
        InputMode::Healing(state) => (state.selected_index, state.input.clone()),
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
            if !input.is_empty() {
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
                app.input_mode = InputMode::SelectingCondition(selected_index);
            }
            _ => {}
        }
    }
}

fn handle_condition_selection_mode(app: &mut App, key: KeyEvent, _combatant_index: usize) {
    match key.code {
        KeyCode::Esc => app.cancel_input(),
        KeyCode::Char(_c) => {
            // For now, just cancel - this is a simplified implementation
            // A full implementation would collect input for condition selection
            app.cancel_input();
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
