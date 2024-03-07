use std::fmt::Display;

use crate::ErrorTrace;

pub trait ResultExt<T>: Sized {
    #[track_caller]
    fn track(self) -> Result<T, ErrorTrace>;

    #[track_caller]
    fn context(self, context: &'static str) -> Result<T, ErrorTrace>;

    #[track_caller]
    fn with_context<F: FnOnce() -> String>(self, context: F) -> Result<T, ErrorTrace>;
}

impl<T> ResultExt<T> for Result<T, ErrorTrace> {
    #[track_caller]
    fn track(self) -> Result<T, ErrorTrace> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => {
                let caller = std::panic::Location::caller();
                Err(e.add_caller(caller))
            }
        }
    }

    #[track_caller]
    fn context(self, context: &'static str) -> Result<T, ErrorTrace> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => {
                let caller = std::panic::Location::caller();
                Err(e.add_caller_with_context(caller, context))
            }
        }
    }

    #[track_caller]
    fn with_context<F: FnOnce() -> String>(self, context: F) -> Result<T, ErrorTrace> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => {
                let caller = std::panic::Location::caller();
                let context = context();
                Err(e.add_caller_with_context(caller, &context))
            }
        }
    }
}

impl<T, E> ResultExt<T> for Result<T, E>
where
    E: Display,
{
    #[track_caller]
    fn track(self) -> Result<T, ErrorTrace> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => {
                let caller = std::panic::Location::caller();
                let trace = ErrorTrace::new(format!("{}", e), caller);
                Err(trace)
            }
        }
    }

    #[track_caller]
    fn context(self, context: &'static str) -> Result<T, ErrorTrace> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => {
                let caller = std::panic::Location::caller();
                let trace = ErrorTrace::new_with_context(format!("{}", e), caller, context);
                Err(trace)
            }
        }
    }

    #[track_caller]
    fn with_context<F: FnOnce() -> String>(self, context: F) -> Result<T, ErrorTrace> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => {
                let caller = std::panic::Location::caller();
                let context = context();
                let trace = ErrorTrace::new_with_context(format!("{}", e), caller, context);
                Err(trace)
            }
        }
    }
}
