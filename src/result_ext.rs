use std::error::Error;

use crate::ErrorTrace;

pub trait ResultExt<T>: Sized {
    #[track_caller]
    fn track(self) -> Result<T, ErrorTrace>;

    #[track_caller]
    fn context(self, context: &'static str) -> Result<T, ErrorTrace>;

    #[track_caller]
    fn with_context<F: FnOnce() -> String>(self, context: F) -> Result<T, ErrorTrace>;
}

impl<T, E> ResultExt<T> for Result<T, E>
where
    E: Error + 'static,
{
    #[track_caller]
    fn track(self) -> Result<T, ErrorTrace> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => {
                let caller = std::panic::Location::caller();
                let tracked_error = ErrorTrace::new(e, caller);
                Err(tracked_error)
            }
        }
    }

    #[track_caller]
    fn context(self, context: &'static str) -> Result<T, ErrorTrace> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => {
                let caller = std::panic::Location::caller();
                let tracked_error = ErrorTrace::new_with_context(e, caller, context);
                Err(tracked_error)
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
                let tracked_error = ErrorTrace::new_with_context(e, caller, &context);
                Err(tracked_error)
            }
        }
    }
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
