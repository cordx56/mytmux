use std::borrow::Cow;
use tmux_interface::*;

pub struct Binds<'a> {
    bind_cmds: Vec<TmuxCommand<'a>>,
}

impl<'a> Binds<'a> {
    pub fn new() -> Self {
        Self {
            bind_cmds: Vec::new(),
        }
    }
    pub fn bind<S, C>(&mut self, key: S, cmd: C) -> &mut Self
    where
        S: Into<Cow<'a, str>>,
        C: Into<TmuxCommand<'a>>,
    {
        self.bind_cmds.push(
            BindKey::new()
                .root()
                .key(key.into())
                .command(cmd.into().to_string())
                .build(),
        );
        self
    }
    pub fn build(&self) -> TmuxCommands<'a> {
        let mut cmds = TmuxCommands::new();
        for c in self.bind_cmds.iter() {
            cmds = cmds.add_command(c.clone());
        }
        cmds
    }
}

pub fn bindkeys_command() -> TmuxCommands<'static> {
    let pane_current_path = format!("#{{{}}}", Formats::new().pane_current_path());
    Binds::new()
        .bind("c-u", DetachClient::new())
        .bind("c-s", ChooseTree::new())
        .bind(
            "c-t",
            NewWindow::new().start_directory(pane_current_path.clone()),
        )
        .bind("c-h", PreviousWindow::new())
        .bind("c-l", NextWindow::new())
        .bind(
            "c-p",
            SplitWindow::new()
                .vertical()
                .start_directory(pane_current_path.clone()),
        )
        .bind(
            "c-v",
            SplitWindow::new()
                .horizontal()
                .start_directory(pane_current_path.clone()),
        )
        .bind("s-left", SelectPane::new().left())
        .bind("s-right", SelectPane::new().right())
        .bind("s-up", SelectPane::new().up())
        .bind("s-down", SelectPane::new().down())
        .build()
}
