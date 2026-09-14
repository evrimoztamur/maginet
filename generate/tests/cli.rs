use std::process::Command;
#[test]
fn help_and_actionable_errors() {
    let run = |args: &[&str]| {
        Command::new(env!("CARGO_BIN_EXE_generate"))
            .args(args)
            .output()
            .unwrap()
    };
    let help = run(&[]);
    assert!(help.status.success());
    assert!(String::from_utf8(help.stdout).unwrap().contains("Usage:"));
    for (args, message) in [
        (vec!["unknown"], "unknown command"),
        (vec!["analyse"], "requires --code"),
        (vec!["analyse", "--code", "!"], "base32"),
        (vec!["campaign", "--games", "0"], "positive"),
        (vec!["campaign", "--workers", "0"], "positive"),
        (vec!["campaign", "--seed"], "missing value"),
        (vec!["campaign", "--seed", "-1"], "nonnegative integer"),
        (vec!["campaign", "--unknown"], "unknown option"),
        (vec!["campaign", "--code", "abc"], "belongs to analyse"),
    ] {
        let result = run(&args);
        assert_eq!(result.status.code(), Some(2));
        assert!(String::from_utf8(result.stderr).unwrap().contains(message));
    }
}
