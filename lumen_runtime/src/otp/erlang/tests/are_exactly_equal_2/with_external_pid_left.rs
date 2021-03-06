use super::*;

use proptest::strategy::Strategy;

#[test]
fn without_external_pid_left_returns_false() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(
                &(
                    strategy::term::pid::external(arc_process.clone()),
                    strategy::term(arc_process.clone())
                        .prop_filter("Left cannot be an external pid", |left| {
                            !left.is_external_pid()
                        }),
                ),
                |(left, right)| {
                    prop_assert_eq!(erlang::are_exactly_equal_2(left, right), false.into());

                    Ok(())
                },
            )
            .unwrap();
    });
}

#[test]
fn with_same_external_pid_right_returns_true() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(
                &strategy::term::pid::external(arc_process.clone()),
                |operand| {
                    prop_assert_eq!(erlang::are_exactly_equal_2(operand, operand), true.into());

                    Ok(())
                },
            )
            .unwrap();
    });
}

#[test]
fn with_same_value_external_pid_right_returns_true() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(
                &(
                    strategy::term::pid::external::node_id(),
                    strategy::term::pid::number(),
                    strategy::term::pid::serial(),
                )
                    .prop_map(|(node, number, serial)| {
                        let mut heap = arc_process.acquire_heap();

                        (
                            heap.external_pid_with_node_id(node, number, serial)
                                .unwrap(),
                            heap.external_pid_with_node_id(node, number, serial)
                                .unwrap(),
                        )
                    }),
                |(left, right)| {
                    prop_assert_eq!(erlang::are_exactly_equal_2(left, right), true.into());

                    Ok(())
                },
            )
            .unwrap();
    });
}

#[test]
fn with_different_external_pid_right_returns_false() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(
                &(
                    strategy::term::pid::external(arc_process.clone()),
                    strategy::term::pid::external(arc_process.clone()),
                )
                    .prop_filter("Right and left must be different", |(left, right)| {
                        left != right
                    }),
                |(left, right)| {
                    prop_assert_eq!(erlang::are_exactly_equal_2(left, right), false.into());

                    Ok(())
                },
            )
            .unwrap();
    });
}
