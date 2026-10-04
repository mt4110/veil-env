use std::process::Command;

fn wrapper() -> Command {
    Command::new(env!("CARGO_BIN_EXE_veil-env"))
}

#[test]
fn help_is_available() {
    let output = wrapper().arg("--help").output().unwrap();
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("run"));
}

#[test]
fn missing_child_is_a_usage_error() {
    let output = wrapper()
        .args(["run", "--source", "mock:demo"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn unsupported_source_is_rejected_without_echoing_it() {
    let output = wrapper()
        .args([
            "run",
            "--source",
            "aws:DO_NOT_LOG_THIS",
            "--",
            "nonexistent-child",
        ])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("unsupported secret source"));
    assert!(!stderr.contains("DO_NOT_LOG_THIS"));
    assert!(!stderr.contains("failed to start"));
}

#[test]
fn spawn_failure_is_reported_without_command_arguments() {
    let output = wrapper()
        .args([
            "run",
            "--source",
            "mock:demo",
            "--",
            "veil-env-nonexistent-executable-fixture",
            "DO_NOT_LOG_THIS",
        ])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("failed to start child process"));
    assert!(!stderr.contains("DO_NOT_LOG_THIS"));
}

#[cfg(unix)]
mod unix {
    use super::*;
    use std::{io::Write, process::Stdio};

    #[test]
    fn injects_overrides_and_inherits() {
        let output = wrapper()
            .env("VEIL_ENV_PARENT_TEST", "inherited")
            .env("VEIL_ENV_DEMO_SECRET", "old-value")
            .args(["run", "--source", "mock:demo", "--", "/bin/sh", "-c",
                "test \"$VEIL_ENV_PARENT_TEST\" = inherited && test \"$VEIL_ENV_DEMO_SECRET\" = dummy-not-a-real-secret"])
            .output().unwrap();
        assert!(output.status.success());
        assert!(output.stdout.is_empty());
        assert!(output.stderr.is_empty());
    }

    #[test]
    fn clear_environment_still_injects() {
        let output = wrapper().env("VEIL_ENV_PARENT_TEST", "inherited")
            .args(["run", "--source", "mock:demo", "--clear-env", "--", "/bin/sh", "-c",
                "test -z \"${VEIL_ENV_PARENT_TEST+x}\" && test \"$VEIL_ENV_DEMO_SECRET\" = dummy-not-a-real-secret"])
            .output().unwrap();
        assert!(output.status.success());
    }

    #[test]
    fn propagates_exit_and_argument_boundaries() {
        let output = wrapper()
            .args([
                "run",
                "--source",
                "mock:demo",
                "--",
                "/bin/sh",
                "-c",
                "test \"$1\" = 'space ; $ literal' || exit 99; exit 37",
                "fixture",
                "space ; $ literal",
            ])
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(37));
    }

    #[test]
    fn inherits_all_standard_streams() {
        let mut child = wrapper()
            .args([
                "run",
                "--source",
                "mock:demo",
                "--",
                "/bin/sh",
                "-c",
                "read -r line; printf '%s' \"$line\"; printf 'error-output' >&2",
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        let mut stdin = child.stdin.take().unwrap();
        stdin.write_all(b"input-output\n").unwrap();
        drop(stdin);
        let output = child.wait_with_output().unwrap();
        assert!(output.status.success());
        assert_eq!(output.stdout, b"input-output");
        assert_eq!(output.stderr, b"error-output");
    }

    #[test]
    fn maps_child_signal_to_shell_style_code() {
        let output = wrapper()
            .args([
                "run",
                "--source",
                "mock:demo",
                "--",
                "/bin/sh",
                "-c",
                "kill -TERM $$",
            ])
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(143));
    }
}
