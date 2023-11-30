use super::consts::*;
use super::consts::*;
use super::style::*;
use super::utils::*;
use std::env;
use tmux_interface::*;

pub fn left_accent() -> StyledTexts<'static> {
    [
        StyledText::new([Style::Bg(C_BLUE)].into(), " "),
        "  ".into(),
    ]
    .into()
}
pub fn windows() -> StyledTexts<'static> {
    let window_index = format!("{}", Variable::WindowIndex);
    let window_name = format!("{}", Variable::WindowName);
    let not_current = format!(" {window_index}{SLASH}{window_name} ");
    let current = StyledTexts::from([
        StyledText::new([Style::Fg(C_RED)].into(), LOWER_LEFT_TRIANGLE),
        StyledText::new([Style::Bg(C_RED)].into(), window_index),
        StyledText::new([Style::Fg(C_RED)].into(), UPPER_LEFT_TRIANGLE),
        format!("{window_name} ").into(),
    ]);
    [format!("#{{W:{not_current},{current}}}").into()].into()
}

fn os_icon<'a>() -> StyledTexts<'a> {
    let os = env::consts::OS;
    if os == "linux" {
        [StyledText::new([Style::Fg(C_YELLOW)].into(), LINUX_ICON).into()].into()
    } else if os == "macos" {
        [StyledText::new([Style::Fg(C_PURPLE)].into(), APPLE_ICON).into()].into()
    } else {
        ["?".into()].into()
    }
}

fn row0left() -> StyledTexts<'static> {
    StyledTexts::concat([
        left_accent(),
        os_icon(),
        [" ".into()].into(),
        green_right_arrow(format!("{}", Variable::SessionName)),
        [" ".into()].into(),
        windows(),
    ])
}
fn row0right() -> StyledTexts<'static> {
    StyledTexts::new(
        Styles::from([Style::Align(Align::Right)]),
        [
            format!("#{{{}}}@{}  ", VARIABLE_USER, Variable::Host).into(),
            StyledText::new([Style::Bg(C_RED)].into(), " "),
        ],
    )
}

pub fn row0() -> StyledTexts<'static> {
    StyledTexts::concat([row0left(), row0right()])
}
