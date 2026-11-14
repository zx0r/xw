use std::process::ExitCode;

fn main() -> ExitCode {
    let argv: Vec<std::ffi::OsString> = std::env::args_os().skip(1).collect();
    let env = xw::args::Env {
        no_color: std::env::var_os("NO_COLOR").is_some_and(|v| !v.is_empty()),
        state_dir: xw::sys::state_dir(),
    };
    let cfg = match xw::args::parse(&argv, &env) {
        Ok(xw::args::Cmd::Run(cfg)) => cfg,
        Ok(xw::args::Cmd::Help) => return emit(xw::args::HELP),
        Ok(xw::args::Cmd::Version) => return emit(xw::args::VERSION),
        Err(msg) => {
            xw::sys::write_stderr(msg);
            return ExitCode::from(2);
        }
    };
    let mut out = xw::fmt::Out::new();
    let healthy = xw::render(&cfg, &mut out);
    if !xw::sys::write_stdout(out.as_bytes()) {
        return ExitCode::FAILURE;
    }
    if healthy {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn emit(msg: &[u8]) -> ExitCode {
    if xw::sys::write_stdout(msg) {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
