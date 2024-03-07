use std::panic::Location;

use crate::Caller;

#[derive(Debug)]
pub struct ErrorTrace {
    pub error: String,
    pub location: Caller,
    pub callers: Vec<Caller>,
}

impl ErrorTrace {
    pub fn new<E>(error: E, caller: &'static Location<'static>) -> Self
    where
        E: Into<String>,
    {
        Self {
            error: error.into(),
            location: Caller::new(caller),
            callers: vec![],
        }
    }

    pub fn new_with_context<E, S>(error: E, caller: &'static Location<'static>, context: S) -> Self
    where
        E: Into<String>,
        S: Into<String>,
    {
        Self {
            error: error.into(),
            location: Caller::new(caller),
            callers: vec![Caller::new_with_context(caller, context)],
        }
    }

    pub fn add_caller(self, caller: &'static Location<'static>) -> Self {
        let mut callers = self.callers;
        callers.push(Caller::new(caller));

        Self {
            error: self.error,
            location: self.location,
            callers,
        }
    }

    pub fn add_caller_with_context<S>(self, caller: &'static Location<'static>, context: S) -> Self
    where
        S: Into<String>,
    {
        let mut callers = self.callers;
        callers.push(Caller::new_with_context(caller, context));

        Self {
            error: self.error,
            location: self.location,
            callers,
        }
    }
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(not(feature = "color"))]
impl ToString for ErrorTrace {
    fn to_string(&self) -> String {
        let mut output = String::new();

        output.push_str("Error Trace:\n");
        output.push_str(&format!("App Version: {}\n", VERSION));
        output.push_str(&format!("{} {}\n", self.location, self.error));

        if !self.callers.is_empty() {
            for caller in self.callers.iter() {
                output.push_str(&format!("{}\n", caller));
            }
        }

        output
    }
}

#[cfg(feature = "color")]
impl ToString for ErrorTrace {
    fn to_string(&self) -> String {
        use owo_colors::OwoColorize;
        let mut output = String::new();

        output.push_str(&"Error Trace:\n".red().bold().to_string());
        output.push_str(&format!("{} {}\n", "App Version".dimmed(), VERSION));
        output.push_str(&format!("{} {}\n", self.location, self.error));

        if !self.callers.is_empty() {
            for caller in self.callers.iter() {
                output.push_str(&format!("{}\n", caller));
            }
        }

        output
    }
}
