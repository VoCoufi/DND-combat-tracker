# D&D 5e Combat Tracker

A terminal-based combat encounter tracker for Dungeons & Dragons 5th Edition, built with Rust.

## Features

- **Initiative & Rounds**: Automatically sorts by initiative and advances rounds/turns
- **Combatant Management**: Add/remove combatants, with optional templates for quick reuse
- **HP + Temp HP**: Damage/heal flows plus temp HP that is consumed before regular HP
- **Statuses & Conditions**: Full condition picker with durations (0 = indefinite) and per-condition clearing
- **Death Saves & Concentration**: Tracks death saves and concentration checks after damage
- **Action & Combatant Menus**: `m` for action menu (damage/heal/status/etc.), `b` for combatant menu (add/remove/templates/save)
- **Combat Log**: Right-side log of recent actions
- **Interactive TUI**: Color-coded HP bars, condition/status badges, and concentration/status displays

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
| `n` | Next turn |
| `m` | Action menu (damage, heal, status, death save, concentration, temp HP, clear) |
| `b` | Combatant menu (add, remove, add from template, save as template) |
| `q` | Quit |

#### Other Quick Keys

You can still use direct shortcuts if you prefer:

| Key | Action |
|-----|--------|
| `d` | Deal damage |
| `h` | Heal |
| `s` | Add status |
| `v` | Roll death save |
| `c` | Set concentration |
| `x` | Clear concentration/status |
| `t` | Add from template |
| `p` | Save template |
| `a` | Add combatant |
| `r` | Remove combatant |

#### Modal Modes

| Key | Action |
|-----|--------|
| `Esc` | Cancel current operation |
| `Enter` | Confirm input / Proceed to next step |
| `↑/↓` | Navigate selections |
| `Backspace` | Delete last character in input |

### Workflow Highlights

- **Turn Advance**: `n` advances turns; log records actions; statuses tick each turn.
- **Damage/Heal**: `d` / `h` (or via Action Menu `m`). Temp HP is consumed before regular HP.
- **Temp HP**: Grant via Action Menu (`m` -> Grant Temp HP).
- **Status Effects**: `s` to add; duration `0` = indefinite. Clear via `x` (Clear Menu) and choose the specific status if multiple.
- **Death Saves**: `v` to record rolls for downed PCs; tracks successes/failures and handles crit 1/20.
- **Concentration**: `c` to set; app prompts for CON save on damage and clears on failure/downed.
- **Templates**: Add via Combatant Menu (`b` -> Add from Template) with name filter; save a combatant as a template via `p` or Combatant Menu (`b` -> Save as Template). Templates live in `templates.json` (git-ignored).
- **Log**: Right-side panel shows recent actions; retains up to 200 entries.

## Supported D&D 5e Conditions

All standard conditions are available from the condition picker:

Blinded, Charmed, Deafened, Frightened, Grappled, Incapacitated, Invisible, Paralyzed, Petrified, Poisoned, Prone, Restrained, Stunned, Unconscious

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

- [ ] Save/load combat encounters
- [ ] Dice roller integration
- [ ] Monster stat blocks
- [ ] Multiple encounter management

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

## License

This project is open source. Feel free to use and modify as needed for your D&D sessions.

## Acknowledgments

Built with Rust and powered by the `ratatui` terminal UI framework.
