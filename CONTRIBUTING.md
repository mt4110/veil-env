# Contributing

Read [the security model](docs/SECURITY_MODEL.md) before changing secret handling.
Use dummy credentials for development, tests, issues, and pull requests.

## Development

Run `nix develop path:.` for the locked development shell, then `just check`.
Outside Nix, install Rust through rustup and run `just lint`, `just test`, and
`just build`. Markdown lint additionally requires `markdownlint-cli2`.
Commit `Cargo.lock` and `flake.lock` when dependencies intentionally change.

## Pull requests

Keep changes focused. Explain the problem, resulting behavior, validation, and
remaining limitations. Run formatting and Clippy with warnings denied.
Security changes need regression tests covering failures as well as success.
Do not weaken tests to make them pass.

## Dependencies

Prefer the standard library and existing dependencies. Before adding a crate,
check necessity, maintenance, security advisories, licensing, and platform cost.
`veil-rs` is not currently a dependency; adopting it requires a concrete need
and a separate design decision. Do not add a vault SDK until its provider scope,
authentication, timeout, response validation, and logging policy are agreed.
