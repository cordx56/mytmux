use std::borrow::Cow;
use std::fmt;
use tmux_interface::{Style, StyleList};

pub struct Styles<'a> {
    style_list: StyleList<'a>,
}

impl<'a> Styles<'a> {
    pub fn new(style_list: StyleList<'a>) -> Self {
        Self { style_list }
    }
    pub fn from<I>(styles: I) -> Self
    where
        I: IntoIterator<Item = Style>,
    {
        let mut list = StyleList::new();
        for s in styles.into_iter() {
            list.add(s);
        }
        Self::new(list)
    }
}
impl fmt::Display for Styles<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#[{}]", self.style_list)
    }
}

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
impl fmt::Display for StyledText<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.styles, self.text)
    }
}

pub struct StyledTexts<'a> {
    styled_texts: Vec<StyledText<'a>>,
}

impl<'a> StyledTexts<'a> {
    pub fn new<I>(styled_texts: I) -> Self
    where
        I: IntoIterator<Item = StyledText<'a>>,
    {
        Self {
            styled_texts: styled_texts.into_iter().collect(),
        }
    }
}
impl fmt::Display for StyledTexts<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for st in self.styled_texts.iter() {
            write!(f, "{}", st)?;
        }
        Ok(())
    }
}
