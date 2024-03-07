use error_trace::{ErrorTrace, OptionExt, ResultExt};

#[test]
fn option_trace() {
    if let Err(e) = option_caller().track() {
        println!("{}", e.to_string());
        assert_eq!(
            e.to_string(),
            String::from(
                r"Error Trace:
App Version: 0.4.2
[tests\option_ext.rs:32] Option was None, expected Some
[tests\option_ext.rs:28]
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
