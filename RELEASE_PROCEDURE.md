# Release Procedure (Git Tags & GitHub Releases)

Teloxidium is currently distributed via versioned Git tags (`vX.Y.Z`) and GitHub Releases rather than crates.io.

## Versioning Policy

Bump clean SemVer versions whenever merging support for a newer Telegram Bot API version or hotfix release (no `-m*` tag suffixes):

- **Breaking changes (`[**BC**]`):** Bump the **minor** version (`0.x.y` → `0.(x+1).0`) for `teloxide` and `teloxide-core` (and `teloxide-macros` only when it changes).
- **Non-breaking / additive Bot API releases & fixes:** Bump the **patch** version (`0.x.y` → `0.x.(y+1)`) for `teloxide` and `teloxide-core`. Note that `v0.23.1` (`teloxide-core 0.19.1`) was shipped as a historical pragmatic patch release carrying forward-compatibility enum variants and story/sticker multipart fixes, documented in `MIGRATION_GUIDE.md`; future patch releases should remain strictly non-breaking.
- **Independent crate version series:** `teloxide` (`0.23.x`) and `teloxide-core` (`0.19.x`) increment together per release, so `teloxide-core 0.19.x` pairs with `teloxide 0.23.x` rather than sharing a single version number.
- **Git tag format:** `v<teloxide-version>` (for example, `v0.23.0`, `v0.23.1`, `v0.24.0`), matching `version` in `crates/teloxide/Cargo.toml`.

## Release Steps

1. Prepare the version bump on a feature or milestone branch from `master`:
   - Update `version` (and `teloxide-core` path dependency version) in `crates/teloxide/Cargo.toml` and `crates/teloxide-core/Cargo.toml` (plus `crates/teloxide-macros/Cargo.toml` if changed).
   - Update `tag = "vX.Y.Z"` examples in `README.md`, `crates/teloxide-core/README.md`, and `crates/teloxide-core/src/lib.rs` (plus API coverage badges and `src/lib.rs` Bot API version notes when bumping Bot API level).
   - When changing Bot API baseline, update `api_version` in `crates/teloxide-core/schema.ron` and `version` / `recent_changes` in `crates/teloxide-core/custom_v2.json`.
   - Refresh `Cargo.lock` by running `cargo check --workspace`.
   - Move entries from `## unreleased` under `## <version> - YYYY-MM-DD` in `CHANGELOG.md` and `crates/teloxide-core/CHANGELOG.md` (and update `MIGRATION_GUIDE.md` when behavior or types change), leaving a fresh empty `## unreleased` heading at the top.
2. Open a Pull Request, wait for CI (`ci-pass`) to succeed, and merge into `master`.
3. Create and push the annotated Git tag from updated `master`:
   ```powershell
   git checkout master
   git pull --ff-only origin master
   git tag -a vX.Y.Z -m "Release vX.Y.Z"
   git push origin vX.Y.Z
   ```
4. Pushing `vX.Y.Z` triggers [`.github/workflows/release.yml`](.github/workflows/release.yml), which verifies that the tag is on `master`, matches `crates/teloxide/Cargo.toml`, `README.md` snippets, and changelog entries, runs the workspace verification suite + docs/doctests, and publishes the GitHub Release.

## Using a Release in Other Projects

### Direct Git Tag Dependency (Recommended)

In your bot's `Cargo.toml`:

```toml
[dependencies]
teloxide = { git = "https://github.com/sailself/teloxidium", tag = "vX.Y.Z", features = ["macros"] }
```

Because `crates/teloxide/Cargo.toml` references `teloxide-core` and `teloxide-macros` via workspace `path` dependencies, Cargo resolves all three crates from the same Git tag automatically while keeping `use teloxide::...` unchanged.

### Crate Naming & Third-Party Crates.io Ecosystem Crates

Teloxidium intentionally keeps the `teloxide`, `teloxide-core`, and `teloxide-macros` crate names so downstream bots can switch cleanly via the Git tag dependency above without rewriting imports. Because Teloxidium's version series (`0.23.x` / `0.19.x`) is not SemVer-compatible with upstream crates.io releases (`0.17.x` / `0.13.x`), `[patch.crates-io]` cannot unify Teloxidium with third-party crates.io packages that depend on upstream `teloxide` (those crates must either be unused or also patched to depend on your Teloxidium Git tag).
