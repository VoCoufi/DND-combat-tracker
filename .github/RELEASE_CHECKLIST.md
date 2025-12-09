# Release Checklist

Follow these steps for each release to ensure a smooth and consistent process.

## Pre-Release (1-2 days before)

### Code Quality

- [ ] All planned features merged to `master`
- [ ] All tests passing locally:
  ```bash
  cargo test
  ```
- [ ] No clippy warnings:
  ```bash
  cargo clippy -- -D warnings
  ```
- [ ] Code properly formatted:
  ```bash
  cargo fmt --all -- --check
  ```
- [ ] Build succeeds in release mode:
  ```bash
  cargo build --release
  ```

### Documentation

- [ ] README.md is up to date with new features
- [ ] CLAUDE.md reflects any architectural changes
- [ ] CONTRIBUTING.md is current
- [ ] Update version number in `Cargo.toml`:
  ```toml
  version = "0.X.X"
  ```
- [ ] Update `CHANGELOG.md`:
  - Add release date for version section
  - Ensure all changes are documented
  - Add comparison link at bottom

### Version Control

- [ ] Commit all changes:
  ```bash
  git add .
  git commit -m "chore: prepare v0.X.X release"
  ```
- [ ] Push to master:
  ```bash
  git push origin master
  ```

## Release Day

### CI Verification

- [ ] Go to [GitHub Actions](https://github.com/VoCoufi/DND-combat-tracker/actions)
- [ ] Verify CI workflow passes on latest commit
- [ ] Check that all three platforms (Linux, macOS, Windows) pass

### Create Release

- [ ] Ensure you're on master and up to date:
  ```bash
  git checkout master
  git pull origin master
  ```

- [ ] Create annotated tag:
  ```bash
  git tag -a v0.X.X -m "Release v0.X.X

  Brief summary of major changes in this release."
  ```

- [ ] Push tag to trigger release workflow:
  ```bash
  git push origin v0.X.X
  ```

### Monitor Release Workflow

- [ ] Go to [GitHub Actions](https://github.com/VoCoufi/DND-combat-tracker/actions)
- [ ] Watch "Release" workflow execute (~15-20 minutes)
- [ ] Verify all build jobs complete successfully:
  - [ ] Linux x86_64
  - [ ] macOS x86_64 (Intel)
  - [ ] macOS aarch64 (Apple Silicon)
  - [ ] Windows x86_64
- [ ] Check [Releases page](https://github.com/VoCoufi/DND-combat-tracker/releases) for new release

### Verify Release

- [ ] All four binary archives are attached to the release:
  - [ ] `dnd-combat-tracker-linux-x86_64.tar.gz`
  - [ ] `dnd-combat-tracker-macos-x86_64.tar.gz`
  - [ ] `dnd-combat-tracker-macos-aarch64.tar.gz`
  - [ ] `dnd-combat-tracker-windows-x86_64.zip`

- [ ] Download and test one binary (spot check):
  ```bash
  # Example for Linux
  tar -xzf dnd-combat-tracker-linux-x86_64.tar.gz
  ./dnd-combat-tracker
  ```
  - [ ] Application launches without errors
  - [ ] Main menu works
  - [ ] Can add a combatant
  - [ ] Can quit cleanly

### Polish Release Notes

- [ ] Edit the GitHub release to enhance auto-generated notes
- [ ] Add highlights from CHANGELOG.md
- [ ] Include installation instructions (should be auto-added)
- [ ] Add any known issues or breaking changes
- [ ] Add link to full CHANGELOG.md
- [ ] Mark as "Latest Release" if appropriate

## Post-Release

### Announcements

Choose platforms based on your target audience:

- [ ] Reddit:
  - [ ] /r/DnD
  - [ ] /r/DnDBehindTheScreen
  - [ ] /r/rust (if significant Rust community interest)
- [ ] Discord servers for DMs
- [ ] Twitter/Mastodon
- [ ] Hacker News "Show HN" (for major releases)

### Monitoring

- [ ] Monitor [GitHub Issues](https://github.com/VoCoufi/DND-combat-tracker/issues) for bug reports
- [ ] Respond to user feedback
- [ ] Create hotfix if critical bugs are found

### Planning

- [ ] Update ROADMAP.md if priorities changed
- [ ] Create milestone for next release
- [ ] Review and close completed issues
- [ ] Plan next release features

## Hotfix Process

If a critical bug is found after release:

1. Create hotfix branch from release tag:
   ```bash
   git checkout -b hotfix/v0.X.X v0.X.X
   ```

2. Fix the bug and test thoroughly

3. Commit and push:
   ```bash
   git commit -am "fix: critical bug description"
   git push origin hotfix/v0.X.X
   ```

4. Create PR to master

5. After merge, create new patch release (e.g., v0.X.X+1)

6. Follow release process again for patch version

## Release Types

### Patch Release (0.X.X → 0.X.X+1)
- Bug fixes only
- No new features
- Backward compatible

### Minor Release (0.X.0 → 0.X+1.0)
- New features
- Bug fixes
- Backward compatible
- May deprecate features

### Major Release (0.X.0 → 1.0.0 or 1.X.0 → 2.0.0)
- Breaking changes
- Major new features
- May remove deprecated features
- Update migration guide

## Troubleshooting

### Build Fails on CI

- Check GitHub Actions logs for specific error
- Reproduce locally with:
  ```bash
  cargo build --release --target <target-triple>
  ```
- Common issues:
  - Missing dependencies
  - Platform-specific code issues
  - Test failures

### Release Workflow Doesn't Trigger

- Ensure tag starts with `v` (e.g., `v0.5.0`, not `0.5.0`)
- Check that tag was pushed: `git push origin --tags`
- Verify workflow file syntax

### Binary Doesn't Work

- Ensure proper target triple used
- Check that binary is executable: `chmod +x dnd-combat-tracker`
- Verify dependencies (should be statically linked)

## Resources

- [Semantic Versioning](https://semver.org/)
- [Keep a Changelog](https://keepachangelog.com/)
- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Rust Release Process](https://doc.rust-lang.org/cargo/reference/publishing.html)

---

**Remember:** Take your time, test thoroughly, and don't rush releases. Quality over speed.
