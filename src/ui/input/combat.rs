use crate::app::{
    AddConcentrationState, App, ConcentrationCheckState, InputMode, SelectionState,
    StatusSelectionState,
};
use crossterm::event::{KeyCode, KeyEvent};

pub(super) fn handle_selection_mode<F>(app: &mut App, key: KeyEvent, on_confirm: F)
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

pub(super) fn update_selection_state(app: &mut App, index: usize, input: String) {
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

pub(super) fn handle_add_concentration_mode(
    app: &mut App,
    key: KeyEvent,
    state: AddConcentrationState,
) {
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

pub(super) fn handle_concentration_check_mode(
    app: &mut App,
    key: KeyEvent,
    state: ConcentrationCheckState,
) {
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
