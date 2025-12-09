# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A terminal-based combat encounter tracker for Dungeons & Dragons 5th Edition built with Rust. Uses `ratatui` for the TUI framework and `crossterm` for terminal manipulation.

## Development Commands

### Build & Run
```bash
cargo build --release      # Production build
cargo run --release        # Run the application
cargo run                  # Run in debug mode
```

### Testing
```bash
cargo test                 # Run all tests
cargo test --lib           # Run library tests only
cargo test combatant       # Run specific test module
cargo test -- --nocapture  # Show println! output in tests
```

### Code Quality
```bash
cargo fmt                  # Format code
cargo clippy               # Run linter
```

## Architecture Overview

### Core Structure

The application follows a **modal state machine pattern** with clean separation of concerns:

- **Models** (`src/models/`): Pure data structures with business logic methods
- **Combat Logic** (`src/combat.rs`): Turn management, initiative sorting, round tracking
- **App State** (`src/app.rs`): Orchestrates models, handles complex workflows
- **UI Rendering** (`src/ui/render.rs`): Stateless rendering functions (954 LOC)
- **Input Handling** (`src/ui/input.rs`): Stateless event handlers (700 LOC)

**Key Principle**: The UI layer is completely stateless - it only reads from `App` and calls methods on `App`. All state lives in the model layer.

### Application State (`src/app.rs`)

Central state container with modal state machine:

```rust
pub struct App {
    pub encounter: CombatEncounter,        // Active combat state
    pub input_mode: InputMode,             // 20+ modal variants
    pub should_quit: bool,                 // Lifecycle flag
    pub message: Option<String>,           // User feedback
    pub templates: Vec<CombatantTemplate>, // Saved templates
    pub log: Vec<LogEntry>,                // Combat log (max 200)
}
```

The `InputMode` enum has 20+ variants representing different UI modals (adding combatants, dealing damage, selecting conditions, etc.). Each mode has its own dedicated input handler.

### Event Loop Pattern (`src/main.rs`)

Classic game loop architecture:
1. **Render** current state (`render()`)
2. **Poll** for events (100ms timeout)
3. **Handle** keyboard input (`handle_key_event()`)
4. **Update** state (mutate `App`)
5. **Repeat**

### Key Domain Models

**Combatant** (`src/models/combatant.rs`):
- Core entity representing creatures in combat
- `take_damage()`: Consumes temp HP first, then regular HP
- `apply_death_save_roll()`: Handles critical successes (revive at 1 HP) and failures (2 fails)
- `decrement_status_effects()`: Called on turn end to tick down durations
- `grant_temp_hp()`: Only replaces if new amount is higher

**CombatEncounter** (`src/combat.rs`):
- Automatically sorts by initiative on combatant addition
- `next_turn()`: Decrements status effects, advances index, increments round
- `previous_turn()`: Allows undo of turn advancement

**StatusEffect** (`src/models/status.rs`):
- Implements all 14 D&D 5e standard conditions
- Duration: `> 0` = timed (ticks down), `0` = indefinite, `< 0` = expired

### Critical Mechanic: Damage + Concentration

When dealing damage, the app automatically checks for concentration:

```rust
// src/app.rs:complete_deal_damage()
let had_concentration = combatant.concentration.clone();
combatant.take_damage(damage);

if combatant.is_unconscious() {
    combatant.clear_concentration();  // Auto-break
} else if let Some(info) = had_concentration {
    let dc = std::cmp::max(10, damage / 2);  // 5e DC rule
    // Transitions to ConcentrationCheck mode
}
```

### Template System

Templates are saved to `templates.json` (git-ignored) in the project root. Uses `serde_json` for serialization.

**Workflow**: Save combatant base stats → Load with filterable picker → Only prompts for initiative

Location: `src/app.rs:692-711`, `src/models/combatant_template.rs`

### Input Handling Pattern

Mode-based dispatch with callback closures to reduce duplication:

```rust
// src/ui/input.rs
match app.input_mode.clone() {
    InputMode::Normal => handle_normal_mode(app, key),
    InputMode::DealingDamage(_) => handle_selection_mode(app, key, |app, idx, input| {
        // Closure handles confirmation logic
    }),
    // ... 20+ modes
}
```

**Multi-step workflows** use step-based state (e.g., `AddCombatantState` with step 0-4).

### Rendering Architecture

Uses **ratatui** widgets for declarative UI. Main render flow:

1. Define layout (header, content, commands, message)
2. Render each section (combatants, log, commands)
3. Render modal overlays based on `input_mode`

**Visual Features**: Color-coded HP bars (Green >50%, Yellow 25-50%, Red <25%, Gray 0%), status badges with durations, concentration indicators, death save tracking

## Adding New Features

### Adding a New Combat Action

1. Add variant to `InputMode` enum in `src/app.rs`
2. Add start method: `pub fn start_new_action(&mut self)`
3. Add completion method: `pub fn complete_new_action(&mut self, ...)`
4. Add handler in `src/ui/input.rs`
5. Add modal renderer in `src/ui/render.rs`
6. Add keyboard shortcut in `handle_normal_mode()` or menu
7. Add tests in `src/app.rs`

### Modifying Combatant Data

1. Update `Combatant` struct in `src/models/combatant.rs`
2. Add methods to manipulate new field
3. Update `render_combatants()` in `src/ui/render.rs` to display
4. Add unit tests in same file (`#[cfg(test)]` section)

### Adding a New Condition

1. Add variant to `ConditionType` enum in `src/models/status.rs`
2. Add description in `description()` method
3. Update `all()` method to include in picker

## Testing Strategy

Tests are co-located with modules using `#[cfg(test)]`. Key test files:

- `src/app.rs`: App-level integration tests
- `src/combat.rs`: Turn management tests
- `src/models/combatant.rs`: Combatant mechanics (HP, temp HP, death saves)
- `src/models/status.rs`: Status effect logic

**Pattern**: Create test helpers like `player(name, hp)` for readability.

## Important Notes

### D&D 5e Rules Implemented

- **Concentration DC**: `max(10, damage/2)`
- **Death Saves**: 3 successes = stabilized, 3 failures = dead, nat 20 = revive at 1 HP, nat 1 = 2 failures
- **Temp HP**: Consumed before regular HP, doesn't stack (higher replaces lower), doesn't heal regular HP
- **Status Effects**: Duration 0 = indefinite (manual removal only)

### State Management

- Combat log capped at 200 entries to prevent unbounded growth
- Code frequently clones state to avoid borrow checker issues (acceptable given small state size)
- Templates save immediately to disk; encounters do NOT persist (roadmap item #4)

### File Locations for Common Tasks

- **Combat mechanics**: `src/combat.rs`, `src/models/combatant.rs`
- **Turn order logic**: `src/combat.rs:next_turn()`, `src/app.rs`
- **Main event loop**: `src/main.rs:53-69`
- **Modal state machine**: `src/app.rs` (InputMode enum)
- **All input handling**: `src/ui/input.rs`
- **All rendering**: `src/ui/render.rs`

### Architectural Decisions

- **Modal state machine** over screen-based navigation keeps context visible
- **Unidirectional data flow**: App → Render; Input → Mutate App → Re-render
- **No database**: Templates persist, encounters don't (by design, for now)
- **Stateless UI**: Rendering and input handling have no internal state

## Dependencies

- `ratatui` 0.29: Terminal UI framework (declarative widgets)
- `crossterm` 0.28: Cross-platform terminal control (events, raw mode)
- `serde` + `serde_json` 1.0: JSON serialization for templates
- `anyhow` 1.0: Error handling with context

## Current Roadmap Status

**Completed Features**: Status effect selection, death saves, concentration tracking, templates, HP visualization, combat log, temp HP, quick reference, unit testing

**High-Priority Planned**: Save/load encounters (#4), bulk add combatants (#5), undo last action (#6)
