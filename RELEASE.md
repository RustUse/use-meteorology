# Release Policy

RustUse/use-meteorology keeps root workspace metadata at `publish = false` by default, while the current focused crate manifests and facade manifest opt in with `publish = true`. The release task is to verify that only the intended crates remain publishable.

The focused crates are publish-independent in v0, while the facade depends on every focused crate. The first publish wave therefore uses a fixed operational order and still publishes the facade last.

## First Publish Wave

Publish focused crates first:

1. `use-weather-observation`
2. `use-atmosphere`
3. `use-air-temperature`
4. `use-atmospheric-pressure`
5. `use-humidity`
6. `use-wind`
7. `use-cloud`
8. `use-precipitation`
9. `use-weather-front`
10. `use-pressure-system`
11. `use-weather-forecast`

After the focused crates are visible in the crates.io index, publish the facade crate:

12. `use-meteorology`

## Publish Surface

Before the first publish wave, confirm that the release surface:

- keeps the workspace-level default at `publish = false`
- keeps every focused crate under `crates/` at `publish = true`
- keeps `crates/use-meteorology/Cargo.toml` at `publish = true`
- leaves future non-release crates opted out until they are intentionally reviewed

## Versioning

- The workspace currently uses lockstep `0.x.y` versioning.
- Before `1.0`, breaking changes should bump the minor version.
- Before `1.0`, additive compatible changes should bump the patch version.
- The facade crate should only advertise actively supported crates and modules.

## Automated Release Validation

The repository includes a release-validation path:

- `.github/workflows/publish-readiness.yml` runs on pull requests, pushes to `main`, and manual dispatch.
- `make release-readiness` runs the same high-value local checks for examples, no-default-features coverage, and focused-crate publish dry-runs.
- `.github/workflows/facade-publish-readiness.yml` is a manual post-publication check that dry-runs `use-meteorology` after the focused crates are live on crates.io.

## Branch Protection Gate

Before the first public release, the canonical GitHub repository should require `Publish Readiness / Release Readiness Checks` on `main`.

This repository can document the required check name, but it cannot enforce branch protection from version-controlled files alone. Set the rule in the GitHub branch protection or ruleset UI before the first crates.io publish.

## Version and Changelog Automation

The repository includes `release-plz` configuration in `release-plz.toml` and maintainer workflows under `.github/workflows/release-plz-*.yml`.

- `Release PR Automation` opens or updates a release PR with lockstep version changes for every publishable crate in the workspace.
- The workspace is configured with one `version_group` so all published crates keep the same version.
- The root `CHANGELOG.md` remains the shared changelog and is updated through the `use-meteorology` package entry, including focused-crate commits.
- `Release Publish Automation` can publish automatically on pushes to `main` after the initial manual wave is complete, crates.io trusted publishing is configured for every published crate, and the `CRATES_IO_AUTOPUBLISH_ENABLED` repository variable is set to `true`.

One-time post-initial-release setup:

- Configure crates.io Trusted Publishing for each published crate with repository owner `RustUse`, repository name `use-meteorology`, and workflow filename `release-plz-release.yml`.
- Leave the crates.io environment field empty unless a matching GitHub Actions environment is intentionally added later.
- Set the repository variable `CRATES_IO_AUTOPUBLISH_ENABLED` to `true` only after the initial manual crates.io wave is complete.
- Do not set `CARGO_REGISTRY_TOKEN` for this workflow when using trusted publishing.

## Maintainer Release Checklist

For normal post-initial-release releases:

1. Merge ordinary PRs with clean final commit subjects or squash titles that match `type: summary` or `type(scope)!: summary`.
2. Let `Release PR Automation` open or update the release PR.
3. Review the release PR for the lockstep version bump, the generated root `CHANGELOG.md`, and any low-signal fallback entries under `Changed`.
4. Clean up the changelog directly in the release PR branch when needed.
5. Merge the release PR after the required checks pass.
6. Let the push-triggered `Release Publish Automation` run on the merged release commit, or manually dispatch it with `post-initial-release = true` if a controlled rerun is needed.
7. Verify the published crates, docs.rs pages, and release tags or artifacts after the workflow completes.

For the initial public crates.io wave:

1. Do not use `Release Publish Automation` yet.
2. Run the full release-readiness path.
3. Publish focused crates in the documented first-wave order.
4. Wait for crates.io index propagation.
5. Run the facade publish readiness workflow or `cargo publish --dry-run --allow-dirty -p use-meteorology`.
6. Publish `use-meteorology`.

## Publish Readiness Checklist

1. Confirm `cargo fmt --all -- --check` passes.
2. Confirm `cargo check --workspace --all-features` passes.
3. Confirm `cargo check --workspace --all-features --examples` passes.
4. Confirm `cargo test --workspace --all-features` passes.
5. Confirm `cargo test --workspace --no-default-features` passes.
6. Confirm `cargo clippy --workspace --all-targets --all-features -- -D warnings` passes.
7. Confirm `cargo deny check` and `cargo audit` pass.
8. Review README examples, crate metadata, repository health files, `Cargo.lock`, and changelog entries.
9. Confirm focused crates plus `use-meteorology` are the only intentionally publishable crates.
10. Publish all focused crates, then wait for crates.io index resolution.
11. Publish the facade crate manually for the first wave.
