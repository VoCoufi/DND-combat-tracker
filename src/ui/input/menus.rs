use crate::app::{App, InputMode};
use crossterm::event::{KeyCode, KeyEvent};

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

pub(super) fn action_menu_items() -> Vec<(ActionMenuItem, &'static str)> {
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

pub(super) fn handle_action_menu_mode(app: &mut App, key: KeyEvent, selected_index: usize) {
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

pub(super) fn combatant_menu_items() -> Vec<(CombatantMenuItem, &'static str)> {
    vec![
        (CombatantMenuItem::AddCombatant, "Add Combatant"),
        (CombatantMenuItem::RemoveCombatant, "Remove Combatant"),
        (CombatantMenuItem::LoadTemplate, "Add from Template"),
        (CombatantMenuItem::SaveTemplate, "Save as Template"),
        (CombatantMenuItem::LoadLibrary, "Load Encounter Library"),
        (CombatantMenuItem::SaveLibrary, "Save to Encounter Library"),
    ]
}

pub(super) fn handle_combatant_menu_mode(app: &mut App, key: KeyEvent, selected_index: usize) {
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

pub(super) fn handle_quick_reference_mode(app: &mut App, key: KeyEvent, selected_index: usize) {
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
