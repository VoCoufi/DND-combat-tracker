# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.6.0] - 2024-12-15

### Added
- Installation script for Linux/macOS (`install.sh`)
  - Auto-detects OS and architecture
  - SHA256 checksum verification
  - Installs to `~/.local/bin` by default
  - Supports `--uninstall` flag
- SHA256 checksums in GitHub releases

## [0.5.0] - 2024-12-09

### Added
- Complete status effect system with all 14 D&D 5e conditions
  - Blinded, Charmed, Deafened, Frightened, Grappled, Incapacitated, Invisible, Paralyzed, Petrified, Poisoned, Prone, Restrained, Stunned, Unconscious
  - Duration tracking (0 = indefinite, >0 = timed with auto-decrement)
  - Condition auto-effects with mechanical reminders
- Death saves tracking with critical success/failure
  - Natural 1 = 2 failures
  - Natural 20 = revive at 1 HP
  - Automatic tracking of 3 successes/failures
- Concentration tracking with automatic DC calculation
  - DC = max(10, damage/2) per D&D 5e rules
  - Automatic save prompts when taking damage
  - Auto-break on unconsciousness
- Save/Load encounter functionality
  - Complete combat state persistence (Ctrl+S to save, Ctrl+O to load)
  - Preserves HP, conditions, turn position, round number, combat log
  - JSON-based serialization
- Encounter Library for reusable encounter templates
  - Save encounters as pristine templates (full HP, no runtime state)
  - Load with fresh combatants and prompt for initiative
  - Includes name, description, and difficulty rating
- Combatant templates for quick NPC/monster addition
  - Save individual stat blocks for reuse
  - Filter by name when loading
  - Stored in `templates.json` (git-ignored)
- Combat log with 200-entry history
  - Tracks damage, healing, status effects, death saves, concentration
  - Persistent right-side panel
  - Included in save files
- Temporary HP mechanics (D&D 5e compliant)
  - Consumed before regular HP
  - Doesn't stack (higher replaces lower)
  - Doesn't heal regular HP
- Quick reference modal for condition descriptions
  - Press `?` to view all 14 conditions with full mechanical effects
  - Scrollable list with advantage/disadvantage rules
- HP visualization with color-coded bars
  - Green (>50%), Yellow (25-50%), Red (<25%), Gray (0%)
  - Visual health status at a glance
- Action Menu (press 'm') for quick access to combat actions
  - Deal Damage, Heal, Add Status Effect, Roll Death Save
  - Set Concentration, Clear Concentration/Status, Grant Temp HP
- Combatant Menu (press 'b') for combatant management
  - Add/Remove Combatants, Templates, Encounter Library

### Features
- Turn-based initiative tracking with automatic sorting (highest first)
- Round counter with automatic increment after last combatant
- Status effect duration management (auto-decrement on turn end)
- Modal state machine for clean UI workflows (28 input modes)
- JSON-based persistence for templates and encounters
- Comprehensive keyboard shortcuts for all actions
- Multi-step input flows for complex operations

### Technical
- 42 unit tests covering core mechanics
  - Combatant mechanics (HP, temp HP, death saves)
  - Status effect duration logic
  - Combat turn advancement
  - App-level integration workflows
  - Encounter library save/load
  - Concentration mechanics
- Stateless UI architecture with ratatui
- Clean separation of concerns: Models, Combat Logic, App State, UI
- Error handling with anyhow
- Logging with env_logger
- Cross-platform support (Linux, macOS, Windows)

### Architecture
- Modal state machine pattern with 28 InputMode variants
- Unidirectional data flow: Event → Handler → Mutate App → Render
- Stateless rendering (render.rs has no internal state)
- Event loop: render → poll → handle → update
- Domain models with business logic methods

[0.5.0]: https://github.com/VoCoufi/DND-combat-tracker/releases/tag/v0.5.0
