use super::consts::*;
use super::utils::*;
use std::borrow::Cow;
use std::env;
use tmux_interface::*;

pub fn set_status() -> TmuxCommands<'static> {
    let default = Styles::from([Style::Bg(C_DEFAULT_BG), Style::Fg(C_DEFAULT_FG)]);
    let current_exe = format!("{}", env::current_exe().unwrap().display());
    let row0 = format!("#({} status 0)", current_exe);
    let row1 = format!("#({} status 1)", current_exe);
    SetGlobalSessionOptions::new()
        .status(None::<N>, Some(Status::TwoRows))
        .status_style(None::<N>, Some(format!("{}", default)))
        .status_interval(None::<N>, Some(1))
        .status_format(None::<N>, Some([row0, row1]))
        .build()
}

fn green_right_arrow<'a, S>(s: S) -> StyledText<'a>
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

fn left_accent() -> StyledText<'static> {
    [
        StyledText::styled([Style::Bg(C_BLUE)], " "),
        StyledText::raw("  "),
    ]
    .into()
}
fn right_accent() -> StyledText<'static> {
    StyledText::styled([Style::Bg(C_RED)], " ")
}
fn windows() -> StyledText<'static> {
    let window_index = format!("{}", Variable::WindowIndex);
    let window_name = format!("{}", Variable::WindowName);
    let not_current = format!(" {window_index}{SLASH}{window_name} ");
    let current = StyledText::from([
        StyledText::styled([Style::Fg(C_RED)], LOWER_LEFT_TRIANGLE),
        StyledText::styled([Style::Bg(C_RED)], window_index),
        StyledText::styled([Style::Fg(C_RED)], UPPER_LEFT_TRIANGLE),
        StyledText::raw(format!("{window_name} ")),
    ]);
    [StyledText::raw(format!("#{{W:{not_current},{current}}}"))].into()
}

fn os_icon<'a>() -> StyledText<'a> {
    let os = env::consts::OS;
    if os == "linux" {
        [StyledText::styled([Style::Fg(C_YELLOW)], LINUX_ICON)].into()
    } else if os == "macos" {
        [StyledText::styled([Style::Fg(C_PURPLE)], APPLE_ICON)].into()
    } else {
        [StyledText::raw("?")].into()
    }
}

fn row0left() -> StyledText<'static> {
    [
        left_accent(),
        os_icon(),
        StyledText::raw(" "),
        green_right_arrow(format!("{}", Variable::SessionName)),
        StyledText::raw(" "),
        windows(),
    ]
    .into()
}
fn row0right() -> StyledText<'static> {
    StyledText::new(
        [Style::Align(Align::Right)],
        [
            StyledText::raw(format!("#{{{}}}@{}  ", VARIABLE_USER, Variable::Host)),
            right_accent(),
        ],
    )
}

pub fn row0() -> StyledText<'static> {
    [row0left(), row0right()].into()
}
pub fn row1() -> StyledText<'static> {
    [left_accent(), right_accent()].into()
}
