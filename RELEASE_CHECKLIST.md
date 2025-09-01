# Release Checklist for quantum-pulse v0.1.5

## Pre-Release Verification

### 1. Code Quality
- [ ] All tests pass: `cargo test --all-features`
- [ ] No clippy warnings: `cargo clippy --all-features -- -D warnings`
- [ ] Code is formatted: `cargo fmt --all -- --check`
- [ ] Examples compile and run: `cargo build --examples --all-features`
- [ ] Documentation builds: `cargo doc --all-features --no-deps`

### 2. Version Updates
- [x] Update version in `quantum-pulse/Cargo.toml` to 0.1.5
- [x] Update version in `quantum-pulse-macros/Cargo.toml` to 0.1.5
- [x] Update dependency version for quantum-pulse-macros in main Cargo.toml
- [x] Update version numbers in README.md examples

### 3. Documentation
- [x] CHANGELOG.md updated with new features and changes
- [x] README.md emphasizes ProfileOp as recommended approach
- [x] USER-GUIDE.md updated with ProfileOp examples
- [x] All code examples in documentation tested
- [ ] API documentation reviewed for new public items
- [x] Logo included and optimized (< 150KB)
- [x] Logo properly referenced in README for crates.io display

### 4. New Features Testing
- [x] ProfileOp derive macro works correctly
- [x] Category deduplication works as expected
- [x] Complex enum variants (tuple, struct) supported
- [x] Multiple enums with same category names don't conflict
- [x] Macro integration tests pass

## Publishing Process

### Step 1: Publish quantum-pulse-macros (MUST BE FIRST)
```bash
cd quantum-pulse-macros
cargo publish
```
Wait for crates.io to index (usually 1-2 minutes)

### Step 2: Verify macro crate is available
```bash
cargo search quantum-pulse-macros
```
Should show version 0.1.5

### Step 3: Update main crate dependency
In `quantum-pulse/Cargo.toml`, change:
```toml
quantum-pulse-macros = { version = "0.1.5", optional = true }
```
(Remove `path = "quantum-pulse-macros"` for the published version)

### Step 4: Test with published macro crate
```bash
cd quantum-pulse
cargo clean
cargo test --all-features
```

### Step 5: Publish main crate
```bash
cargo publish
```

### Step 6: Verify publication
```bash
cargo search quantum-pulse
```
Should show version 0.1.5

## Post-Release

### 1. Git Tasks
- [ ] Commit all changes
- [ ] Tag release: `git tag -a v0.1.5 -m "Release v0.1.5: ProfileOp derive macro"`
- [ ] Push tags: `git push origin v0.1.5`
- [ ] Create GitHub release with changelog

### 2. Documentation
- [ ] Verify docs.rs built successfully
- [ ] Update any external documentation or blog posts
- [ ] Announce in relevant Rust communities if appropriate

### 3. Project Maintenance
- [ ] Restore local path dependency for development:
  ```toml
  quantum-pulse-macros = { version = "0.1.5", path = "quantum-pulse-macros", optional = true }
  ```
- [ ] Bump to next development version (0.1.6-dev) if continuing development
- [ ] Create issues for any known limitations or future improvements

## Rollback Plan

If issues are discovered post-release:

1. **Yank if critical**: `cargo yank --version 0.1.5`
2. **Fix issues** in a patch release (0.1.6)
3. **Document** the issue in CHANGELOG.md
4. **Communicate** with users about the fix

## Known Issues for 0.1.5

- [ ] Some tests have flaky behavior due to shared state (non-critical)
- [ ] Examples have unused warnings for async futures (cosmetic)

## Feature Highlights for 0.1.5

✅ **ProfileOp Derive Macro**: Zero-boilerplate profiling
✅ **Automatic Category Management**: Intelligent description reuse  
✅ **Support for Complex Enums**: All variant types supported
✅ **Improved Documentation**: ProfileOp-first approach
✅ **Better Ergonomics**: Cleaner, more maintainable code

## Testing Commands Summary

```bash
# Full test suite
cargo test --all-features

# Specific macro tests
cargo test --features macros --test macro_test

# Run new example
cargo run --example macro_derive --features full

# Package verification
cargo package --list --allow-dirty

# Verify logo is included and sized correctly
ls -lh logo.png  # Should be < 150KB

# Documentation check
cargo doc --all-features --no-deps --open
```

## Notes

- The macro crate MUST be published before the main crate
- Allow 1-2 minutes for crates.io indexing between publishes
- Version 0.1.5 is backward compatible with 0.1.4
- The `ProfileOp` derive is optional but strongly recommended