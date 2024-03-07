use error_trace::{ErrorTrace, ResultExt};

#[test]
fn context_trace() {
    if let Err(e) = context_caller().track() {
        println!("{}", e.to_string());
        assert_eq!(
            e.to_string(),
            String::from(
                r"Error Trace:
App Version: 0.4.2
[tests\context.rs:30] Some Error
[tests\context.rs:30] Test
[tests\context.rs:26] Some value: 'Test 2'
[tests\context.rs:5]
"
            )
        );
    } else {
        unreachable!()
    }
}

fn context_caller() -> Result<(), ErrorTrace> {
    let some_value = "Test 2";
    context().with_context(|| format!("Some value: '{}'", some_value))
}

fn context() -> Result<(), ErrorTrace> {
    Err("Some Error").context("Test")
}
