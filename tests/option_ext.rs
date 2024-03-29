use error_trace::{ErrorTrace, OptionExt, ResultExt};

#[test]
fn option_trace() {
    if let Err(e) = option_caller().track() {
        println!("{}", e.to_string());
        assert_eq!(
            e.to_string(),
            String::from(
                r"Error Trace:
[tests\option_ext.rs:31] Option was None, expected Some
[tests\option_ext.rs:27]
[tests\option_ext.rs:5]
"
            )
        );
    } else {
        unreachable!()
    }

    if let Err(_) = Some(()).track() {
        unreachable!()
    }
}

fn option_caller() -> Result<(), ErrorTrace> {
    option().track()
}

fn option() -> Result<(), ErrorTrace> {
    None.track()
}
