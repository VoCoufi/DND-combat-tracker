use crate::app::{App, InputMode, SelectionState};
use crossterm::event::{KeyCode, KeyEvent};

use super::combat::update_selection_state;

pub(super) fn handle_add_combatant_mode(app: &mut App, key: KeyEvent) {
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

pub(super) fn handle_removing_mode(app: &mut App, key: KeyEvent) {
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

pub(super) fn handle_template_selection_mode(app: &mut App, key: KeyEvent, state: SelectionState) {
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
