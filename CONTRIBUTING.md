# Contributing to D&D Combat Tracker

Thank you for your interest in contributing! This document provides guidelines and instructions for contributing to the project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Code Style](#code-style)
- [Testing](#testing)
- [Submitting Changes](#submitting-changes)
- [Feature Requests](#feature-requests)
- [Bug Reports](#bug-reports)

## Code of Conduct

- Be respectful and inclusive
- Provide constructive feedback
- Focus on what is best for the community
- Show empathy towards other community members

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:
   ```bash
   git clone https://github.com/YOUR-USERNAME/DND-combat-tracker.git
   cd DND-combat-tracker
   ```
3. **Create a branch** for your changes:
   ```bash
   git checkout -b feature/my-awesome-feature
   ```

## Development Setup

### Prerequisites

- **Rust 1.90.0 or later** ([install here](https://rustup.rs/))
- A terminal with ANSI color support

### Build and Run

```bash
# Build the project
cargo build

# Run in development mode
cargo run

# Build optimized release version
cargo build --release
cargo run --release
```

### Project Structure

```
src/
├── models/          # Pure data structures with business logic
│   ├── combatant.rs         # Combatant struct, HP, death saves
│   ├── status.rs            # StatusEffect, ConditionType
│   └── combatant_template.rs
├── combat.rs        # Combat encounter logic, turn management
├── app/             # Application state orchestration
│   ├── mod.rs               # App struct, InputMode enum
│   ├── state.rs             # State transitions
│   └── persistence.rs       # Save/load functionality
├── ui/              # Stateless rendering and input handling
│   ├── render.rs            # All rendering logic
│   └── input/               # Input handling by mode
└── main.rs          # Event loop
```

For detailed architectural guidance, see [`CLAUDE.md`](./CLAUDE.md).

## Code Style

### Rust Guidelines

We follow standard Rust conventions:

1. **Format your code** with rustfmt:
   ```bash
   cargo fmt
   ```

2. **Lint your code** with clippy:
   ```bash
   cargo clippy -- -D warnings
   ```

3. **Check for errors** without building:
   ```bash
   cargo check
   ```

### Code Standards

- Use descriptive variable and function names
- Add doc comments (`///`) for public APIs
- Keep functions focused and single-purpose
- Prefer explicit error handling over `.unwrap()` in production code
- Add inline comments for complex logic
- Follow the existing code organization patterns

### Architecture Principles

- **Stateless UI**: Rendering and input handling have no internal state
- **Unidirectional data flow**: `Event → Handler → Mutate App → Render`
- **Domain models**: Business logic lives in model structs, not UI code
- **Modal state machine**: Use `InputMode` variants for different UI states

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test module
cargo test combatant
cargo test status
cargo test app

# Run with output visible
cargo test -- --nocapture

# Run library tests only
cargo test --lib
```

### Writing Tests

- Co-locate tests with the code they test using `#[cfg(test)]` modules
- Test both success and error cases
- Use descriptive test names: `test_combatant_takes_damage_reduces_hp()`
- Create helper functions for common test setup (e.g., `player(name, hp)`)

### Test Coverage Requirements

- All new public functions should have tests
- Bug fixes should include regression tests
- Aim for >80% coverage of core logic (models, combat)

Example test structure:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn player(name: &str, hp: i32) -> Combatant {
        Combatant::new(name.to_string(), 20, hp, 15, true)
    }

    #[test]
    fn test_take_damage_reduces_hp() {
        let mut c = player("Test", 30);
        c.take_damage(10);
        assert_eq!(c.hp_current, 20);
    }
}
```

## Submitting Changes

### Pull Request Process

1. **Ensure your code passes all checks**:
   ```bash
   cargo test
   cargo fmt --all -- --check
   cargo clippy -- -D warnings
   ```

2. **Update documentation** if needed:
   - Update `README.md` for user-facing features
   - Update `CLAUDE.md` for architectural changes
   - Update `ROADMAP.md` if implementing planned features

3. **Commit your changes** with clear messages:
   ```bash
   git add .
   git commit -m "feat: add legendary action tracking"
   ```

   Use conventional commit prefixes:
   - `feat:` - New features
   - `fix:` - Bug fixes
   - `docs:` - Documentation only
   - `refactor:` - Code changes that neither fix bugs nor add features
   - `test:` - Adding or updating tests
   - `chore:` - Maintenance tasks

4. **Push to your fork**:
   ```bash
   git push origin feature/my-awesome-feature
   ```

5. **Create a Pull Request** on GitHub:
   - Provide a clear title and description
   - Reference any related issues (e.g., "Fixes #42")
   - Describe what changed and why
   - Include screenshots for UI changes

### PR Review Process

- Maintainers will review your PR
- Address any requested changes
- Once approved, maintainers will merge your PR
- Your contribution will be included in the next release

## Feature Requests

Have an idea for a new feature?

1. **Check the roadmap**: See [`ROADMAP.md`](./ROADMAP.md) and [`Claude_ROADMAP.md`](./Claude_ROADMAP.md) for planned features
2. **Search existing issues**: Your idea may already be discussed
3. **Open a new issue** with the template:

```markdown
### Feature Description
A clear description of what you want to happen.

### Use Case
Why is this valuable for DMs? When would this be used?

### Proposed Implementation
(Optional) Technical approach or suggestions.

### Priority
Your suggested priority: High/Medium/Low

### D&D 5e Rules Reference
(If applicable) Link to relevant D&D 5e rules.
```

## Bug Reports

Found a bug? Help us fix it!

1. **Search existing issues** to avoid duplicates
2. **Open a new issue** with details:

```markdown
### Bug Description
A clear description of what the bug is.

### Steps to Reproduce
1. Start application
2. Add combatant
3. Deal damage
4. See error

### Expected Behavior
What you expected to happen.

### Actual Behavior
What actually happened.

### Environment
- OS: [e.g., Ubuntu 22.04, macOS 14, Windows 11]
- Rust version: [output of `rustc --version`]
- Application version: [e.g., 0.5.0]

### Additional Context
Any other context, logs, or screenshots.
```

## Development Tips

### Adding a New Combat Action

See `CLAUDE.md` for detailed instructions. General pattern:

1. Add variant to `InputMode` enum in `src/app/mod.rs`
2. Add start method: `pub fn start_new_action(&mut self)`
3. Add completion method: `pub fn complete_new_action(&mut self, ...)`
4. Add handler in `src/ui/input/entry.rs`
5. Add modal renderer in `src/ui/render.rs`
6. Add keyboard shortcut
7. Add tests

### Working with the Codebase

- **Models are pure**: No UI logic in `src/models/`
- **UI is stateless**: All state lives in `App` and domain models
- **State transitions**: Use `App` methods to orchestrate workflows
- **Error handling**: Use `anyhow::Result` and provide context

### Common Pitfalls

- Don't use `.unwrap()` in production code paths
- Don't add state to rendering functions
- Don't bypass the `InputMode` state machine
- Don't forget to add tests for new features

## Questions?

- Open an issue with the "question" label
- Check existing issues and discussions
- Review `CLAUDE.md` for architectural guidance

## License

By contributing, you agree that your contributions will be licensed under the same terms as the project (MIT OR Apache-2.0).

---

Thank you for contributing to D&D Combat Tracker!
