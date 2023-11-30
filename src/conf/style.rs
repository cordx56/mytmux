use super::consts::*;
use super::utils::*;
use std::borrow::Cow;
use std::env;
use tmux_interface::*;

pub fn green_right_arrow<'a, S>(s: S) -> StyledTexts<'a>
where
    S: Into<Cow<'a, str>>,
{
    let s = s.into();
    [
        StyledText::new(
            [Style::Bg(C_GREEN), Style::Fg(C_DEFAULT_BG)].into(),
            format!("{RIGHT_TRIANGLE} "),
        ),
        StyledText::new(
            [Style::Bg(C_GREEN), Style::Fg(C_DEFAULT_FG)].into(),
            format!("{s} "),
        ),
        StyledText::new(
            [Style::Bg(C_DEFAULT_BG), Style::Fg(C_GREEN)].into(),
            RIGHT_TRIANGLE,
        ),
    ]
    .into()
}

pub fn options_command() -> TmuxCommands<'static> {
    let default = Styles::from([Style::Bg(C_DEFAULT_BG), Style::Fg(C_DEFAULT_FG)]);
    let current_exe = format!("{}", env::current_exe().unwrap().display());
    let row0 = format!("#({} status 0)", current_exe);
    //let row1 = format!("#({} status 0)", current_exe);
    let tn: Option<Cow<'static, str>> = None;
    SetGlobalSessionOptions::new()
        .status(tn.clone(), Some(Status::TwoRows))
        .status_style(tn.clone(), Some(format!("{}", default)))
        .status_interval(tn.clone(), Some(1))
        .status_format(tn.clone(), Some([row0]))
        .build()
}
