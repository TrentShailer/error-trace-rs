use error_trace::{ErrorTrace, ResultExt};

#[test]
fn context_trace() {
    if let Err(e) = context_caller().track() {
        println!("{}", e.to_string());
        assert_eq!(
            e.to_string(),
            String::from(
                r"Error Trace:
[tests\context.rs:29] Some Error
[tests\context.rs:29] Test
[tests\context.rs:25] Some value: 'Test 2'
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
