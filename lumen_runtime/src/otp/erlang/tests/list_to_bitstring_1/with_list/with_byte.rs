use super::*;

#[test]
fn with_atom_errors_badarg() {
    with_tail_errors_badarg(|_| Term::str_to_atom("", DoNotCare).unwrap());
}

#[test]
fn with_local_reference_errors_badarg() {
    with_tail_errors_badarg(|mut process| Term::local_reference(&mut process));
}

#[test]
fn with_empty_list_returns_1_byte_binary() {
    with(|head_byte, head, mut process| {
        let tail = Term::EMPTY_LIST;
        let iolist = Term::cons(head, tail, &mut process);

        assert_eq!(
            erlang::list_to_bitstring_1(iolist, &mut process),
            Ok(Term::slice_to_binary(&[head_byte], &mut process))
        );
    })
}

#[test]
fn with_improper_list_returns_binary() {
    with_tail_errors_badarg(|mut process| {
        let tail_head = 1.into_process(&mut process);
        let tail_tail = 2.into_process(&mut process);

        Term::cons(tail_head, tail_tail, &mut process)
    });
}

#[test]
fn with_proper_list_returns_binary() {
    with(|_, head, mut process| {
        let tail_head_byte = 1;
        let tail_head = tail_head_byte.into_process(&mut process);
        let tail_tail = Term::EMPTY_LIST;
        let tail = Term::cons(tail_head, tail_tail, &mut process);

        let iolist = Term::cons(head, tail, &mut process);

        assert_eq!(
            erlang::list_to_bitstring_1(iolist, &mut process),
            Ok(Term::slice_to_binary(&[0, 1], &mut process))
        );
    });
}

#[test]
fn with_byte_errors_badarg() {
    with_tail_errors_badarg(|mut process| 254.into_process(&mut process));
}

#[test]
fn with_small_integer_with_byte_overflow_errors_badarg() {
    with_tail_errors_badarg(|mut process| 256.into_process(&mut process));
}

#[test]
fn with_big_integer_errors_badarg() {
    with_tail_errors_badarg(|mut process| {
        (crate::integer::small::MAX + 1).into_process(&mut process)
    });
}

#[test]
fn with_float_errors_badarg() {
    with_tail_errors_badarg(|mut process| 1.0.into_process(&mut process));
}

#[test]
fn with_local_pid_errors_badarg() {
    with_tail_errors_badarg(|_| Term::local_pid(0, 0).unwrap());
}

#[test]
fn with_external_pid_errors_badarg() {
    with_tail_errors_badarg(|mut process| Term::external_pid(1, 0, 0, &mut process).unwrap());
}

#[test]
fn with_tuple_errors_badarg() {
    with_tail_errors_badarg(|mut process| Term::slice_to_tuple(&[], &mut process));
}

#[test]
fn with_map_errors_badarg() {
    with_tail_errors_badarg(|mut process| Term::slice_to_map(&[], &mut process));
}

#[test]
fn with_heap_binary_returns_binary() {
    with(|head_byte, head, mut process| {
        let tail = Term::slice_to_binary(&[1, 2], &mut process);

        let iolist = Term::cons(head, tail, &mut process);

        assert_eq!(
            erlang::list_to_bitstring_1(iolist, &mut process),
            Ok(Term::slice_to_binary(&[head_byte, 1, 2], &mut process))
        );
    })
}

#[test]
fn with_subbinary_with_bit_count_0_returns_binary() {
    with(|head_byte, head, mut process| {
        let original = Term::slice_to_binary(&[1], &mut process);
        let tail = Term::subbinary(original, 0, 0, 1, 0, &mut process);

        let iolist = Term::cons(head, tail, &mut process);

        assert_eq!(
            erlang::list_to_bitstring_1(iolist, &mut process),
            Ok(Term::slice_to_binary(&[head_byte, 1], &mut process))
        );
    });
}

#[test]
fn with_subbinary_with_bit_count_1_returns_subbinary() {
    with(|head_byte, head, mut process| {
        let original = Term::slice_to_binary(&[2, 0b1 << (8 - 1)], &mut process);
        let tail = Term::subbinary(original, 0, 0, 1, 1, &mut process);

        let iolist = Term::cons(head, tail, &mut process);

        assert_eq!(
            erlang::list_to_bitstring_1(iolist, &mut process),
            Ok(Term::subbinary(
                Term::slice_to_binary(&[head_byte, 2, 128], &mut process),
                0,
                0,
                2,
                1,
                &mut process
            ))
        );
    });
}

#[test]
fn with_subbinary_with_bit_count_2_returns_subbinary() {
    with(|_head_byte, head, mut process| {
        let original = Term::slice_to_binary(&[1, 0b11 << (8 - 2)], &mut process);
        let tail = Term::subbinary(original, 0, 0, 1, 2, &mut process);

        let iolist = Term::cons(head, tail, &mut process);

        assert_eq!(
            erlang::list_to_bitstring_1(iolist, &mut process),
            Ok(Term::subbinary(
                Term::slice_to_binary(&[0, 1, 3 << (8 - 2)], &mut process),
                0,
                0,
                1 + 1,
                2,
                &mut process
            ))
        );
    });
}

#[test]
fn with_subbinary_with_bit_count_3_returns_subbinary() {
    with(|_head_byte, head, mut process| {
        let original = Term::slice_to_binary(&[1, 0b101 << (8 - 3)], &mut process);
        let tail = Term::subbinary(original, 0, 0, 1, 3, &mut process);

        let iolist = Term::cons(head, tail, &mut process);

        assert_eq!(
            erlang::list_to_bitstring_1(iolist, &mut process),
            Ok(Term::subbinary(
                Term::slice_to_binary(&[0, 1, 0b101 << (8 - 3)], &mut process),
                0,
                0,
                1 + 1,
                3,
                &mut process
            ))
        );
    });
}

#[test]
fn with_subbinary_with_bit_count_4_returns_subbinary() {
    with(|_head_byte, head, mut process| {
        let original = Term::slice_to_binary(&[1, 0b0101 << (8 - 4)], &mut process);
        let tail = Term::subbinary(original, 0, 0, 1, 4, &mut process);

        let iolist = Term::cons(head, tail, &mut process);

        assert_eq!(
            erlang::list_to_bitstring_1(iolist, &mut process),
            Ok(Term::subbinary(
                Term::slice_to_binary(&[0, 1, 0b0101 << (8 - 4)], &mut process),
                0,
                0,
                1 + 1,
                4,
                &mut process
            ))
        );
    });
}

#[test]
fn with_subbinary_with_bit_count_5_returns_subbinary() {
    with(|_head_byte, head, mut process| {
        let original = Term::slice_to_binary(&[1, 0b10101 << (8 - 5)], &mut process);
        let tail = Term::subbinary(original, 0, 0, 1, 5, &mut process);

        let iolist = Term::cons(head, tail, &mut process);

        assert_eq!(
            erlang::list_to_bitstring_1(iolist, &mut process),
            Ok(Term::subbinary(
                Term::slice_to_binary(&[0, 1, 0b10101 << (8 - 5)], &mut process),
                0,
                0,
                1 + 1,
                5,
                &mut process
            ))
        );
    });
}

#[test]
fn with_subbinary_with_bit_count_6_returns_subbinary() {
    with(|_head_byte, head, mut process| {
        let original = Term::slice_to_binary(&[1, 0b010101 << (8 - 6)], &mut process);
        let tail = Term::subbinary(original, 0, 0, 1, 6, &mut process);

        let iolist = Term::cons(head, tail, &mut process);

        assert_eq!(
            erlang::list_to_bitstring_1(iolist, &mut process),
            Ok(Term::subbinary(
                Term::slice_to_binary(&[0, 1, 0b010101 << (8 - 6)], &mut process),
                0,
                0,
                1 + 1,
                6,
                &mut process
            ))
        );
    });
}

#[test]
fn with_subbinary_with_bit_count_7_returns_subbinary() {
    with(|_head_byte, head, mut process| {
        let original = Term::slice_to_binary(&[1, 0b1010101 << (8 - 7)], &mut process);
        let tail = Term::subbinary(original, 0, 0, 1, 7, &mut process);

        let iolist = Term::cons(head, tail, &mut process);

        assert_eq!(
            erlang::list_to_bitstring_1(iolist, &mut process),
            Ok(Term::subbinary(
                Term::slice_to_binary(&[0, 1, 0b1010101 << (8 - 7)], &mut process),
                0,
                0,
                1 + 1,
                7,
                &mut process
            )),
        )
    });
}

fn with_tail_errors_badarg<T>(tail: T)
where
    T: FnOnce(&mut Process) -> Term,
{
    with(|_, head, mut process| {
        let iolist = Term::cons(head, tail(&mut process), &mut process);

        assert_badarg!(erlang::list_to_bitstring_1(iolist, &mut process));
    });
}

fn with<F>(f: F)
where
    F: FnOnce(u8, Term, &mut Process) -> (),
{
    with_process(|mut process| {
        let head_byte: u8 = 0;
        let head = head_byte.into_process(&mut process);

        f(head_byte, head, &mut process);
    })
}
