# Development and operation guidelines

## Current scope

This is a mock-only prototype. It does not contact AWS, 1Password, or veil-drop.
The goal of this stage is to verify command parsing, environment injection,
transparent terminal I/O, and exit-code propagation using dummy data.
Stop this stage when those checks and the development setup are reviewable.
Real provider integration and distribution require a separate scope decision.

## Operation

Use `mock:demo` with a trusted executable. Environment inheritance is the default
for compatibility; injected keys override inherited keys. `--clear-env` removes
inherited variables, so specify an absolute executable path and provide any
required application environment explicitly in a future provider.
Never put real secrets in CLI arguments, logs, issue reports, or test fixtures.
The child retains responsibility for its own logging, files, and descendants.

## Next gates

Before real credentials: decide provider authentication, timeouts, payload schema,
allowed environment keys, memory handling, and parent/child signal behavior.
Before a release: validate supported operating systems, private reporting access,
and distribution metadata; then prepare cargo-dist with explicit publishing approval.
Nix pins dependencies, but does not prove bit-for-bit output reproducibility or
that a compromised dependency is safe.
