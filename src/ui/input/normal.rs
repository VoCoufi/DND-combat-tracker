use crate::app::{App, InputMode};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub(super) fn handle_normal_mode(app: &mut App, key: KeyEvent) {
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
