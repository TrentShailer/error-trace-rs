use error_trace::{ErrorTrace, ResultExt, StringError};

#[test]
fn string_error_trace() {
    if let Err(e) = string_err_caller().track() {
        assert_eq!(
            e.to_string(),
            String::from(
                r"Error Trace:
[tests\string_error.rs:26] Some message
[tests\string_error.rs:22]
[tests\string_error.rs:5]
"
            )
        )
    } else {
        unreachable!()
    }
}

fn string_err_caller() -> Result<(), ErrorTrace> {
    string_err().track()
}

fn string_err() -> Result<(), ErrorTrace> {
    Err(StringError::new("Some message")).track()
}
