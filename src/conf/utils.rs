use super::consts::STYLE_DEFAULT;
use std::borrow::Cow;
use std::fmt;
use tmux_interface::{Style, StyleList};

#[derive(Clone)]
pub struct Styles<'a> {
    style_list: StyleList<'a>,
}

impl<'a> Styles<'a> {
    pub fn new(style_list: StyleList<'a>) -> Self {
        Self { style_list }
    }
}
impl<I> From<I> for Styles<'_>
where
    I: IntoIterator<Item = Style>,
{
    fn from(styles: I) -> Self {
        let mut list = StyleList::new();
        for s in styles.into_iter() {
            list.add(s);
        }
        Self::new(list)
    }
}
impl fmt::Display for Styles<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.style_list)
    }
}
impl<'a> From<Styles<'a>> for String {
    fn from(value: Styles<'a>) -> Self {
        format!("{}", value)
    }
}

#[derive(Clone)]
pub struct StyledText<'a> {
    styles: Styles<'a>,
    text: Cow<'a, str>,
}

impl<'a> StyledText<'a> {
    pub fn new<T>(styles: Styles<'a>, text: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Self {
            styles,
            text: text.into(),
        }
    }
}
impl<'a, T> From<T> for StyledText<'a>
where
    T: Into<Cow<'a, str>>,
{
    fn from(value: T) -> Self {
        Self {
            styles: [].into(),
            text: value.into(),
        }
    }
}
impl fmt::Display for StyledText<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "#[{}]#[{}]{}#[{}]#[{}]",
            Style::PushDefault,
            self.styles,
            self.text,
            STYLE_DEFAULT,
            Style::PopDefault
        )
    }
}
impl<'a> From<StyledText<'a>> for String {
    fn from(value: StyledText<'a>) -> Self {
        format!("{}", value)
    }
}

#[derive(Clone)]
pub struct StyledTexts<'a> {
    styles: Option<Styles<'a>>,
    styled_texts: Vec<StyledText<'a>>,
}

impl<'a> StyledTexts<'a> {
    pub fn new<S, I>(styles: S, styled_texts: I) -> Self
    where
        S: Into<Styles<'a>>,
        I: IntoIterator<Item = StyledText<'a>>,
    {
        Self {
            styles: Some(styles.into()),
            styled_texts: styled_texts.into_iter().collect(),
        }
    }
    pub fn concat<I>(i: I) -> Self
    where
        I: IntoIterator<Item = StyledTexts<'a>>,
    {
        let mut styled_texts = Vec::new();
        for v in i.into_iter() {
            styled_texts.extend(v.styled_texts);
        }
        Self {
            styles: None,
            styled_texts,
        }
    }
}
impl<'a, I> From<I> for StyledTexts<'a>
where
    I: IntoIterator<Item = StyledText<'a>>,
{
    fn from(value: I) -> Self {
        Self {
            styles: None,
            styled_texts: value.into_iter().collect(),
        }
    }
}
impl fmt::Display for StyledTexts<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ss) = &self.styles {
            write!(
                f,
                "#[{}]#[{}]#[{}]",
                Style::PushDefault,
                ss,
                Style::PushDefault
            )?;
        }
        for st in self.styled_texts.iter() {
            write!(f, "{}", st)?;
        }
        if self.styles.is_some() {
            write!(
                f,
                "#[{}]#[{}]#[{}]",
                Style::PopDefault,
                STYLE_DEFAULT,
                Style::PopDefault
            )?;
        }
        Ok(())
    }
}
impl<'a> From<StyledTexts<'a>> for String {
    fn from(value: StyledTexts<'a>) -> Self {
        format!("{}", value)
    }
}
