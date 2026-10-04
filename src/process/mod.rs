use std::{
    collections::HashMap,
    ffi::OsString,
    process::{Command, Stdio},
};

use anyhow::{Context, Result, ensure};

pub fn run(command: &[OsString], secrets: HashMap<String, String>, clear_env: bool) -> Result<i32> {
    let (program, args) = command.split_first().context("missing child command")?;
    // Validate before spawn; errors deliberately omit names and values.
    for (key, value) in &secrets {
        ensure!(
            !key.is_empty() && !key.contains(['=', '\0']) && !value.contains('\0'),
            "invalid secret environment entry"
        );
    }
    let mut builder = Command::new(program);
    builder
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
    if clear_env {
        builder.env_clear();
    }
    builder.envs(&secrets);
    let spawned = builder.spawn();
    // Release the wrapper's owned copies before waiting. Drop is not zeroization.
    drop(builder);
    drop(secrets);
    let mut child = spawned.context("failed to start child process")?;
    let status = child.wait().context("failed to wait for child process")?;
    if let Some(code) = status.code() {
        return Ok(code);
    }
    #[cfg(unix)]
    {
        use std::os::unix::process::ExitStatusExt;
        if let Some(signal) = status.signal() {
            return Ok(128 + signal);
        }
    }
    Ok(1)
}
