use std::process::Command;

#[test]
fn test_cli_all_widgets_default_icons() {
    let output = Command::new(env!("CARGO_BIN_EXE_xw"))
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains('\u{f035b}'));
    assert!(stdout.contains('\u{efc5}'));
    assert!(stdout.contains('\u{f02ca}'));
    assert!(stdout.contains('\u{f176}'));
}

#[test]
fn test_cli_all_widgets_no_icons() {
    let output = Command::new(env!("CARGO_BIN_EXE_xw"))
        .arg("--no-icons")
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("CPU"));
    assert!(stdout.contains("MEM"));
    assert!(stdout.contains("SSD"));
    assert!(stdout.contains("UP"));
}

#[test]
fn test_cli_individual_widgets() {
    let output = Command::new(env!("CARGO_BIN_EXE_xw"))
        .arg("cpu")
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains('\u{f035b}'));
}

#[test]
fn test_cli_uptime_widget() {
    let output = Command::new(env!("CARGO_BIN_EXE_xw"))
        .arg("uptime")
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains('\u{f176}'));
}

#[test]
fn test_cli_git_widget() {
    let output = Command::new(env!("CARGO_BIN_EXE_xw"))
        .arg("git")
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains('\u{f418}'));
}

#[test]
fn test_cli_git_status_detailed() {
    let output = Command::new(env!("CARGO_BIN_EXE_xw"))
        .args(["git", "-G"])
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains('\u{f418}'));
}

#[test]
fn test_cli_git_with_path_arg() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let output = Command::new(env!("CARGO_BIN_EXE_xw"))
        .args(["git", manifest_dir])
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains('\u{f418}'));
}

#[test]
fn test_cli_no_color_has_no_escape_codes() {
    let output = Command::new(env!("CARGO_BIN_EXE_xw"))
        .arg("--no-color")
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());
    assert!(!output.stdout.contains(&0x1b));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("CPU"));
    assert!(stdout.contains("MEM"));
    assert!(stdout.contains("SSD"));
    assert!(stdout.contains("UP"));
}

#[test]
fn test_cli_custom_separator() {
    let output = Command::new(env!("CARGO_BIN_EXE_xw"))
        .args(["--separator", " :: ", "cpu", "mem"])
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains(" :: "));
}

#[test]
fn test_cli_version_flag() {
    let output = Command::new(env!("CARGO_BIN_EXE_xw"))
        .arg("-V")
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.starts_with("xw "));
}

#[test]
fn test_cli_help_flag() {
    let output = Command::new(env!("CARGO_BIN_EXE_xw"))
        .arg("-h")
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("WIDGETS:"));
    assert!(stdout.contains("OPTIONS:"));
}

#[test]
fn test_cli_tmux_style() {
    let output = Command::new(env!("CARGO_BIN_EXE_xw"))
        .args(["-s", "tmux", "mem"])
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("#[fg=colour"));
}

#[test]
fn test_cli_ansi_style() {
    let output = Command::new(env!("CARGO_BIN_EXE_xw"))
        .args(["-s", "ansi", "mem"])
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());
    assert!(output.stdout.contains(&0x1b));
}

#[test]
fn test_cli_plain_style() {
    let output = Command::new(env!("CARGO_BIN_EXE_xw"))
        .args(["-s", "plain", "mem"])
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());
    assert!(!output.stdout.contains(&0x1b));
    assert!(!output.stdout.windows(5).any(|w| w == b"#[fg="));
}

#[test]
fn test_cli_bytes_flag() {
    let output = Command::new(env!("CARGO_BIN_EXE_xw"))
        .args(["--bytes", "mem"])
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains('/') && (stdout.contains('G') || stdout.contains('M')));
}

#[test]
fn test_cli_invalid_arg() {
    let output = Command::new(env!("CARGO_BIN_EXE_xw"))
        .arg("--unknown-flag")
        .output()
        .expect("failed to execute process");

    assert!(!output.status.success());
}

#[test]
fn test_lib_render_in_process() {
    let mut out = xw::fmt::Out::new();
    let temp = std::env::temp_dir();
    let cfg = xw::Config {
        widgets: xw::Widgets::ALL,
        dialect: xw::fmt::Dialect::Plain,
        icons: false,
        bytes: false,
        git_detail: false,
        sep: b" | ",
        path: None,
        state_dir: Some(temp.as_os_str()),
    };
    let ok = xw::render(&cfg, &mut out);
    assert!(ok);
    assert_ne!(out.as_bytes(), []);
}

#[test]
fn test_out_buffer_overflow() {
    let mut out = xw::fmt::Out::new();
    let huge = [b'a'; xw::fmt::OUT_CAP + 10];
    out.push(&huge);
    assert!(out.full());
    let mark = out.mark();
    out.rollback(mark);
    assert!(!out.full());
}
