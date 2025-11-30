use crate::app::{AddConcentrationState, App, InputMode, StatusSelectionState};
use crossterm::event::KeyEvent;

use super::combatant::{
    handle_add_combatant_mode, handle_removing_mode, handle_template_selection_mode,
};
use super::combat::{
    handle_add_concentration_mode, handle_concentration_check_mode, handle_selection_mode,
};
use super::menus::{handle_action_menu_mode, handle_combatant_menu_mode, handle_quick_reference_mode};
use super::normal::handle_normal_mode;
use super::persistence::{
    handle_confirm_library_load, handle_confirm_library_overwrite, handle_load_encounter_mode,
    handle_loading_library_mode, handle_save_encounter_mode, handle_save_library_mode,
    handle_setting_library_initiatives_mode,
};
use super::status::{
    handle_clear_choice_mode, handle_condition_selection_mode, handle_status_clear_selection,
    handle_status_selection_mode,
};

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
