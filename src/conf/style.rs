use super::consts::*;
use super::utils::*;
use std::borrow::Cow;
use std::env;
use tmux_interface::*;

pub fn green_right_arrow<'a, S>(s: S) -> StyledText<'a>
where
    S: Into<Cow<'a, str>>,
{
    let s = s.into();
    [
        StyledText::styled(
            [Style::Bg(C_GREEN), Style::Fg(C_DEFAULT_BG)],
            format!("{RIGHT_TRIANGLE} "),
        ),
        StyledText::styled(
            [Style::Bg(C_GREEN), Style::Fg(C_DEFAULT_FG)],
            format!("{s} "),
        ),
        StyledText::styled(
            [Style::Bg(C_DEFAULT_BG), Style::Fg(C_GREEN)],
            RIGHT_TRIANGLE,
        ),
    ]
    .into()
}

pub fn options_command() -> TmuxCommands<'static> {
    let default = Styles::from([Style::Bg(C_DEFAULT_BG), Style::Fg(C_DEFAULT_FG)]);
    let current_exe = format!("{}", env::current_exe().unwrap().display());
    let row0 = format!("#({} status 0)", current_exe);
    let row1 = format!("#({} status 1)", current_exe);
    let tn: Option<Cow<'static, str>> = None;
    SetGlobalSessionOptions::new()
        .status(tn.clone(), Some(Status::TwoRows))
        .status_style(tn.clone(), Some(format!("{}", default)))
        .status_interval(tn.clone(), Some(1))
        .status_format(tn.clone(), Some([row0, row1]))
        .build()
}
