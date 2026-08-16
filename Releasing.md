# Releasing

Maintainer notes for publishing a new version of `problem_details` to crates.io.

Releases are driven by a pushed `v*` tag. [`cargo-release`](https://github.com/crate-ci/cargo-release)
prepares the tag locally; [`.github/workflows/release.yml`](.github/workflows/release.yml)
does the publishing. Nothing is ever uploaded from a workstation — `release.toml`
sets `publish = false` for exactly that reason.

## Per-release steps

1. **Collect changelog entries.** Add them under the `## [Unreleased]` heading in
   `Changelog.md` as changes land. Mark breaking changes with ⚠️, both on the
   individual entry and appended to the heading (`## [Unreleased] ⚠️`), matching
   the existing entries.

2. **Make sure the working tree is clean.** `cargo-release` refuses to run
   otherwise, including on untracked files.

3. **Dry run.** `cargo-release` dry-runs by default:

   ```sh
   cargo release minor
   ```

   Check the printed version bump, the changelog diff, the commit message and the
   tag name. Use `patch` / `major` instead of `minor` as appropriate — the crate
   is pre-1.0, so a breaking change is a minor bump.

4. **Execute.**

   ```sh
   cargo release minor --execute
   ```

   This bumps the version in `Cargo.toml`, rewrites `## [Unreleased]` to
   `## [<version>] - <date>`, commits as `Bump version to <version>`, tags
   `v<version>`, and pushes the commit and tag to `origin`.

5. **Approve the deployment.** The tag triggers the release workflow. It first
   re-runs the full CI suite against the tagged commit, then stops at the
   `release` environment for approval. Approve it in the Actions UI, or:

   ```sh
   gh api repos/frenetisch-applaudierend/problem-details-rs/actions/runs/<run-id>/pending_deployments \
     -f state=approved -F 'environment_ids[]=<env-id>' -f comment=''
   ```

   After approval the workflow verifies the tag against the manifest version,
   publishes to crates.io, and creates a GitHub release from the changelog
   section.

6. **Add the `## [Unreleased]` heading back** to `Changelog.md` for the next
   cycle, and commit it. `cargo-release` consumes the heading, so it is gone
   after a release.

7. **Check [docs.rs](https://docs.rs/problem_details).** It builds from its own
   queue, usually within 10–20 minutes of publishing. The build log for a
   version is at `https://docs.rs/crate/problem_details/<version>/builds`.

## One-time setup

This is already configured. It is written down because it lives outside the
repository and cannot be reconstructed from the files here.

### Trusted publishing

The workflow uploads using crates.io [Trusted Publishing](https://crates.io/docs/trusted-publishing):
it exchanges the GitHub Actions OIDC identity for a short-lived registry token,
so there is no long-lived `CRATES_IO_TOKEN` secret anywhere. This requires
`id-token: write` permission on the publishing job.

The entry is registered on the crate's crates.io settings page and matches on
three values, all of which must stay in sync with the repository:

| Setting | Value |
| --- | --- |
| Repository owner | `frenetisch-applaudierend` |
| Repository name | `problem-details-rs` |
| Workflow filename | `release.yml` |
| Environment | `release` |

**Renaming or moving the release workflow breaks publishing** until the crates.io
entry is updated to match. The same goes for renaming the repository.

### The `release` environment

A GitHub Environment named `release` gates the publish job, with the maintainer
as a required reviewer. This is the only human checkpoint before an irreversible
upload — crates.io versions can be yanked, but never replaced or deleted.

## Troubleshooting

**403 at the "Authenticate with crates.io" step.** The Trusted Publishing entry
does not match the workflow. Check all four values in the table above. Nothing
has been uploaded at this point; fix the entry and re-run the job. The tag stays
valid.

**`cargo release` fails on the changelog replacement.** The replacement in
`release.toml` uses `exactly = 1`, so it fails if the `## [Unreleased]` heading
is missing (see step 6) or appears more than once. This is a deliberate loud
failure rather than a silent no-op.

**"Tag vX.Y.Z implies version X.Y.Z, but Cargo.toml says ...".** The tag and the
manifest disagree, which would publish a version nobody expects. This normally
means a tag was created by hand rather than by `cargo-release`. Delete the tag,
fix the manifest, and tag again.

**Version already exists on crates.io.** `cargo publish --dry-run` in the
`package` CI job does not check the registry, so a stale version passes CI and
only fails at the real upload. Since `cargo-release` derives the version by
bumping the manifest, this should only happen if a release was interrupted after
publishing but before pushing.

**MSRV job fails after a dependency update.** The MSRV in `Cargo.toml`
(`rust-version`) is enforced by the `msrv` CI job, which builds with
`--no-dev-deps` because the dev-dependencies require a newer toolchain than the
library. Either pin the dependency back or raise the MSRV — raising it is a
breaking change and belongs in the changelog with a ⚠️.
