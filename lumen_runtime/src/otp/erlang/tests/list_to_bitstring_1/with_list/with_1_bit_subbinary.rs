use super::*;

#[test]
fn without_byte_bitstring_or_list_element_errors_badarg() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(
                &(
                    strategy::term::binary::sub::with_bit_count(1, arc_process.clone()),
                    is_not_byte_bitstring_nor_list(arc_process.clone()),
                )
                    .prop_map(|(head, tail)| arc_process.cons(head, tail).unwrap()),
                |list| {
                    prop_assert_eq!(
                        erlang::list_to_bitstring_1(list, &arc_process),
                        Err(badarg!().into())
                    );

                    Ok(())
                },
            )
            .unwrap();
    });
}

#[test]
fn with_empty_list_returns_bitstring() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(
                &strategy::term::binary::sub::with_bit_count(1, arc_process.clone())
                    .prop_map(|head| (arc_process.cons(head, Term::NIL).unwrap(), head)),
                |(list, bitstring)| {
                    prop_assert_eq!(
                        erlang::list_to_bitstring_1(list, &arc_process),
                        Ok(bitstring)
                    );

                    Ok(())
                },
            )
            .unwrap();
    });
}

#[test]
fn with_improper_list_returns_binary() {
    with_tail_errors_badarg(|process| {
        let tail_head = process.integer(254).unwrap();
        let tail_tail = process.integer(253).unwrap();

        process.cons(tail_head, tail_tail).unwrap()
    })
}

#[test]
fn with_proper_list_returns_binary() {
    with(|head, process| {
        let tail_head = process.integer(254).unwrap();
        let tail_tail = Term::NIL;
        let tail = process.cons(tail_head, tail_tail).unwrap();

        let iolist = process.cons(head, tail).unwrap();

        assert_eq!(
            erlang::list_to_bitstring_1(iolist, &process),
            Ok(bitstring!(1, 255, 0 :: 1, &process))
        );
    })
}

#[test]
fn with_heap_binary_returns_bitstring() {
    with(|head, process| {
        let tail = process.binary_from_bytes(&[3, 4]).unwrap();

        let iolist = process.cons(head, tail).unwrap();

        assert_eq!(
            erlang::list_to_bitstring_1(iolist, &process),
            Ok(bitstring!(1, 129, 130, 0 :: 1, &process))
        );
    })
}

#[test]
fn with_subbinary_with_bit_count_0_returns_binary() {
    with(|head, process| {
        let original = process.binary_from_bytes(&[0b0000_0010]).unwrap();
        let tail = process
            .subbinary_from_original(original, 0, 0, 1, 0)
            .unwrap();

        let iolist = process.cons(head, tail).unwrap();

        assert_eq!(
            erlang::list_to_bitstring_1(iolist, &process),
            Ok(bitstring!(1, 0b1_0000000 | 0b0000000_1, 0b0 :: 1, &process))
        );
    });
}

#[test]
fn with_subbinary_with_bit_count_1_returns_subbinary() {
    with(|head, process| {
        let tail = bitstring!(0b0000_0010, 0b1 :: 1, &process);
        let iolist = process.cons(head, tail).unwrap();

        assert_eq!(
            erlang::list_to_bitstring_1(iolist, &process),
            Ok(bitstring!(1, 129, 1 :: 2, &process))
        );
    });
}

#[test]
fn with_subbinary_with_bit_count_2_returns_subbinary() {
    with(|head, process| {
        let tail = bitstring!(0b0000_0010, 0b01 :: 2, &process);
        let iolist = process.cons(head, tail).unwrap();

        assert_eq!(
            erlang::list_to_bitstring_1(iolist, &process),
            Ok(bitstring!(1, 129, 1 :: 3, &process))
        );
    });
}

#[test]
fn with_subbinary_with_bit_count_3_returns_subbinary() {
    with(|head, process| {
        let tail = bitstring!(0b0000_0010, 0b101 :: 3, &process);
        let iolist = process.cons(head, tail).unwrap();

        assert_eq!(
            erlang::list_to_bitstring_1(iolist, &process),
            Ok(bitstring!(1, 129, 5 :: 4, &process))
        );
    });
}

#[test]
fn with_subbinary_with_bit_count_4_returns_subbinary() {
    with(|head, process| {
        let tail = bitstring!(0b0000_0010, 0b0101 :: 4, &process);
        let iolist = process.cons(head, tail).unwrap();

        assert_eq!(
            erlang::list_to_bitstring_1(iolist, &process),
            Ok(bitstring!(1, 129, 5 :: 5, &process))
        );
    });
}

#[test]
fn with_subbinary_with_bit_count_5_returns_subbinary() {
    with(|head, process| {
        let tail = bitstring!(0b0000_0010, 0b10101 :: 5, &process);
        let iolist = process.cons(head, tail).unwrap();

        assert_eq!(
            erlang::list_to_bitstring_1(iolist, &process),
            Ok(bitstring!(1, 129, 21 :: 6, &process))
        );
    });
}

#[test]
fn with_subbinary_with_bit_count_6_returns_subbinary() {
    with(|head, process| {
        let tail = bitstring!(0b0000_0010, 0b010101 :: 6, &process);
        let iolist = process.cons(head, tail).unwrap();

        assert_eq!(
            erlang::list_to_bitstring_1(iolist, &process),
            Ok(bitstring!(1, 129, 21 :: 7, &process))
        );
    });
}

#[test]
fn with_subbinary_with_bit_count_7_returns_subbinary() {
    with(|head, process| {
        let tail = bitstring!(0b0000_0010, 0b1010101 :: 7, &process);
        let iolist = process.cons(head, tail).unwrap();

        assert_eq!(
            erlang::list_to_bitstring_1(iolist, &process),
            Ok(process.binary_from_bytes(&[1, 129, 85]).unwrap()),
        )
    });
}

fn with_tail_errors_badarg<T>(tail: T)
where
    T: FnOnce(&Process) -> Term,
{
    with(|head, process| {
        let iolist = process.cons(head, tail(&process)).unwrap();

        assert_badarg!(erlang::list_to_bitstring_1(iolist, &process));
    });
}

fn with<F>(f: F)
where
    F: FnOnce(Term, &Process) -> (),
{
    with_process(|process| {
        let head = bitstring!(1, 0b1 :: 1, &process);

        f(head, &process);
    })
}
