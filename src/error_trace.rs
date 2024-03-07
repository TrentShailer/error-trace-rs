use std::{error::Error, fmt::Display, panic::Location};

use crate::Caller;

#[derive(Debug)]
pub struct ErrorTrace {
    pub error: Box<dyn Error>,
    pub location: Caller,
    pub callers: Vec<Caller>,
}

impl ErrorTrace {
    pub fn new<E>(error: E, caller: &'static Location<'static>) -> Self
    where
        E: Error + 'static,
    {
        Self {
            error: Box::new(error),
            location: Caller::new(caller),
            callers: vec![],
        }
    }

    pub fn new_with_context<E, S>(error: E, caller: &'static Location<'static>, context: S) -> Self
    where
        E: Error + 'static,
        S: Into<String>,
    {
        Self {
            error: Box::new(error),
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

impl<E> From<E> for ErrorTrace
where
    E: Error + 'static,
{
    #[track_caller]
    fn from(value: E) -> Self {
        let caller = std::panic::Location::caller();
        Self::new(value, caller)
    }
}

#[cfg(not(feature = "color"))]
impl Display for ErrorTrace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", "Error Trace:")?;
        writeln!(f, "{} {}", self.location, self.error)?;

        if !self.callers.is_empty() {
            for caller in self.callers.iter() {
                writeln!(f, "{}", caller)?;
            }
        }

        Ok(())
    }
}

#[cfg(feature = "color")]
impl Display for ErrorTrace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use owo_colors::OwoColorize;

        writeln!(f, "{}", "Error Trace:".red().bold())?;
        writeln!(f, "{} {}", self.location, self.error)?;

        if !self.callers.is_empty() {
            for caller in self.callers.iter() {
                writeln!(f, "{}", caller)?;
            }
        }

        Ok(())
    }
}
