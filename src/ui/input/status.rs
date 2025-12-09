#![allow(clippy::collapsible_else_if)]

use crate::app::{
    App, ClearAction, ConditionSelectionState, InputMode, SelectionState, StatusSelectionState,
};
use crossterm::event::{KeyCode, KeyEvent};

use super::combat::update_selection_state;

pub(super) fn handle_status_selection_mode(app: &mut App, key: KeyEvent) {
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

pub(super) fn handle_condition_selection_mode(
    app: &mut App,
    key: KeyEvent,
    state: ConditionSelectionState,
) {
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

pub(super) fn handle_status_clear_selection(
    app: &mut App,
    key: KeyEvent,
    state: StatusSelectionState,
) {
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

pub(super) fn handle_clear_choice_mode(app: &mut App, key: KeyEvent, choice: ClearAction) {
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
