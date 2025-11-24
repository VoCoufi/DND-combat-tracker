# D&D 5e Combat Tracker

A terminal-based combat encounter tracker for Dungeons & Dragons 5th Edition, built with Rust.

## Features

- **Initiative Tracking**: Automatically sorts and manages turn order
- **Combatant Management**: Add and remove players and monsters during combat
- **HP Tracking**: Deal damage and heal combatants with real-time HP display
- **Status Effects**: Track D&D 5e conditions (Stunned, Poisoned, Prone, etc.) with automatic duration countdown
- **Interactive TUI**: Beautiful terminal UI with color coding
  - Green for player characters
  - Red for enemies
  - Yellow for status effects
  - HP-based color coding (green/yellow/red based on health)
- **Round Tracking**: Automatically increments rounds and advances turns

## Installation

### Prerequisites

- Rust 1.90.0 or later
- A terminal that supports ANSI colors

### Build from Source

```bash
git clone <repository-url>
cd dnd-combat-tracker
cargo build --release
```

### Run

```bash
cargo run --release
```

## Usage

### Keyboard Controls

#### Normal Mode (Main View)

| Key | Action |
|-----|--------|
| `n` | Next turn (advances to next combatant) |
| `a` | Add a new combatant |
| `d` | Deal damage to a combatant |
| `h` | Heal a combatant |
| `s` | Add a status effect |
| `r` | Remove a combatant |
| `q` | Quit the application |

#### Modal Modes

| Key | Action |
|-----|--------|
| `Esc` | Cancel current operation |
| `Enter` | Confirm input / Proceed to next step |
| `↑/↓` | Navigate combatant selection |
| `Backspace` | Delete last character in input |

### Workflow

#### 1. Adding Combatants

1. Press `a` to start adding a combatant
2. Enter the following information when prompted:
   - **Name**: Combatant's name (e.g., "Goblin", "Thorin")
   - **Initiative**: Initiative roll result (e.g., 15)
   - **Max HP**: Maximum hit points (e.g., 25)
   - **AC**: Armor class (e.g., 14)
   - **Is Player?**: Enter `y` for player characters, `n` for NPCs/monsters
3. Press `Enter` after each field
4. The combatant will be automatically added and sorted by initiative

#### 2. Managing Combat

- Press `n` to advance to the next turn
  - Status effects automatically decrement at the end of each turn
  - Round counter increments when returning to the first combatant

#### 3. Dealing Damage

1. Press `d` to deal damage
2. Use `↑/↓` to select the target
3. Type the damage amount
4. Press `Enter` to confirm

#### 4. Healing

1. Press `h` to heal
2. Use `↑/↓` to select the target
3. Type the heal amount
4. Press `Enter` to confirm

#### 5. Adding Status Effects

1. Press `s` to add a status effect
2. Use `↑/↓` to select the combatant
3. Press `Enter` to proceed
4. Select the condition type (currently simplified - press `Esc` to cancel)

*Note: Status effect selection is simplified in this version. Future updates will include full condition selection.*

#### 6. Removing Combatants

1. Press `r` to remove a combatant
2. Use `↑/↓` to select the combatant to remove
3. Press `Enter` to confirm removal

## Supported D&D 5e Conditions

The tracker supports all standard D&D 5e conditions:

- Blinded
- Charmed
- Deafened
- Frightened
- Grappled
- Incapacitated
- Invisible
- Paralyzed
- Petrified
- Poisoned
- Prone
- Restrained
- Stunned
- Unconscious

## Example Combat Session

```
1. Start the application
2. Press 'a' and add your players:
   - Name: Thorin, Init: 18, HP: 35, AC: 16, Player: y
   - Name: Lyra, Init: 14, HP: 28, AC: 14, Player: y
3. Press 'a' and add enemies:
   - Name: Goblin 1, Init: 15, HP: 7, AC: 13, Player: n
   - Name: Goblin 2, Init: 12, HP: 7, AC: 13, Player: n
4. Combat automatically sorts by initiative
5. Press 'n' to advance through turns
6. Use 'd' to deal damage, 'h' to heal as needed
7. Press 'q' when combat is complete
```

## Technical Details

### Architecture

- **Models**: Core data structures (Combatant, StatusEffect, ConditionType)
- **Combat**: Combat encounter logic and turn management
- **App**: Application state management
- **UI**: Rendering and input handling using `ratatui`

### Dependencies

- `ratatui` (0.29): Terminal UI framework
- `crossterm` (0.28): Cross-platform terminal manipulation
- `serde` (1.0): Serialization framework
- `serde_json` (1.0): JSON serialization
- `anyhow` (1.0): Error handling

## Future Enhancements

- [ ] Full status effect selection modal
- [ ] Save/load combat encounters
- [ ] Dice roller integration
- [ ] Death saves tracking for player characters
- [ ] Concentration tracking
- [ ] Combat log/history
- [ ] Monster stat blocks
- [ ] Multiple encounter management

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## License

This project is open source. Feel free to use and modify as needed for your D&D sessions.

## Acknowledgments

Built with Rust and powered by the `ratatui` terminal UI framework.
