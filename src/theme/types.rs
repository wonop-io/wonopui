#[cfg(feature = "ThemeProvider")]
use std::rc::Rc;

use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ClassesContainer<T> {
    pub value: T,
}

#[cfg(not(feature = "ThemeProvider"))]
impl ClassesContainer<&'static str> {
    pub fn empty() -> Self {
        ClassesContainer { value: "" }
    }
}

#[cfg(feature = "ThemeProvider")]
impl ClassesContainer<String> {
    pub fn empty() -> Self {
        ClassesContainer {
            value: String::new(),
        }
    }
}

impl Copy for ClassesContainer<&'static str> {}

impl<T: AsRef<str>> ClassesContainer<T> {
    pub fn as_str(&self) -> &str {
        self.value.as_ref()
    }

    pub fn to_owned(&self) -> ClassesContainer<String> {
        ClassesContainer {
            value: self.value.as_ref().to_owned(),
        }
    }
}

impl<T: AsRef<str>> From<&ClassesContainer<T>> for yew::Classes {
    fn from(container: &ClassesContainer<T>) -> Self {
        Classes::from(container.as_str())
    }
}

impl<T: AsRef<str>> From<ClassesContainer<T>> for yew::Classes {
    fn from(container: ClassesContainer<T>) -> Self {
        Classes::from(container.as_str())
    }
}

impl<T: AsRef<str>> std::fmt::Display for ClassesContainer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(not(feature = "ThemeProvider"))]
pub type ClassesStr = ClassesContainer<&'static str>;

#[cfg(feature = "ThemeProvider")]
pub type ClassesStr = ClassesContainer<String>;