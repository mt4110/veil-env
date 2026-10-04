# Security model

## Intended guarantee

The wrapper does not create a `.env` file or deliberately write fetched values
to disk or logs. `Command::envs` supplies environment entries when starting the
child. Standard input, output, and error are inherited directly.
The executable is launched directly without adding a shell.

## Memory lifetime

Fetched strings exist in the parent, and `Command` stores its own environment
entries. The child receives a separate environment. The wrapper drops its
owned map and command builder immediately after the spawn attempt.
Dropping a Rust `String` releases ownership; it does not guarantee memory wiping.
There is no locked-memory or zeroization implementation in this prototype.

OS process teardown releases that process's address space. It does not prove
physical erasure of secret bytes or removal from swap, hibernation images,
crash dumps, debugger captures, or other processes' copies. Therefore this
project does not claim that plaintext can never reach disk at the OS level.

## Trust boundary

The child and its descendants can read, print, save, or transmit injected values.
Environment inheritance can also expose unrelated parent credentials.
Use only trusted executables; `--clear-env` is an explicit compatibility tradeoff.
Users with appropriate OS privileges may inspect process memory or environments.
This tool is not a sandbox, DLP system, or defense against a compromised endpoint.

## Exit and signals

Normal child exit codes are returned unchanged. On Unix, child signal termination
is represented as `128 + signal`; this does not reproduce signal termination of
the wrapper itself. CLI usage errors return 2 and wrapper errors return 1.
Windows child codes are passed as an i32 to the process exit API.

There is no explicit signal forwarding, process-group management, parent-death
hook, or guaranteed descendant cleanup. Terminal-generated signals may reach
both processes through their shared foreground process group; a signal sent
only to the parent need not reach the child. SIGKILL cannot run a cleanup handler.
Killing the parent does not guarantee child termination or secret destruction.

## Providers and logs

Only `mock:demo` returns fixed dummy values. Every other source fails before
launching the child; there is no silent mock fallback. Source identifiers,
command arguments, secret names, and values are not logged by the wrapper.
Child output is transparent and may itself contain sensitive information.
Future providers must sanitize error output and validate environment payloads.

## References

- [Rust Command environment and process API](https://doc.rust-lang.org/std/process/struct.Command.html)
- [Rust Child lifecycle](https://doc.rust-lang.org/std/process/struct.Child.html)
- [Rust ExitStatus](https://doc.rust-lang.org/std/process/struct.ExitStatus.html)
