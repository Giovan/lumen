use super::*;

#[test]
fn without_binary_returns_false() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(
                &strategy::term::is_not_binary(arc_process.clone()),
                |term| {
                    prop_assert_eq!(erlang::is_binary_1(term), false.into());

                    Ok(())
                },
            )
            .unwrap();
    });
}

#[test]
fn with_binary_returns_true() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(&strategy::term::is_binary(arc_process.clone()), |term| {
                prop_assert_eq!(erlang::is_binary_1(term), true.into());

                Ok(())
            })
            .unwrap();
    });
}
