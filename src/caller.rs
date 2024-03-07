use std::{
    fmt::{self, Display, Formatter},
    panic::Location,
};

#[derive(Debug)]
pub struct Caller {
    pub message: String,
    pub file: &'static str,
    pub line: u32,
}

impl Caller {
    pub fn new(caller: &'static Location<'static>) -> Self {
        Self {
            message: String::new(),
            file: caller.file(),
            line: caller.line(),
        }
    }

    pub fn new_with_context<S>(caller: &'static Location<'static>, message: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            message: message.into(),
            file: caller.file(),
            line: caller.line(),
        }
    }
}

#[cfg(not(feature = "color"))]
impl Display for Caller {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.message.is_empty() {
            write!(f, "{}", format!("[{}:{}]", self.file, self.line))
        } else {
            write!(
                f,
                "{} {}",
                format!("[{}:{}]", self.file, self.line),
                self.message
            )
        }
    }
}

#[cfg(feature = "color")]
impl Display for Caller {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use owo_colors::OwoColorize;
        if self.message.is_empty() {
            write!(f, "{}", format!("[{}:{}]", self.file, self.line).dimmed())
        } else {
            write!(
                f,
                "{} {}",
                format!("[{}:{}]", self.file, self.line).dimmed(),
                self.message
            )
        }
    }
}
