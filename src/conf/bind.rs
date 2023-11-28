use std::borrow::Cow;
use tmux_interface::{BindKey, Formats, NextWindow, TmuxCommand, TmuxCommands};

fn bind<'a, S>(key: S, cmd: TmuxCommand) -> TmuxCommand<'a>
where
    S: Into<Cow<'a, str>>,
{
    BindKey::new()
        .root()
        .key(key.into())
        .command(cmd.to_string())
        .build()
}

pub fn bindkeys_command() -> TmuxCommands<'static> {
    let cmds = TmuxCommands::new();
    cmds.add_command(bind("c-u", TmuxCommand::detach_client().build()))
        .add_command(bind("c-s", TmuxCommand::choose_tree().build()))
        .add_command(bind(
            "c-t",
            TmuxCommand::new_window()
                .start_directory(format!("#{{{}}}", Formats::new().pane_current_path()))
                .build(),
        ))
        .add_command(bind("c-h", TmuxCommand::previous_window().build()))
        .add_command(bind("c-l", NextWindow::new().build()))
}
