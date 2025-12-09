# D&D 5e Combat Tracker

![CI](https://github.com/VoCoufi/DND-combat-tracker/workflows/CI/badge.svg)
![Release](https://github.com/VoCoufi/DND-combat-tracker/workflows/Release/badge.svg)
![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)
![Version](https://img.shields.io/badge/version-0.5.0-green)

A feature-rich, terminal-based combat encounter tracker for Dungeons & Dragons 5th Edition, built with Rust. Manage initiative, HP, conditions, death saves, concentration, and more with an intuitive keyboard-driven interface powered by `ratatui`.

## Installation

### Option 1: Download Prebuilt Binary (Recommended)

The easiest way to get started is to download a prebuilt binary for your operating system:

1. Visit the [Releases page](https://github.com/VoCoufi/DND-combat-tracker/releases)
2. Download the appropriate binary for your OS:
   - **Linux**: `dnd-combat-tracker-linux-x86_64.tar.gz`
   - **macOS (Intel)**: `dnd-combat-tracker-macos-x86_64.tar.gz`
   - **macOS (Apple Silicon)**: `dnd-combat-tracker-macos-aarch64.tar.gz`
   - **Windows**: `dnd-combat-tracker-windows-x86_64.zip`

3. Extract and run:

   **Linux/macOS:**
   ```bash
   tar -xzf dnd-combat-tracker-*.tar.gz
   ./dnd-combat-tracker
   ```

   **Windows:**
   - Extract the ZIP file
   - Run `dnd-combat-tracker.exe`

### Option 2: Build from Source

If you have Rust installed, you can build from source:

**Prerequisites:**
- **Rust**: 1.90.0 or later ([install here](https://rustup.rs/))
- **Terminal**: Any terminal supporting ANSI colors (most modern terminals)

**Build and run:**
```bash
git clone https://github.com/VoCoufi/DND-combat-tracker.git
cd DND-combat-tracker
cargo build --release
./target/release/dnd-combat-tracker
```

### Option 3: Install with Cargo

If you have Rust installed, you can install directly from the repository:

```bash
cargo install --git https://github.com/VoCoufi/DND-combat-tracker
dnd-combat-tracker
```

## Quick Start

Here's a typical combat session workflow:

1. **Start the application**: `cargo run --release`

2. **Add player characters**: Press `a` or `b` → "Add Combatant"
   ```
   Name: Thorin
   Initiative: 18
   HP: 35
   AC: 16
   Is Player? y
   ```

3. **Add enemies**: Press `a` again
   ```
   Name: Goblin 1
   Initiative: 15
   HP: 7
   AC: 13
   Is Player? n
   ```

4. **Combat automatically sorts** by initiative (highest first)

5. **Advance turns**: Press `n` to move to next turn
   - Status effects automatically decrement
   - Round counter increments after last combatant

6. **Take actions**: Press `m` for Action Menu
   - Deal Damage → Select target → Enter amount
   - Heal → Select target → Enter amount
   - Add Status Effect → Select target → Choose condition → Set duration

7. **Save encounter**: Press `Ctrl+S` to save current state

8. **Quit**: Press `q` when combat ends

## Features

### Combat Management
- **Initiative & Turn Order**: Automatic sorting by initiative with round tracking
- **Turn Advancement**: Progress through turns with automatic status effect duration tracking
- **Combat Log**: Persistent right-side panel tracking all combat events (200 entry history)

### Combatant Management
- **Add/Remove Combatants**: Full workflow for adding creatures with name, initiative, HP, AC, and player/NPC designation
- **HP & Temporary HP**: Complete D&D 5e temp HP mechanics (consumed first, higher replaces lower, doesn't heal regular HP)
- **Damage & Healing**: Deal damage with automatic concentration checks; heal with simple prompts
- **Color-Coded HP Bars**: Visual feedback (Green >50%, Yellow 25-50%, Red <25%, Gray 0%)

### D&D 5e Mechanics
- **Death Saves**: Full implementation with nat 1 (2 failures), nat 20 (revive at 1 HP), and 3 success/failure tracking
- **Concentration**: Automatic concentration checks on damage with proper DC calculation: `max(10, damage/2)`
- **Status Effects & Conditions**: All 14 standard D&D 5e conditions with duration tracking
  - Blinded, Charmed, Deafened, Frightened, Grappled, Incapacitated, Invisible, Paralyzed, Petrified, Poisoned, Prone, Restrained, Stunned, Unconscious
  - Duration 0 = indefinite, >0 = timed (auto-decrements on turn end)
- **Condition Quick Reference**: Press `?` for scrollable modal with full mechanical descriptions

### Persistence & Templates
- **Save/Load Encounters**: Save complete combat state to file (`Ctrl+S`) and resume later (`Ctrl+O`)
  - Preserves HP, conditions, turn position, round number, combat log, and all state
- **Encounter Library**: Build reusable encounter templates with name, description, and difficulty rating
  - Save entire encounters as pristine templates (no runtime state)
  - Load with fresh HP and prompt for new initiative rolls
  - Perfect for DMs prepping recurring encounters
- **Combatant Templates**: Save individual combatant stat blocks for quick reuse
  - Stored in `templates.json` (git-ignored)
  - Filter by name when loading

### User Interface
- **Modal State Machine**: Menu-driven interface with 28 specialized input modes
- **Action Menu**: Quick access to all combat actions via `m` key
- **Combatant Menu**: Centralized combatant management via `b` key
- **Stateless Rendering**: Responsive TUI with clear visual hierarchy and color coding
- **Keyboard-First Design**: All features accessible via intuitive keyboard shortcuts

## Usage Guide

### Keyboard Reference

#### Direct Shortcuts (Normal Mode)

| Key | Action | Description |
|-----|--------|-------------|
| `n` | Next Turn | Advance to next combatant in initiative order |
| `a` | Add Combatant | Start workflow to add new combatant |
| `d` | Deal Damage | Deal damage to selected combatant |
| `h` | Heal | Restore HP to selected combatant |
| `s` | Add Status | Apply status effect/condition |
| `v` | Death Save | Record death saving throw result |
| `c` | Concentration | Set concentration spell on combatant |
| `x` | Clear | Clear concentration or status effects |
| `m` | Action Menu | Open menu with all combat actions |
| `b` | Combatant Menu | Open menu for combatant management |
| `?` | Quick Reference | View D&D 5e condition descriptions |
| `q` | Quit | Exit application |

#### Control Shortcuts

| Key | Action | Description |
|-----|--------|-------------|
| `Ctrl+S` | Save Encounter | Save current combat state to file |
| `Ctrl+O` | Load Encounter | Load saved encounter from file |

#### Modal Navigation

| Key | Action | Context |
|-----|--------|---------|
| `Esc` | Cancel | Exit current modal/menu without changes |
| `Enter` | Confirm | Proceed to next step or confirm input |
| `↑` / `↓` | Navigate | Move selection up/down in lists |
| `Backspace` | Delete | Remove last character in text input |

### Action Menu (`m` key)

The Action Menu provides quick access to all combat actions:

1. **Deal Damage**: Select target, enter amount, automatic concentration check if applicable
2. **Heal**: Select target, enter HP to restore
3. **Add Status Effect**: Select target, choose condition, set duration
4. **Roll Death Save**: Record nat 1/20 and success/failure for unconscious creatures
5. **Set Concentration**: Mark combatant as concentrating on a spell
6. **Clear Concentration/Status**: Remove concentration or specific status effects
7. **Grant Temp HP**: Give temporary hit points (higher replaces lower)

### Combatant Menu (`b` key)

The Combatant Menu centralizes all combatant management:

1. **Add Combatant**: Multi-step workflow for new combatant (name, init, HP, AC, player flag)
2. **Remove Combatant**: Delete combatant from encounter
3. **Add from Template**: Load saved combatant template, prompt for initiative only
4. **Save as Template**: Save current combatant's base stats for reuse
5. **Load Encounter Library**: Load complete encounter template with fresh combatants
6. **Save to Encounter Library**: Save current encounter as reusable template

### Feature Deep-Dives

#### Save/Load Encounters

**Save Encounter** (`Ctrl+S`):
- Saves complete combat state including:
  - All combatant data (HP, AC, initiative, player flag)
  - Current HP and temp HP values
  - Active status effects and durations
  - Concentration state
  - Current turn index and round number
  - Full combat log history
- Prompts for filename
- Stored as JSON in project directory

**Load Encounter** (`Ctrl+O`):
- Prompts for filename
- Restores exact combat state from file
- Resume mid-combat or replay previous encounters
- Validates file exists before loading

**Use Case**: Save at end of session when combat isn't finished, load next session to resume exactly where you left off.

#### Encounter Library

**Save to Library**:
1. Press `b` → "Save to Encounter Library"
2. Enter encounter name (e.g., "Goblin Ambush")
3. Enter description (e.g., "3 goblins, 1 hobgoblin")
4. Set difficulty (Easy/Medium/Hard/Deadly)
5. Saves as pristine template (full HP, no conditions, no runtime state)

**Load from Library**:
1. Press `b` → "Load Encounter Library"
2. Select encounter from list
3. Confirm load (replaces current encounter)
4. App prompts for initiative for each combatant
5. Fresh combat starts with full HP

**Difference from Save/Load**:
- **Save/Load**: Runtime state snapshot (mid-combat saves, exact HP/conditions preserved)
- **Library**: Pristine templates (fresh HP, clean slate, reusable encounters)

**Use Case**: Build library of common encounters during prep. Load "Dragon Lair" or "Random Bandits" instantly during session with fresh stats.

#### Combatant Templates

**Save Template**:
1. Select combatant with `↑`/`↓` in main view
2. Press `b` → "Save as Template"
3. Enter template name
4. Saves to `templates.json`

**Load Template**:
1. Press `b` → "Add from Template"
2. Filter by typing name
3. Select template
4. Prompt for initiative only
5. Combatant added with full HP from template

**Storage**: `templates.json` in project root (git-ignored)

**Use Case**: Quickly add recurring NPCs or monster types without re-entering HP/AC each time.

#### Death Saves & Concentration

**Death Saves**:
- Tracked for unconscious combatants (0 HP)
- Press `v` or Action Menu → "Roll Death Save"
- **Natural 1**: 2 failures added
- **Natural 20**: Creature revives at 1 HP
- **Success**: 3 successes = stabilized (unconscious but not dying)
- **Failure**: 3 failures = dead
- Visual indicator shows success/failure count in combatant list

**Concentration**:
- Press `c` or Action Menu → "Set Concentration"
- Select concentrating combatant, enter spell name
- Indicator appears in combatant display: `[Conc: Spell Name]`
- When combatant takes damage:
  - Automatically prompts for Constitution save
  - DC = `max(10, damage / 2)` per D&D 5e rules
  - Pass: concentration maintained
  - Fail: concentration broken, spell ends
- Unconscious = auto-break concentration

**Use Case**: Track caster concentration without manual DC calculation. App handles math automatically.

#### Status Effects & Conditions

**Apply Status**:
1. Press `s` or Action Menu → "Add Status Effect"
2. Select target combatant
3. Choose from 14 D&D 5e conditions
4. Set duration:
   - `0` = Indefinite (manual removal only)
   - `1+` = Timed rounds (auto-decrements on combatant's turn end)
5. Condition badge appears on combatant: `[Prone]`, `[Paralyzed (2)]`

**Quick Reference**:
- Press `?` to open scrollable condition reference modal
- Shows all 14 conditions with full mechanical descriptions
- Navigate with `↑`/`↓`, close with `Esc` or `q`
- Descriptions include advantage/disadvantage, auto-fail saves, speed changes, etc.

**Clear Status**:
- Press `x` or Action Menu → "Clear Concentration/Status"
- Choose "Clear Status"
- Select target combatant
- If multiple statuses, select which to remove
- If single status, removes immediately

**Use Case**: Apply "Prone" condition after shove, set duration 0 for manual removal. Apply "Hold Person" paralysis for 3 rounds.

#### HP & Temporary HP

**Damage Flow**:
1. Press `d` or Action Menu → "Deal Damage"
2. Select target
3. Enter damage amount
4. Temporary HP consumed first, then regular HP
5. If concentration active, automatic save prompt

**Temporary HP Rules** (per D&D 5e):
- Consumed before regular HP
- Doesn't heal regular HP
- Doesn't stack: higher value replaces lower
- Grant via Action Menu → "Grant Temp HP"

**HP Visualization**:
- Color-coded bars in combatant list:
  - **Green**: >50% HP remaining
  - **Yellow**: 25-50% HP remaining
  - **Red**: <25% HP remaining
  - **Gray**: 0 HP (unconscious/dead)
- Shows current/max: `HP: 18/35`
- Temp HP shown separately: `Temp: 5`

**Example**:
```
Combatant has 20 HP, 5 temp HP
Take 8 damage → 5 temp consumed, 3 HP lost → 17 HP, 0 temp
Grant 10 temp HP → 17 HP, 10 temp (replaces 0)
```

#### Combat Log

- Right-side panel showing recent combat events
- Automatically records:
  - Damage dealt and to whom
  - Healing applied
  - Status effects added/removed
  - Death save results
  - Concentration set/broken
  - Turn advances and round increments
- Retains up to 200 entries (prevents unbounded growth)
- Scrolls automatically to show most recent
- Persists in save files

**Use Case**: Review what happened last turn, track damage sources, audit combat flow.

## D&D 5e Rules Implemented

This tracker implements the following D&D 5th Edition rules:

### Concentration
- DC calculation: `max(10, damage_taken / 2)`
- Automatically prompts for save when concentrating creature takes damage
- Auto-breaks on unconsciousness
- Tracks spell name for reference

### Death Saves
- Three successes = stabilized (0 HP but not dying)
- Three failures = dead
- Natural 1 = 2 failures (critical failure)
- Natural 20 = revive at 1 HP (critical success)
- Resets when healed above 0 HP

### Temporary Hit Points
- Always consumed before regular HP
- Never stacks: new temp HP only replaces if higher than current
- Doesn't heal regular HP
- Lost when combatant reaches 0 HP

### Status Effects
- Duration 0 = indefinite (manual removal required)
- Duration >0 = timed (decrements at end of affected creature's turn)
- Duration <0 = expired (removed from combatant)
- All 14 standard conditions with accurate mechanical effects

### Initiative
- Automatic sorting by initiative value (descending)
- Turn order maintains until combatant removed
- Round counter increments after last combatant's turn

## Architecture

This project follows a **modal state machine pattern** with strict separation of concerns:

### Core Structure

```
src/
├── models/          # Pure data structures with business logic
│   ├── combatant.rs         # Combatant struct, HP, death saves
│   ├── status.rs            # StatusEffect, ConditionType, durations
│   └── combatant_template.rs   # Template serialization
├── combat.rs        # Combat encounter logic, turn management
├── app/             # Application state orchestration
│   ├── mod.rs               # App struct, InputMode enum (28 variants)
│   ├── state.rs             # State transitions
│   └── persistence.rs       # Save/load, encounter library
├── ui/              # Stateless rendering and input handling
│   ├── render.rs            # All rendering logic (954 LOC)
│   └── input/
│       ├── entry.rs         # Input event dispatcher
│       ├── normal.rs        # Normal mode keyboard shortcuts
│       ├── menus.rs         # Action/Combatant menu handlers
│       └── ...              # Mode-specific handlers
└── main.rs          # Event loop (render → poll → handle → update)
```

### Design Patterns

**Modal State Machine**:
- 28 `InputMode` enum variants represent different UI states
- Each mode has dedicated input handler and render function
- State transitions orchestrated through `App` methods
- Examples: `Normal`, `AddingCombatant`, `DealingDamage`, `ActionMenu`, `QuickReference`

**Stateless UI Layer**:
- `render.rs` and `input.rs` have zero internal state
- All state lives in `App` and domain models
- Rendering functions read from `App`, input handlers mutate `App`
- Unidirectional data flow: `Event → Handler → Mutate App → Render`

**Model-View Pattern**:
- **Models**: `Combatant`, `StatusEffect`, `CombatEncounter` (pure business logic)
- **View**: `render.rs` (declarative ratatui widgets)
- **Controller**: `App` (orchestrates models, manages workflow)

**Event Loop** (main.rs):
```rust
loop {
    render(&mut terminal, &app)?;           // 1. Render current state
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            handle_key_event(&mut app, key);    // 2. Handle input
        }
    }
    if app.should_quit { break; }           // 3. Check exit
}
```

### Key Architectural Decisions

- **Modal over screen-based navigation**: Keeps combat encounter visible while performing actions
- **No database**: Templates and encounters use JSON files (simplicity over complexity)
- **Stateless rendering**: Easier testing, no synchronization bugs
- **Clone over borrow**: Small state size makes cloning acceptable for cleaner code

## Technical Details

### Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `ratatui` | 0.29 | Terminal UI framework (declarative widgets) |
| `crossterm` | 0.28 | Cross-platform terminal control (events, raw mode) |
| `serde` | 1.0 | Serialization framework (derive macros) |
| `serde_json` | 1.0 | JSON serialization for templates/encounters |
| `anyhow` | 1.0 | Error handling with context |
| `log` | 0.4 | Logging facade |
| `env_logger` | 0.11 | Logger implementation |

### Requirements

- **Rust**: 1.90.0 or later (2024 edition)
- **Terminal**: ANSI color support (most modern terminals)
- **OS**: Cross-platform (Linux, macOS, Windows via `crossterm`)

## Development

### Build Commands

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Run in debug mode
cargo run

# Run in release mode (faster)
cargo run --release
```

### Testing

```bash
# Run all tests (42 total)
cargo test

# Run specific test module
cargo test combatant
cargo test status
cargo test app

# Show println! output in tests
cargo test -- --nocapture

# Run library tests only (exclude integration tests)
cargo test --lib
```

**Test Coverage**: 42 passing tests covering:
- Combatant mechanics (HP, temp HP, death saves)
- Status effect duration logic
- Combat turn advancement
- App-level integration workflows
- Encounter library save/load
- Concentration mechanics

### Code Quality

```bash
# Format code (rustfmt)
cargo fmt

# Lint code (clippy)
cargo clippy

# Check for errors without building
cargo check
```

### Adding New Features

See `CLAUDE.md` for detailed guidance on:
- Adding new combat actions (InputMode → start method → complete method → handler → renderer)
- Modifying combatant data structures
- Adding new conditions
- Testing patterns and co-located test modules

### Project Files

- **Combat mechanics**: `src/combat.rs`, `src/models/combatant.rs`
- **Turn order logic**: `src/combat.rs:next_turn()`, `src/app/mod.rs`
- **Main event loop**: `src/main.rs:53-69`
- **Modal state machine**: `src/app/state.rs` (InputMode enum)
- **All input handling**: `src/ui/input/entry.rs` (dispatcher), `src/ui/input/normal.rs` (normal mode)
- **All rendering**: `src/ui/render.rs`

**Completed Features**:
- ✅ Status effect selection and duration tracking
- ✅ Death saves with critical success/failure
- ✅ Concentration tracking with automatic DC calculation
- ✅ Combatant templates (save/load individual stat blocks)
- ✅ HP visualization with color-coded bars
- ✅ Combat log (200 entry history)
- ✅ Temporary HP mechanics (D&D 5e compliant)
- ✅ Quick reference modal for conditions
- ✅ Comprehensive unit testing (42 tests)
- ✅ **Save/Load Encounters** (complete combat state persistence)
- ✅ **Encounter Library** (reusable encounter templates)

**High-Priority Planned**:
- Legendary Actions tracking for boss monsters
- Undo system for last action
- Bulk add combatants (import multiple at once)
- Damage type resistance/immunity/vulnerability

See the roadmap for complete feature list with priorities, difficulty estimates, and technical specifications.

## Contributing

Contributions are welcome! To contribute:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests and linting (`cargo test && cargo fmt && cargo clippy`)
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to your branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

Please ensure:
- All tests pass
- Code is formatted with `cargo fmt`
- No clippy warnings
- New features include tests
- README is updated if adding user-facing features

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Acknowledgments

Built with:
- **Rust** - Systems programming language emphasizing safety and performance
- **ratatui** - Terminal UI framework for building rich interactive applications
- **crossterm** - Cross-platform terminal manipulation library

Special thanks to the D&D 5e community and the Rust TUI ecosystem.

## Legal Notice

This application is an independent tool for tracking combat encounters using game mechanics and is not affiliated with, endorsed, sponsored, or specifically approved by Wizards of the Coast LLC.

Dungeons & Dragons, D&D, and Wizards of the Coast are trademarks of Wizards of the Coast LLC in the U.S.A. and other countries. ©1993-2025 Wizards of the Coast LLC.

This software uses game mechanics available under the Open Game License but does not reproduce copyrighted material. All monster names, spell names, and other content belong to their respective copyright holders.

---

**Happy adventuring, and may your initiative rolls be high!**
