use super::*;

#[test]
fn returns_true() {
    with_process_arc(|monitoring_arc_process| {
        let monitored_arc_process = process::test(&monitoring_arc_process);

        let monitor_reference = monitor_2::native(
            &monitoring_arc_process,
            r#type(),
            monitored_arc_process.pid_term(),
        )
        .unwrap();

        let monitored_monitor_count_before = monitor_count(&monitored_arc_process);
        let monitoring_monitored_count_before = monitored_count(&monitoring_arc_process);

        assert_eq!(
            native(
                &monitoring_arc_process,
                monitor_reference,
                options(&monitoring_arc_process)
            ),
            Ok(true.into())
        );

        let monitored_monitor_count_after = monitor_count(&monitored_arc_process);
        let monitoring_monitored_count_after = monitored_count(&monitoring_arc_process);

        assert_eq!(
            monitored_monitor_count_after,
            monitored_monitor_count_before - 1
        );
        assert_eq!(
            monitoring_monitored_count_after,
            monitoring_monitored_count_before - 1
        );
    });
}

#[test]
fn flushes_existing_message_and_returns_false() {
    with_process_arc(|monitoring_arc_process| {
        let monitored_arc_process = process::test(&monitoring_arc_process);
        let monitored_pid_term = monitored_arc_process.pid_term();

        let monitor_reference =
            monitor_2::native(&monitoring_arc_process, r#type(), monitored_pid_term).unwrap();

        let reason = atom_unchecked("normal");
        exit_1::place_frame_with_arguments(&monitored_arc_process, Placement::Replace, reason)
            .unwrap();

        assert!(Scheduler::current().run_through(&monitored_arc_process));

        assert!(monitored_arc_process.is_exiting());
        assert!(!monitoring_arc_process.is_exiting());

        let tag = atom_unchecked("DOWN");

        assert!(has_message(
            &monitoring_arc_process,
            monitoring_arc_process
                .tuple_from_slice(&[tag, monitor_reference, r#type(), monitored_pid_term, reason])
                .unwrap()
        ));

        assert_eq!(
            native(
                &monitoring_arc_process,
                monitor_reference,
                options(&monitoring_arc_process)
            ),
            Ok(false.into())
        );

        assert!(!has_message(
            &monitoring_arc_process,
            monitoring_arc_process
                .tuple_from_slice(&[tag, monitor_reference, r#type(), monitored_pid_term, reason])
                .unwrap()
        ));
    });
}

#[test]
fn prevents_future_messages() {
    with_process_arc(|monitoring_arc_process| {
        let monitored_arc_process = process::test(&monitoring_arc_process);
        let monitored_pid_term = monitored_arc_process.pid_term();

        let monitor_reference =
            monitor_2::native(&monitoring_arc_process, r#type(), monitored_pid_term).unwrap();

        let reason = atom_unchecked("normal");
        let tag = atom_unchecked("DOWN");

        assert!(!has_message(
            &monitoring_arc_process,
            monitoring_arc_process
                .tuple_from_slice(&[tag, monitor_reference, r#type(), monitored_pid_term, reason])
                .unwrap()
        ));

        assert_eq!(
            native(
                &monitoring_arc_process,
                monitor_reference,
                options(&monitoring_arc_process)
            ),
            Ok(true.into())
        );

        exit_1::place_frame_with_arguments(&monitored_arc_process, Placement::Replace, reason)
            .unwrap();

        assert!(Scheduler::current().run_through(&monitored_arc_process));

        assert!(monitored_arc_process.is_exiting());
        assert!(!monitoring_arc_process.is_exiting());

        assert!(!has_message(
            &monitoring_arc_process,
            monitoring_arc_process
                .tuple_from_slice(&[tag, monitor_reference, r#type(), monitored_pid_term, reason])
                .unwrap()
        ));
    });
}
