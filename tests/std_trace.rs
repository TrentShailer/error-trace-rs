use error_trace::{ErrorTrace, ResultExt};

#[test]
fn std_error_trace() {
    if let Err(e) = std_err_caller().track() {
        println!("{}", e.to_string());
        assert_eq!(
            e.to_string(),
            String::from(
                r"Error Trace:
[tests\std_trace.rs:27] The system cannot find the file specified. (os error 2)
[tests\std_trace.rs:23]
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
