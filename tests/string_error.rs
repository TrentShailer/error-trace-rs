use error_trace::{ErrorTrace, ResultExt};

#[test]
fn string_error_trace() {
    if let Err(e) = string_err_caller().track() {
        println!("{}", e.to_string());
        assert_eq!(
            e.to_string(),
            String::from(
                r"Error Trace:
[tests\string_error.rs:27] Some message
[tests\string_error.rs:23]
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
    Err("Some message").track()
}
