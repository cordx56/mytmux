use super::utils::*;
use std::borrow::Cow;
use std::env;
use tmux_interface::*;

pub fn set_opts() -> TmuxCommands<'static> {
    let mut r = TmuxCommands::new();
    let shell = env::var("SHELL").unwrap_or("/bin/bash".into());
    for s in SetGlobalSessionOptions::new()
        .default_shell(None::<N>, Some(shell))
        .mouse(None::<N>, Some(Switch::On))
        .build()
        .into_cmds()
    {
        r.push(s);
    }
    for s in SetServerOptions::new()
        .default_terminal(Some("xterm-256color"))
        .build()
        .into_cmds()
    {
        r.push(s);
    }
    r.push(
        SetOption::new()
            .append()
            .global()
            .option(TERMINAL_OVERRIDES)
            .value(",xterm-256color:Tc")
            .build(),
    );
    r
}
