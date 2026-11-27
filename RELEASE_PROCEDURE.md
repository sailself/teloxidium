# Release Procedure (Git Tags & GitHub Releases)

Teloxidium is currently distributed via versioned Git tags (`vX.Y.Z`) and GitHub Releases rather than crates.io.

## Versioning Policy

Bump clean SemVer versions whenever merging support for a newer Telegram Bot API version or a breaking hotfix (no `-m*` tag suffixes):

- **Breaking changes (`[**BC**]`):** Bump the **minor** version (`0.x.y` → `0.(x+1).0`) for `teloxide` and `teloxide-core` (and `teloxide-macros` only when it changes).
- **Non-breaking / additive Bot API releases & fixes:** Bump the **patch** version (`0.x.y` → `0.x.(y+1)`) for `teloxide` and `teloxide-core`.
- **Git tag format:** `v<teloxide-version>` (for example, `v0.18.0`, `v0.18.1`, `v0.19.0`), matching `version` in `crates/teloxide/Cargo.toml`.

## Release Steps

1. Prepare the version bump on a feature or milestone branch from `master`:
   - Update `version` in `crates/teloxide/Cargo.toml` and `crates/teloxide-core/Cargo.toml` (plus `crates/teloxide-macros/Cargo.toml` if changed).
   - Refresh `Cargo.lock` by running `cargo check --workspace`.
   - Move entries from `## unreleased` under `## <version> - YYYY-MM-DD` in `CHANGELOG.md` and `crates/teloxide-core/CHANGELOG.md` (and update `MIGRATION_GUIDE.md` if breaking), leaving a fresh empty `## unreleased` heading at the top.
2. Open a Pull Request, wait for CI (`ci-pass`) to succeed, and merge into `master`.
3. Create and push the annotated Git tag from updated `master`:
   ```powershell
   git checkout master
   git pull --ff-only origin master
   git tag -a vX.Y.Z -m "Release vX.Y.Z"
   git push origin vX.Y.Z
   ```
4. Pushing `vX.Y.Z` triggers [`.github/workflows/release.yml`](.github/workflows/release.yml), which verifies that the tag matches `crates/teloxide/Cargo.toml`, runs the workspace verification suite, and publishes the GitHub Release with changelog notes.

## Using a Release in Other Projects

### Direct Git Tag Dependency (Recommended)

In your bot's `Cargo.toml`:

```toml
[dependencies]
teloxide = { git = "https://github.com/sailself/teloxidium", tag = "vX.Y.Z", features = ["macros"] }
```

Because `crates/teloxide/Cargo.toml` references `teloxide-core` and `teloxide-macros` via workspace `path` dependencies, Cargo resolves all three crates from the same Git tag automatically while keeping `use teloxide::...` unchanged.

### Overriding Upstream `teloxide` via `[patch.crates-io]`

If another dependency in your workspace pulls in upstream `teloxide` or `teloxide-core` from crates.io, add a `[patch.crates-io]` section in your root `Cargo.toml`:

```toml
[dependencies]
teloxide = { version = "0.18", features = ["macros"] }

[patch.crates-io]
teloxide = { git = "https://github.com/sailself/teloxidium", tag = "vX.Y.Z" }
teloxide-core = { git = "https://github.com/sailself/teloxidium", tag = "vX.Y.Z" }
teloxide-macros = { git = "https://github.com/sailself/teloxidium", tag = "vX.Y.Z" }
```

> **Note:** `[patch.crates-io]` requires the `teloxide` version requested in `[dependencies]` to be SemVer-compatible with `crates/teloxide/Cargo.toml` at that tag (for example, `version = "0.18"` when the tag is `v0.18.x`).
