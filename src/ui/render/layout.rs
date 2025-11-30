use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::{App, InputMode};

use super::{
    encounter::render_combatants,
    log::render_log,
    menus::{render_action_menu, render_combatant_menu, render_quick_reference},
    modals::{
        render_add_combatant_modal, render_add_concentration_modal, render_clear_choice_modal,
        render_concentration_check, render_condition_selection, render_confirm_load_modal,
        render_confirm_overwrite_modal, render_library_initiative_modal,
        render_load_encounter_modal, render_loading_library_modal, render_save_encounter_modal,
        render_save_library_modal, render_selection_modal, render_status_clear_modal,
        render_template_selection_modal,
    },
};

pub fn render(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Main content
            Constraint::Length(3), // Commands
            Constraint::Length(2), // Message
        ])
        .split(f.area());

    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(40), Constraint::Length(40)])
        .split(chunks[1]);

    // Render header
    render_header(f, chunks[0], app);

    // Render main content
    render_combatants(f, content_chunks[0], app);
    render_log(f, content_chunks[1], app);

    // Render commands
    render_commands(f, chunks[2], app);

    // Render message
    if let Some(ref msg) = app.message {
        render_message(f, chunks[3], msg);
    }

    // Render modal if needed
    match &app.input_mode {
        InputMode::AddingCombatant(state) => render_add_combatant_modal(f, state),
        InputMode::DealingDamage(state) => {
            render_selection_modal(f, state, "Deal Damage", "Enter damage amount:", app)
        }
        InputMode::Healing(state) => {
            render_selection_modal(f, state, "Heal", "Enter heal amount:", app)
        }
        InputMode::AddingStatus(state) => {
            render_selection_modal(f, state, "Add Status Effect", "Select combatant:", app)
        }
        InputMode::SelectingCondition(state) => render_condition_selection(f, state, app),
        InputMode::RollingDeathSave(state) => {
            render_selection_modal(f, state, "Death Save", "Enter d20 roll:", app)
        }
        InputMode::ConcentrationTarget(state) => {
            render_selection_modal(f, state, "Set Concentration", "Select combatant:", app)
        }
        InputMode::ApplyingConcentration(state) => render_add_concentration_modal(f, state, app),
        InputMode::ConcentrationCheck(state) => render_concentration_check(f, state, app),
        InputMode::ClearActionSelection(choice) => render_clear_choice_modal(f, choice),
        InputMode::ClearingConcentration(state) => {
            render_selection_modal(f, state, "Clear Concentration", "Select combatant:", app)
        }
        InputMode::ClearingStatus(state) => {
            render_selection_modal(f, state, "Clear Status Effects", "Select combatant:", app)
        }
        InputMode::SelectingStatusToClear(state) => render_status_clear_modal(f, state, app),
        InputMode::SelectingTemplate(state) => render_template_selection_modal(f, state, app),
        InputMode::SavingTemplate(state) => {
            render_selection_modal(f, state, "Save Template", "Select combatant to save:", app)
        }
        InputMode::GrantingTempHp(state) => {
            render_selection_modal(f, state, "Grant Temp HP", "Enter temp HP amount:", app)
        }
        InputMode::ActionMenu(selected) => render_action_menu(f, *selected),
        InputMode::CombatantMenu(selected) => render_combatant_menu(f, *selected),
        InputMode::QuickReference(selected) => render_quick_reference(f, *selected, app),
        InputMode::Removing(state) => render_selection_modal(
            f,
            state,
            "Remove Combatant",
            "Select combatant to remove:",
            app,
        ),
        InputMode::SavingEncounter(state) => render_save_encounter_modal(f, state),
        InputMode::LoadingEncounter(state) => render_load_encounter_modal(f, state, app),
        InputMode::SavingLibrary(state) => render_save_library_modal(f, state),
        InputMode::LoadingLibrary(state) => render_loading_library_modal(f, state, app),
        InputMode::SettingLibraryInitiatives(state) => {
            render_library_initiative_modal(f, state, app)
        }
        InputMode::ConfirmingLibraryOverwrite(_) => render_confirm_overwrite_modal(f),
        InputMode::ConfirmingLibraryLoad(_) => render_confirm_load_modal(f),
        InputMode::Normal => {}
    }
}

fn render_header(f: &mut Frame, area: Rect, app: &App) {
    let title = format!(
        " D&D 5e Combat Tracker | Round: {} ",
        app.encounter.round_number
    );
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Cyan));
    f.render_widget(block, area);
}

fn render_commands(f: &mut Frame, area: Rect, app: &App) {
    let commands = match app.input_mode {
        InputMode::Normal => {
            "[n] Next  [m] Action  [b] Combatant  [Ctrl+S] Save  [Ctrl+O] Load  [?] Ref  [q] Quit"
        }
        _ => "[Esc] Cancel",
    };

    let block = Block::default()
        .title(" Commands ")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White));

    let paragraph = Paragraph::new(commands)
        .block(block)
        .alignment(Alignment::Center);

    f.render_widget(paragraph, area);
}

fn render_message(f: &mut Frame, area: Rect, msg: &str) {
    let paragraph = Paragraph::new(msg)
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Center);

    f.render_widget(paragraph, area);
}
