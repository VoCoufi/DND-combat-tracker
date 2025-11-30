use crate::combat::CombatEncounter;
use crate::models::{CombatantTemplate, LogEntry};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

/// Complete saved encounter with all state and history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedEncounter {
    pub encounter: CombatEncounter,
    pub log: Vec<LogEntry>,
    pub saved_at: u64,
    pub version: String,
}

/// Lightweight combatant for encounter library (no runtime state or initiative)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LibraryCombatant {
    pub name: String,
    pub hp_max: i32,
    pub armor_class: i32,
    pub is_player: bool,
}

/// Encounter template for library with metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EncounterTemplate {
    pub name: String,
    pub description: String,
    pub difficulty: String, // Free text, can be empty
    pub combatants: Vec<LibraryCombatant>,
    pub created_at: u64,
}

// Template file operations

pub fn templates_path() -> &'static str {
    "templates.json"
}

pub fn load_templates() -> Result<Vec<CombatantTemplate>, String> {
    let path = templates_path();
    if !Path::new(path).exists() {
        return Ok(Vec::new());
    }

    let content = fs::read_to_string(path).map_err(|e| {
        log::error!("Failed to read templates from {}: {}", path, e);
        format!("Could not read templates file: {}", e)
    })?;

    serde_json::from_str(&content).map_err(|e| {
        log::error!("Failed to parse templates JSON from {}: {}", path, e);
        format!("Templates file is corrupted: {}", e)
    })
}

pub fn save_templates(templates: &[CombatantTemplate]) -> Result<(), String> {
    let path = templates_path();
    let json = serde_json::to_string_pretty(templates).map_err(|e| {
        log::error!("Failed to serialize templates to JSON: {}", e);
        e.to_string()
    })?;

    fs::write(path, json).map_err(|e| {
        log::error!("Failed to write templates to {}: {}", path, e);
        e.to_string()
    })
}

// Encounter save/load functions

pub fn encounters_dir() -> &'static str {
    "encounters"
}

pub fn ensure_encounters_dir() -> Result<(), String> {
    let dir = encounters_dir();
    if !Path::new(dir).exists() {
        fs::create_dir(dir).map_err(|e| {
            log::error!("Failed to create encounters directory: {}", e);
            format!("Could not create encounters directory: {}", e)
        })?;
    }
    Ok(())
}

pub fn save_encounter(
    encounter: &CombatEncounter,
    log: &[LogEntry],
    filename: &str,
) -> Result<(), String> {
    ensure_encounters_dir()?;

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    let saved = SavedEncounter {
        encounter: encounter.clone(),
        log: log.to_vec(),
        saved_at: timestamp,
        version: env!("CARGO_PKG_VERSION").to_string(),
    };

    let path = format!("{}/{}.json", encounters_dir(), filename);
    let json = serde_json::to_string_pretty(&saved).map_err(|e| {
        log::error!("Failed to serialize encounter to JSON: {}", e);
        format!("Could not save encounter: {}", e)
    })?;

    fs::write(&path, json).map_err(|e| {
        log::error!("Failed to write encounter to {}: {}", path, e);
        format!("Could not write encounter file: {}", e)
    })?;

    Ok(())
}

pub fn load_encounter(filename: &str) -> Result<SavedEncounter, String> {
    let path = format!("{}/{}.json", encounters_dir(), filename);

    if !Path::new(&path).exists() {
        return Err(format!("Encounter file not found: {}", filename));
    }

    let content = fs::read_to_string(&path).map_err(|e| {
        log::error!("Failed to read encounter from {}: {}", path, e);
        format!("Could not read encounter file: {}", e)
    })?;

    serde_json::from_str(&content).map_err(|e| {
        log::error!("Failed to parse encounter JSON from {}: {}", path, e);
        format!("Encounter file is corrupted: {}", e)
    })
}

pub fn list_encounter_files() -> Result<Vec<String>, String> {
    let dir = encounters_dir();

    if !Path::new(dir).exists() {
        return Ok(Vec::new());
    }

    let entries = fs::read_dir(dir).map_err(|e| {
        log::error!("Failed to read encounters directory: {}", e);
        format!("Could not read encounters directory: {}", e)
    })?;

    let mut files = Vec::new();
    for entry in entries {
        if let Ok(entry) = entry {
            if let Some(filename) = entry.file_name().to_str() {
                if filename.ends_with(".json") {
                    // Remove .json extension
                    let name = filename.trim_end_matches(".json").to_string();
                    files.push(name);
                }
            }
        }
    }

    files.sort();
    Ok(files)
}

// Library file operations

pub fn library_dir() -> &'static str {
    "library"
}

pub fn ensure_library_dir() -> Result<(), String> {
    let dir = library_dir();
    if !Path::new(dir).exists() {
        fs::create_dir(dir).map_err(|e| {
            log::error!("Failed to create library directory: {}", e);
            format!("Could not create library directory: {}", e)
        })?;
    }
    Ok(())
}

pub fn save_library_template(template: &EncounterTemplate, filename: &str) -> Result<(), String> {
    ensure_library_dir()?;

    let path = format!("{}/{}.json", library_dir(), filename);
    let json = serde_json::to_string_pretty(template).map_err(|e| {
        log::error!("Failed to serialize library template to JSON: {}", e);
        format!("Could not save library template: {}", e)
    })?;

    fs::write(&path, json).map_err(|e| {
        log::error!("Failed to write library template to {}: {}", path, e);
        format!("Could not write library file: {}", e)
    })?;

    Ok(())
}

pub fn load_library_template(filename: &str) -> Result<EncounterTemplate, String> {
    let path = format!("{}/{}.json", library_dir(), filename);

    if !Path::new(&path).exists() {
        return Err(format!("Library template not found: {}", filename));
    }

    let content = fs::read_to_string(&path).map_err(|e| {
        log::error!("Failed to read library template from {}: {}", path, e);
        format!("Could not read library file: {}", e)
    })?;

    serde_json::from_str(&content).map_err(|e| {
        log::error!("Failed to parse library template JSON from {}: {}", path, e);
        format!("Library file is corrupted: {}", e)
    })
}

pub fn list_library_files() -> Result<Vec<String>, String> {
    let dir = library_dir();

    if !Path::new(dir).exists() {
        return Ok(Vec::new());
    }

    let entries = fs::read_dir(dir).map_err(|e| {
        log::error!("Failed to read library directory: {}", e);
        format!("Could not read library directory: {}", e)
    })?;

    let mut files = Vec::new();
    for entry in entries {
        if let Ok(entry) = entry {
            if let Some(filename) = entry.file_name().to_str() {
                if filename.ends_with(".json") {
                    // Remove .json extension
                    let name = filename.trim_end_matches(".json").to_string();
                    files.push(name);
                }
            }
        }
    }

    files.sort();
    Ok(files)
}

pub fn library_template_exists(filename: &str) -> bool {
    let path = format!("{}/{}.json", library_dir(), filename);
    Path::new(&path).exists()
}
