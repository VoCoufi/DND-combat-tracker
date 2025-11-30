use super::persistence::{EncounterTemplate, LibraryCombatant};

#[derive(Debug, Clone, PartialEq)]
pub enum InputMode {
    Normal,
    AddingCombatant(AddCombatantState),
    DealingDamage(SelectionState),
    Healing(SelectionState),
    AddingStatus(SelectionState),
    SelectingCondition(ConditionSelectionState),
    RollingDeathSave(SelectionState),
    ConcentrationTarget(SelectionState),
    ApplyingConcentration(AddConcentrationState),
    ConcentrationCheck(ConcentrationCheckState),
    ClearingConcentration(SelectionState),
    ClearActionSelection(ClearAction),
    ClearingStatus(SelectionState),
    SelectingStatusToClear(StatusSelectionState),
    SelectingTemplate(SelectionState),
    SavingTemplate(SelectionState),
    ActionMenu(usize),
    CombatantMenu(usize),
    GrantingTempHp(SelectionState),
    QuickReference(usize),
    Removing(SelectionState),
    SavingEncounter(SaveEncounterState),
    LoadingEncounter(SelectionState),
    SavingLibrary(SaveLibraryState),
    LoadingLibrary(SelectionState),
    SettingLibraryInitiatives(LoadLibraryState),
    ConfirmingLibraryOverwrite(SaveLibraryState),
    ConfirmingLibraryLoad(String), // stores template name
}

#[derive(Debug, Clone, PartialEq)]
pub struct AddCombatantState {
    pub step: usize, // 0: name, 1: initiative, 2: hp, 3: ac, 4: is_player
    pub name: String,
    pub initiative: String,
    pub hp: String,
    pub ac: String,
    pub is_player: String,
}

impl Default for AddCombatantState {
    fn default() -> Self {
        Self {
            step: 0,
            name: String::new(),
            initiative: String::new(),
            hp: String::new(),
            ac: String::new(),
            is_player: String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelectionState {
    pub selected_index: usize,
    pub input: String,
}

impl Default for SelectionState {
    fn default() -> Self {
        Self {
            selected_index: 0,
            input: String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConditionSelectionState {
    pub combatant_index: usize,
    pub input: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AddConcentrationState {
    pub combatant_index: usize,
    pub step: usize, // 0: spell name, 1: con mod
    pub spell_name: String,
    pub con_mod: String,
}

impl Default for AddConcentrationState {
    fn default() -> Self {
        Self {
            combatant_index: 0,
            step: 0,
            spell_name: String::new(),
            con_mod: String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConcentrationCheckState {
    pub combatant_index: usize,
    pub dc: i32,
    pub input: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClearAction {
    Concentration,
    StatusEffects,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StatusSelectionState {
    pub combatant_index: usize,
    pub selected_status_index: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SaveEncounterState {
    pub input: String,  // filename
}

impl Default for SaveEncounterState {
    fn default() -> Self {
        Self {
            input: String::new(),
        }
    }
}

/// State for saving encounter to library (3-step process)
#[derive(Debug, Clone, PartialEq)]
pub struct SaveLibraryState {
    pub step: usize, // 0: name, 1: description, 2: difficulty
    pub name: String,
    pub description: String,
    pub difficulty: String,
}

impl Default for SaveLibraryState {
    fn default() -> Self {
        Self {
            step: 0,
            name: String::new(),
            description: String::new(),
            difficulty: String::new(),
        }
    }
}

/// State for loading library and setting initiatives
#[derive(Debug, Clone, PartialEq)]
pub struct LoadLibraryState {
    pub template: EncounterTemplate,
    pub combatants_with_init: Vec<(LibraryCombatant, String)>, // (combatant, initiative_input)
    pub current_index: usize, // Which combatant we're setting initiative for
}
