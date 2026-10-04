# veil-env

[日本語](README.md)

A Rust CLI intended to reduce manual `.env` persistence by injecting vault
values into a child process environment. This is a **mock-only prototype**;
AWS, 1Password, and veil-drop integrations are not implemented.

## Getting started

Use `nix develop path:.`, or install rustup outside Nix. Rust is pinned in
`rust-toolchain.toml`, Cargo dependencies in `Cargo.lock`, and Nix inputs in `flake.lock`.

```sh
cargo build --locked
cargo run --locked -- run --source mock:demo -- npm run dev
```

The child receives `VEIL_ENV_DEMO_SECRET=dummy-not-a-real-secret`.
The npm command is an example for an existing application; this repository
contains no Node application.

```sh
# macOS / Linux: clear inherited environment
cargo run --locked -- run --source mock:demo --clear-env -- /bin/sh -c 'test -n "$VEIL_ENV_DEMO_SECRET"'
```

Environment inheritance is the default; fetched values override matching keys.
Place the executable and arguments after `--`. No shell is added automatically.
Standard I/O is inherited directly and normal child exit codes are propagated.

## Security boundaries

The wrapper does not save fetched values to files. Values still exist in parent
and child memory. Environment inspection, swap, dumps, and child persistence or
transmission are outside this guarantee. Killing the parent does not guarantee
child termination, and explicit signal forwarding is not implemented.
Read [the security model](docs/SECURITY_MODEL.md) before using real credentials.

## Development

```sh
just lint
just test
just build
just lint-md
```

See [contributing](CONTRIBUTING.md), [guidelines](docs/GUIDELINES.md),
[target audience](docs/TARGET_AUDIENCE.md), and [private security reporting](SECURITY.md).
CI defines macOS, Linux, and Windows checks; results need separate verification.
cargo-dist distribution is deferred to a reviewed release stage with publishing approval.

MIT License.
