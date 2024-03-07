use error_trace::{ErrorTrace, ResultExt};

#[test]
fn std_error_trace() {
    if let Err(e) = std_err_caller().track() {
        assert_eq!(
            e.to_string(),
            String::from(
                r"Error Trace:
App Version: 0.4.1
[tests\std_trace.rs:26] The system cannot find the file specified. (os error 2)
[tests\std_trace.rs:22]
[tests\std_trace.rs:5]
"
            )
        );
    } else {
        unreachable!()
    }
}

fn std_err_caller() -> Result<(), ErrorTrace> {
    std_err().track()
}

fn std_err() -> Result<(), ErrorTrace> {
    std::fs::read("RandomFile").map(drop).track()
}
