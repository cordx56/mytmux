use super::consts::STYLE_DEFAULT;
use std::borrow::Cow;
use std::fmt;
use tmux_interface::{Style, StyleList};

#[derive(Debug, Clone)]
pub struct Styles<'a> {
    style_list: StyleList<'a>,
}

impl<'a> Styles<'a> {
    pub fn new(style_list: StyleList<'a>) -> Self {
        Self { style_list }
    }
    pub fn len(&self) -> usize {
        if let Some(v) = &self.style_list.styles {
            v.len()
        } else {
            0
        }
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

#[derive(Debug, Clone)]
pub enum StyledText<'a> {
    Styled(Styles<'a>, Vec<StyledText<'a>>),
    Raw(Cow<'a, str>),
}

impl<'a> StyledText<'a> {
    pub fn new<S, T>(styles: S, styled_texts: T) -> Self
    where
        S: Into<Styles<'a>>,
        T: Into<StyledText<'a>>,
    {
        Self::Styled(styles.into(), vec![styled_texts.into()])
    }
    pub fn styled<S, T>(styles: S, raw: T) -> Self
    where
        S: Into<Styles<'a>>,
        T: Into<Cow<'a, str>>,
    {
        Self::Styled(styles.into(), vec![Self::raw(raw)])
    }
    pub fn raw<T>(raw: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        Self::Raw(raw.into())
    }
}
impl<'a, I, S> From<I> for StyledText<'a>
where
    S: Into<StyledText<'a>>,
    I: IntoIterator<Item = S>,
{
    fn from(value: I) -> Self {
        Self::Styled([].into(), value.into_iter().map(|st| st.into()).collect())
    }
}
impl fmt::Display for StyledText<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Styled(styles, styled_texts) => {
                let is_styled = if styles.len() == 0 { false } else { true };
                if is_styled {
                    write!(
                        f,
                        "#[{}]#[{}]#[{}]",
                        Style::PushDefault,
                        styles,
                        Style::PushDefault
                    )?;
                }
                for st in styled_texts {
                    write!(f, "{}", st)?;
                }
                if is_styled {
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
            Self::Raw(raw) => {
                write!(f, "{}", raw)
            }
        }
    }
}
