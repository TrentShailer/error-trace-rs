use crate::ErrorTrace;

pub trait OptionExt<T>: Sized {
    #[track_caller]
    fn track(self) -> Result<T, ErrorTrace>;

    #[track_caller]
    fn context(self, context: &'static str) -> Result<T, ErrorTrace>;

    #[track_caller]
    fn with_context<F: FnOnce() -> String>(self, context: F) -> Result<T, ErrorTrace>;

    #[cfg(feature = "log")]
    #[track_caller]
    fn log_none(self) -> Result<T, ErrorTrace>;
}

impl<T> OptionExt<T> for Option<T> {
    #[track_caller]
    fn track(self) -> Result<T, ErrorTrace> {
        match self {
            Some(v) => Ok(v),
            None => {
                let caller = std::panic::Location::caller();
                let tracked_error = ErrorTrace::new("Option was None, expected Some", caller);
                Err(tracked_error)
            }
        }
    }

    #[track_caller]
    fn context(self, context: &'static str) -> Result<T, ErrorTrace> {
        match self {
            Some(v) => Ok(v),
            None => {
                let caller = std::panic::Location::caller();
                let tracked_error = ErrorTrace::new(context, caller);
                Err(tracked_error)
            }
        }
    }

    #[track_caller]
    fn with_context<F: FnOnce() -> String>(self, context: F) -> Result<T, ErrorTrace> {
        match self {
            Some(v) => Ok(v),
            None => {
                let caller = std::panic::Location::caller();
                let context = context();
                let tracked_error = ErrorTrace::new(context, caller);
                Err(tracked_error)
            }
        }
    }

    #[cfg(feature = "log")]
    #[track_caller]
    fn log_none(self) -> Result<T, ErrorTrace> {
        match self {
            Some(v) => Ok(v),
            None => {
                let caller = std::panic::Location::caller();
                let tracked_error = ErrorTrace::new("Option was None, expected Some", caller);
                log::error!("{}", tracked_error.to_string());
                Err(tracked_error)
            }
        }
    }
}
