use crate::app::{App, LoadLibraryState, SaveEncounterState, SaveLibraryState, SelectionState};
use crossterm::event::{KeyCode, KeyEvent};

pub(super) fn handle_save_encounter_mode(app: &mut App, key: KeyEvent, state: SaveEncounterState) {
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

pub(super) fn handle_load_encounter_mode(app: &mut App, key: KeyEvent, state: SelectionState) {
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

pub(super) fn handle_save_library_mode(app: &mut App, key: KeyEvent, state: SaveLibraryState) {
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

pub(super) fn handle_loading_library_mode(app: &mut App, key: KeyEvent, state: SelectionState) {
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

pub(super) fn handle_setting_library_initiatives_mode(
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

pub(super) fn handle_confirm_library_overwrite(
    app: &mut App,
    key: KeyEvent,
    state: SaveLibraryState,
) {
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

pub(super) fn handle_confirm_library_load(app: &mut App, key: KeyEvent, filename: String) {
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

use crate::app::InputMode;
