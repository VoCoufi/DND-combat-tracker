# D&D Combat Tracker - Roadmap

This roadmap outlines planned features and improvements for the D&D 5e Combat Tracker. Features are organized by priority, with detailed technical specifications to guide implementation.

## How to Use This Document

- **Priority Levels**: High (critical for DM workflow) → Medium (quality of life) → Low (nice to have)
- **Difficulty**: Easy (< 1 day) | Medium (1-3 days) | Hard (3-7 days) | Very Hard (> 1 week)
- **Effort Estimates**: Approximate development time for an experienced Rust developer
- **Status**: 🟢 Completed | 🟡 In Progress | ⚪ Planned | 🔴 Blocked

---

## High Priority Features

Essential features that significantly improve the DM experience during live play sessions.

### 1. Complete Status Effect Selection Interface
**Status**: 🟢 Completed
**Difficulty**: Medium
**Estimated Effort**: 2-3 days

**Description:**
Implement a full modal interface for selecting conditions and specifying duration when adding status effects to combatants.

**Use Case:**
Currently, the status effect feature is simplified and non-functional. DMs frequently apply conditions like Stunned, Poisoned, or Prone during combat and need a quick way to select and apply them.

**Technical Approach:**
- Create a two-step modal: (1) select combatant, (2) select condition from ConditionType::all()
- Add an input field for duration in rounds
- Update `handle_condition_selection_mode()` in `src/ui/input.rs` to properly collect input
- Store temporary state for condition selection in `App`
- Call `complete_add_status()` when confirmed

**Dependencies:**
- Existing `ConditionType` enum ✅
- `StatusEffect::new()` method ✅
- Combatant status effect storage ✅

**Testing:**
- Apply each of the 14 condition types
- Verify duration decrements correctly on turn end
- Test canceling mid-selection
- Verify multiple conditions on same combatant

---

### 2. Death Saves Tracking
**Status**: 🟢 Completed
**Difficulty**: Medium
**Estimated Effort**: 2-3 days

**Description:**
Track death saving throws for player characters who reach 0 HP, including successes, failures, and stabilization.

**Use Case:**
When PCs drop to 0 HP in D&D 5e, they make death saving throws. DMs need to track 3 successes (stabilized) or 3 failures (dead). Critical rolls affect multiple saves.

**Technical Approach:**
- Add `DeathSaves` struct with `successes: u8`, `failures: u8`, `is_stable: bool`
- Add `Option<DeathSaves>` field to `Combatant`
- When HP drops to 0 for a player, initialize death saves
- Add new input mode: `RollingDeathSave(combatant_index)`
- New UI modal to roll/record death saves (d20 result → 10+: success, <10: fail, 1: two fails, 20: gain 1 HP)
- Auto-stabilize at 3 successes, mark dead at 3 failures
- Clear death saves when healed above 0 HP

**Dependencies:**
- Modify `Combatant` struct in `src/models/combatant.rs`
- Add new `DeathSaves` struct in `src/models/`
- New UI modal in `src/ui/render.rs`
- Input handling for death save recording

**Testing:**
- PC drops to 0 HP → death saves appear
- Track successes and failures correctly
- Heal to 1+ HP → death saves clear
- Take damage at 0 HP → auto-fail one save
- Critical 20 → restore 1 HP
- Critical 1 → two failures

---

### 3. Concentration Tracking
**Status**: 🟢 Completed
**Difficulty**: Hard
**Estimated Effort**: 3-4 days

**Description:**
Track which combatants are concentrating on spells and automatically prompt for Constitution saves when they take damage.

**Use Case:**
Many D&D 5e spells require concentration. When a caster takes damage, they must make a CON save (DC = 10 or half damage, whichever is higher) or lose concentration.

**Technical Approach:**
- Add `concentration: Option<ConcentrationInfo>` to `Combatant`
- Create `ConcentrationInfo` struct with:
  ```rust
  struct ConcentrationInfo {
      spell_name: String,
      duration_remaining: i32, // rounds
      constitution_modifier: i32,
  }
  ```
- When dealing damage to a concentrating combatant:
  - Calculate save DC: `max(10, damage / 2)`
  - Prompt DM to roll CON save or enter result
  - If failed: remove concentration and prompt for notification
- Add visual indicator in combatant list (e.g., "[Concentrating: Haste]")
- Add command to manually apply/remove concentration

**Dependencies:**
- Modify `Combatant` struct
- Enhance damage dealing flow to check concentration
- New UI elements for concentration display
- Input modal for CON save results

**Testing:**
- Apply concentration → shows in UI
- Take damage → prompts for save
- Successful save → concentration continues
- Failed save → concentration breaks
- Duration expires → concentration ends
- Apply new concentration → replaces old (can only concentrate on one spell)

---

### 4. Save/Load Encounters
**Status**: ⚪ Planned
**Difficulty**: Medium
**Estimated Effort**: 2 days

**Description:**
Persist combat encounters to JSON files and reload them later.

**Use Case:**
DMs may need to pause mid-combat (session ends, emergency) and resume later. Also useful for preparing common encounters.

**Technical Approach:**
- All models already derive `Serialize` and `Deserialize` ✅
- Add `save_encounter()` method to write `CombatEncounter` to JSON
- Add `load_encounter()` method to read from JSON
- Add keyboard shortcuts: `Ctrl+S` (save), `Ctrl+O` (open/load)
- Create modal for filename input
- Store saves in `~/.config/dnd-combat-tracker/saves/` or `./saves/`
- Handle file I/O errors gracefully

**Dependencies:**
- `serde_json` already added ✅
- File system operations (std::fs)
- New input modes: `SavingEncounter`, `LoadingEncounter`

**Testing:**
- Save mid-combat → reload → verify all data intact (HP, initiative, status effects, turn order, round number)
- Save empty encounter
- Load non-existent file → error handling
- Load corrupted JSON → error handling

---

### 5. Bulk Add Combatants
**Status**: ⚪ Planned
**Difficulty**: Easy
**Estimated Effort**: 1 day

**Description:**
Add multiple identical combatants at once (e.g., "5 goblins") with automatic numbering.

**Use Case:**
DMs frequently run encounters with multiple copies of the same creature (pack of wolves, goblin horde). Manually adding each one is tedious.

**Technical Approach:**
- Modify `AddingCombatant` modal to add optional "Quantity" field (default: 1)
- If quantity > 1, create N combatants with names like "Goblin 1", "Goblin 2", etc.
- Option to randomize HP within a range for each copy
- Option to use same initiative or roll separately (could prompt for each)

**Dependencies:**
- Modify `AddCombatantState` in `src/app.rs`
- Update rendering and input handling for quantity field

**Testing:**
- Add 5 goblins → creates 5 separate entries
- Verify proper numbering
- Each combatant is independent (damage to one doesn't affect others)

---

### 6. Undo Last Action
**Status**: ⚪ Planned
**Difficulty**: Hard
**Estimated Effort**: 3-5 days

**Description:**
Implement undo functionality for the last action (damage dealt, combatant added, turn advanced, etc.).

**Use Case:**
Mistakes happen during fast-paced combat. DMs may misclick, enter wrong damage, or advance turn accidentally.

**Technical Approach:**
- Implement command pattern with action history stack
- Create `Action` enum:
  ```rust
  enum Action {
      DamageDealt { combatant_idx: usize, amount: i32 },
      Healed { combatant_idx: usize, amount: i32 },
      CombatantAdded { combatant: Combatant },
      CombatantRemoved { combatant: Combatant, index: usize },
      TurnAdvanced { prev_index: usize, prev_round: u32 },
      StatusAdded { combatant_idx: usize, effect: StatusEffect },
  }
  ```
- Maintain `action_history: Vec<Action>` in `App` (limit to last 10-20 actions)
- Each action implements `undo()` method
- Add keyboard shortcut `Ctrl+Z` or `u` for undo
- Display brief message confirming undo

**Dependencies:**
- Significant refactoring of `App` methods to record actions
- Need to clone state before mutations for some actions

**Testing:**
- Undo each action type
- Undo multiple times in sequence
- Undo with empty history → no crash
- Verify undo correctly restores all state

---

### 7. Monster/Combatant Templates
**Status**: ⚪ Planned
**Difficulty**: Medium
**Estimated Effort**: 2-3 days

**Description:**
Save commonly-used combatants as templates for quick addition (e.g., "Goblin" template with preset HP, AC, initiative modifier).

**Use Case:**
DMs repeatedly use the same monster stat blocks. Templates eliminate repetitive data entry.

**Technical Approach:**
- Create `CombatantTemplate` struct with base stats
- Store templates in JSON file (`~/.config/dnd-combat-tracker/templates.json`)
- Add "Load from Template" option in combatant addition flow
- Allow creating new templates from existing combatants
- Include common D&D 5e monsters as default templates (goblins, orcs, zombies, etc.)

**Dependencies:**
- File I/O for template storage
- UI for template selection modal

**Testing:**
- Create custom template → save → load in new session
- Add combatant from template → verify stats match
- Edit template → verify changes persist

---

### 8. Improved HP Visualization
**Status**: 🟢 Completed
**Difficulty**: Easy
**Estimated Effort**: 1 day

**Description:**
Add visual HP bars next to each combatant showing current/max HP as a filled bar.

**Use Case:**
Quickly assess combatant health status at a glance without reading numbers.

**Technical Approach:**
- Use `ratatui::widgets::Gauge` or custom ASCII bar: `[████████░░] 32/40`
- Color code: Green (>50%), Yellow (25-50%), Red (<25%), Gray (0%)
- Already have `hp_percentage()` method ✅
- Render inline with combatant info

**Dependencies:**
- Minor update to `render_combatants()` in `src/ui/render.rs`

**Testing:**
- Verify bar fills correctly at various HP values
- Color changes at correct thresholds
- Works with very small and very large HP pools

---

## Medium Priority Features

Quality-of-life improvements and expanded functionality.

### 9. Built-in Dice Roller
**Status**: ⚪ Planned
**Difficulty**: Medium
**Estimated Effort**: 2-3 days

**Description:**
Add a dice rolling system supporting standard notation (1d20, 2d6+3, advantage/disadvantage).

**Use Case:**
While the current design assumes physical dice, a built-in roller can speed up gameplay and reduce errors.

**Technical Approach:**
- Create `dice` module with parser for dice notation
- Support: `NdN`, `NdN+M`, `NdN-M`, advantage (`adv 1d20`), disadvantage (`dis 1d20`)
- Add keyboard shortcut `Ctrl+R` to open dice roller modal
- Display roll results with breakdown (e.g., `2d6+3 = [4, 2] + 3 = 9`)
- Optionally integrate into damage/healing flows ("Roll damage" button)

**Dependencies:**
- Random number generation (use `rand` crate)
- Dice notation parser (regex or parser combinator)
- New UI modal for dice roller

**Testing:**
- Various dice notations parse correctly
- Advantage takes higher of 2d20
- Disadvantage takes lower of 2d20
- Edge cases: d1, d100, negative modifiers

---

### 10. Combat Log/History
**Status**: ⚪ Planned
**Difficulty**: Medium
**Estimated Effort**: 2 days

**Description:**
Display a scrollable log of combat events (damage dealt, turns advanced, effects applied).

**Use Case:**
DMs and players may need to review what happened earlier in combat (dispute resolution, tracking spell effects, etc.).

**Technical Approach:**
- Add `combat_log: Vec<LogEntry>` to `App` or `CombatEncounter`
- Create `LogEntry` struct:
  ```rust
  struct LogEntry {
      round: u32,
      timestamp: String,
      message: String,
  }
  ```
- Log events like: "Round 2: Goblin 1 took 8 damage (HP: 7 → -1)"
- Add UI panel (bottom or separate tab) to display last 10-20 entries
- Option to export log to file

**Dependencies:**
- Modify all combat operations to log events
- Add rendering for log panel

**Testing:**
- All actions create log entries
- Log correctly shows chronological order
- Log persists through save/load
- Export log creates valid file

---

### 11. Edit Combatant Stats Mid-Combat
**Status**: ⚪ Planned
**Difficulty**: Easy
**Estimated Effort**: 1-2 days

**Description:**
Allow editing combatant properties (name, HP, AC, initiative) after they've been added.

**Use Case:**
Mistakes in data entry, or changing circumstances (PC gains temporary AC buff, need to correct initiative roll).

**Technical Approach:**
- Add keyboard shortcut `e` (edit) or right-click menu
- Open modal pre-filled with current values
- Allow editing any field
- Re-sort by initiative if initiative changed

**Dependencies:**
- New input mode: `EditingCombatant(index, EditCombatantState)`
- Modal similar to `AddingCombatant` but pre-filled

**Testing:**
- Edit each field type
- Edit during different turns
- Verify initiative re-sort works
- Cancel edit → no changes applied

---

### 12. Temporary HP
**Status**: ⚪ Planned
**Difficulty**: Easy
**Estimated Effort**: 1 day

**Description:**
Track temporary hit points separately from regular HP.

**Use Case:**
Many D&D 5e abilities grant temporary HP (False Life, Inspiring Leader). Temp HP is lost first before regular HP.

**Technical Approach:**
- Add `temp_hp: i32` field to `Combatant` (default: 0)
- Modify `take_damage()`: consume temp HP first, then regular HP
- Display temp HP in UI: `HP: 32/40 (+5 temp)`
- Add command to grant temp HP (separate from healing)

**Dependencies:**
- Modify `Combatant` struct
- Update damage logic
- UI changes to display temp HP

**Testing:**
- Grant temp HP → take damage → temp HP consumed first
- Take more damage than temp HP → regular HP reduced correctly
- Temp HP doesn't stack (granting new temp HP replaces old if higher)
- Temp HP doesn't heal regular HP

---

### 13. Legendary Actions & Reactions
**Status**: ⚪ Planned
**Difficulty**: Hard
**Estimated Effort**: 4-5 days

**Description:**
Track legendary action points and reaction availability for relevant creatures.

**Use Case:**
Boss monsters often have legendary actions (3 per round, taken after other turns). Tracking these manually is error-prone.

**Technical Approach:**
- Add `legendary_actions: Option<LegendaryActions>` to `Combatant`
  ```rust
  struct LegendaryActions {
      max_per_round: u8,
      remaining: u8,
      actions: Vec<String>, // descriptions
  }
  ```
- Add `has_reaction: bool` to `Combatant`
- Reset legendary actions at start of creature's turn
- Reset all reactions at start of round
- Add UI to spend legendary actions
- Visual indicator for available legendary actions/reactions

**Dependencies:**
- Modify `Combatant` struct
- Enhance turn management to reset counters
- New UI elements and controls

**Testing:**
- Legendary actions reset correctly
- Can't spend more than max
- Reactions reset each round
- Multiple creatures with legendary actions

---

### 14. Initiative Re-roll
**Status**: ⚪ Planned
**Difficulty**: Easy
**Estimated Effort**: < 1 day

**Description:**
Allow re-rolling initiative for all combatants or specific combatants.

**Use Case:**
Some effects require re-rolling initiative, or DM may want to randomize turn order for subsequent rounds.

**Technical Approach:**
- Add command `i` (re-roll initiative)
- Modal to select: "All combatants" or "Select specific"
- Input new initiative values
- Re-sort initiative order
- Preserve current round number

**Dependencies:**
- New input mode for initiative re-entry
- Re-use existing initiative sorting

**Testing:**
- Re-roll all → order changes
- Re-roll one → only that combatant moves
- Verify turn tracking remains consistent

---

### 15. Quick Reference Panel
**Status**: ⚪ Planned
**Difficulty**: Easy
**Estimated Effort**: 1 day

**Description:**
Display a reference panel with condition effects (what each condition does).

**Use Case:**
DMs may forget exact effects of conditions (what does Restrained do again?).

**Technical Approach:**
- Add keyboard shortcut `?` or `F1` for help
- Modal displaying all 14 conditions with descriptions
- Load descriptions from static data or embedded resource
- Scrollable list

**Dependencies:**
- Static condition descriptions
- Help modal UI

**Testing:**
- All conditions display correctly
- Scrolling works for long list
- Modal dismisses properly

---

### 16. Search/Filter Combatants
**Status**: ⚪ Planned
**Difficulty**: Easy
**Estimated Effort**: 1 day

**Description:**
Add ability to search/filter combatant list by name or type (PC vs NPC).

**Use Case:**
In large battles (10+ combatants), finding a specific creature quickly.

**Technical Approach:**
- Add search input field (toggle with `/` key)
- Filter combatant display based on search term
- Support filters: `player:yes`, `player:no`, `hp:<10`, etc.

**Dependencies:**
- Input handling for search field
- Filter logic in render

**Testing:**
- Search by name (partial match)
- Filter by player status
- Clear search restores full list

---

### 17. Notes Field per Combatant
**Status**: ⚪ Planned
**Difficulty**: Easy
**Estimated Effort**: 1 day

**Description:**
Add a notes/description field to each combatant for DM reminders.

**Use Case:**
Track special abilities, roleplay notes, or combat tactics for specific combatants.

**Technical Approach:**
- Add `notes: String` to `Combatant`
- Display notes in expanded view or on hover
- Add command to edit notes

**Dependencies:**
- Modify `Combatant` struct
- UI for displaying/editing notes

**Testing:**
- Add notes to combatant
- Notes persist through save/load
- Long notes display properly

---

### 18. Condition Auto-Effects
**Status**: ⚪ Planned
**Difficulty**: Very Hard
**Estimated Effort**: 5-7 days

**Description:**
Automatically apply mechanical effects of conditions (e.g., Prone combatants show "attacks against: advantage").

**Use Case:**
Conditions have specific game effects. Automating reminders reduces DM cognitive load.

**Technical Approach:**
- Create mapping of conditions to mechanical effects
- Display effect reminders in combatant info
- Example: Prone → "melee attacks against: advantage, ranged attacks against: disadvantage"
- Does not auto-calculate, just displays reminders

**Dependencies:**
- Condition effect database
- Enhanced UI to show effect reminders

**Testing:**
- Each condition displays correct effects
- Multiple conditions combine properly
- Effects clear when condition expires

---

## Low Priority Features

Advanced features for enhanced experience but not essential for core functionality.

### 19. Multiple Encounter Management
**Status**: ⚪ Planned
**Difficulty**: Hard
**Estimated Effort**: 3-4 days

**Description:**
Manage multiple separate encounters simultaneously and switch between them.

**Use Case:**
Split parties, or prepping multiple encounters during session prep.

**Technical Approach:**
- Modify `App` to contain `encounters: Vec<CombatEncounter>`
- Add `active_encounter_index: usize`
- UI to list encounters, switch between them
- Keyboard shortcuts to switch (Alt+1, Alt+2, etc.)

**Dependencies:**
- Significant `App` restructuring
- UI for encounter management

**Testing:**
- Create multiple encounters
- Switch between encounters → state preserved
- Remove encounter
- Save/load multiple encounters

---

### 20. Custom User-Defined Conditions
**Status**: ⚪ Planned
**Difficulty**: Medium
**Estimated Effort**: 2 days

**Description:**
Allow DMs to create custom conditions beyond the standard 14.

**Use Case:**
Homebrew effects, environmental hazards, or campaign-specific conditions.

**Technical Approach:**
- Extend `ConditionType` enum with `Custom(String)` variant
- Store custom condition definitions in config
- Allow creating/editing custom conditions

**Dependencies:**
- Modify `ConditionType` enum
- Configuration file storage

**Testing:**
- Create custom condition
- Apply to combatant
- Custom condition persists through save/load

---

### 21. Export Combat Log
**Status**: ⚪ Planned
**Difficulty**: Easy
**Estimated Effort**: < 1 day

**Description:**
Export combat log to markdown or JSON file for record-keeping.

**Use Case:**
Session notes, post-game analysis, or sharing combat summaries.

**Technical Approach:**
- Add export command
- Format log as markdown or JSON
- Save to file with timestamp

**Dependencies:**
- Combat log implementation (Feature #10)
- File I/O

**Testing:**
- Export creates valid file
- Markdown formatting correct
- JSON parseable

---

### 22. Player View Mode
**Status**: ⚪ Planned
**Difficulty**: Very Hard
**Estimated Effort**: 1-2 weeks

**Description:**
Separate display mode for players showing only public information (initiative order, visible HP).

**Use Case:**
Project combat tracker on screen for players to see without revealing hidden info.

**Technical Approach:**
- TCP server to stream combat state
- Web-based or separate TUI client for player view
- Filter out hidden info (enemy exact HP, DM notes)
- Real-time updates

**Dependencies:**
- Network programming
- Client application
- State synchronization

**Testing:**
- Multiple clients connect
- Updates propagate correctly
- Hidden info properly filtered

---

### 23. Color Themes
**Status**: ⚪ Planned
**Difficulty**: Easy
**Estimated Effort**: 1 day

**Description:**
Support custom color schemes (dark mode, light mode, high contrast).

**Use Case:**
Different terminal environments, accessibility needs.

**Technical Approach:**
- Define `Theme` struct with color mappings
- Load theme from config file
- Apply colors throughout UI

**Dependencies:**
- Configuration file
- Refactor hard-coded colors

**Testing:**
- Switch between themes
- All UI elements use theme colors

---

### 24. Sound Effects
**Status**: ⚪ Planned
**Difficulty**: Medium
**Estimated Effort**: 2 days

**Description:**
Optional sound effects for combat events (damage, death, turn change).

**Use Case:**
Enhanced atmosphere during play.

**Technical Approach:**
- Use audio library (rodio crate)
- Embed or load sound files
- Toggle in settings

**Dependencies:**
- Audio library
- Sound files
- Config for enable/disable

**Testing:**
- Sounds play on correct events
- Can disable sounds
- Doesn't crash if audio device unavailable

---

### 25. REST API
**Status**: ⚪ Planned
**Difficulty**: Very Hard
**Estimated Effort**: 1-2 weeks

**Description:**
Expose combat tracker state via REST API for integration with external tools.

**Use Case:**
Integration with virtual tabletops, Discord bots, custom tools.

**Technical Approach:**
- HTTP server (actix-web or warp)
- REST endpoints for combat state, actions
- Webhook support for events

**Dependencies:**
- Web framework
- Async runtime
- API documentation

**Testing:**
- All endpoints work
- Concurrent requests handled
- Authentication/authorization

---

## Technical Improvements

Architectural enhancements and code quality improvements.

### 26. Unit Testing
**Status**: ⚪ Planned
**Difficulty**: Medium
**Estimated Effort**: 3-4 days

**Description:**
Add comprehensive unit tests for combat logic, combatant operations, and status effects.

**Technical Approach:**
- Test modules for each component
- Focus on `combat.rs`, `combatant.rs`, `status.rs`
- Achieve >80% coverage for core logic
- Use property-based testing (proptest) for combat state

**Dependencies:**
- Testing frameworks already available ✅

**Testing:**
- All edge cases covered
- CI integration

---

### 27. Integration Testing
**Status**: ⚪ Planned
**Difficulty**: Hard
**Estimated Effort**: 4-5 days

**Description:**
Automated UI testing to verify complete workflows.

**Technical Approach:**
- Simulate keyboard input sequences
- Assert on UI state
- Test full combat scenarios end-to-end

**Dependencies:**
- Test harness for TUI interaction
- Mock terminal backend

**Testing:**
- All user workflows tested
- Regression tests for bug fixes

---

### 28. Configuration File Support
**Status**: ⚪ Planned
**Difficulty**: Easy
**Estimated Effort**: 1 day

**Description:**
Load user preferences from config file (~/.config/dnd-combat-tracker/config.toml).

**Technical Approach:**
- Use `serde` + `toml` crate
- Config options: default save location, color theme, sound enabled, etc.

**Dependencies:**
- `toml` crate

**Testing:**
- Config loads correctly
- Missing config → use defaults
- Invalid config → error message

---

### 29. Better Error Handling
**Status**: ⚪ Planned
**Difficulty**: Medium
**Estimated Effort**: 2-3 days

**Description:**
Improve error messages and graceful degradation.

**Technical Approach:**
- Replace `unwrap()` with proper error handling
- User-friendly error messages in UI
- Log errors to file for debugging

**Dependencies:**
- Logging framework (tracing/log)

**Testing:**
- All error paths return user-friendly messages
- App doesn't crash on errors

---

### 30. Refactor Input Handling
**Status**: ⚪ Planned
**Difficulty**: Medium
**Estimated Effort**: 2-3 days

**Description:**
Clean up input.rs to use more robust state machine pattern.

**Technical Approach:**
- Formalize state transitions
- Reduce code duplication
- Better separation of concerns

**Dependencies:**
- Current input handling code

**Testing:**
- All input flows still work
- Code is more maintainable

---

## Contributing

Interested in implementing a feature from this roadmap? Here's how:

1. **Check Status**: Ensure the feature isn't already in progress (🟡)
2. **Open Issue**: Create a GitHub issue referencing the feature number
3. **Discuss Approach**: Comment on the technical approach, suggest alternatives
4. **Fork & Implement**: Follow the technical approach outlined
5. **Test Thoroughly**: Cover all testing considerations listed
6. **Submit PR**: Include tests, documentation updates, and reference the issue

### Development Setup

```bash
git clone <repository>
cd dnd-combat-tracker
cargo build
cargo test
cargo run
```

### Code Standards

- Follow Rust naming conventions
- Add doc comments for public APIs
- Include unit tests for new logic
- Update README.md if user-facing changes
- Run `cargo fmt` and `cargo clippy` before committing

---

## Feedback

Have ideas not on this roadmap? Open an issue with:
- **Feature Description**: What it does
- **Use Case**: Why it's valuable for DMs
- **Priority**: Your suggested priority level

We prioritize features based on:
1. Impact on DM workflow during live sessions
2. Implementation complexity vs. value
3. Community feedback and requests
4. Alignment with D&D 5e rules

---

*Last Updated: 2025-11-25*
*Total Features Planned: 30+*
