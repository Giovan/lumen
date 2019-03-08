#![cfg_attr(not(test), allow(dead_code))]

use std::convert::TryInto;

use crate::integer::big;
use crate::process::Process;
use crate::term::{BadArgument, Tag::*, Term};

pub fn convert(time: rug::Integer, from_unit: Unit, to_unit: Unit) -> rug::Integer {
    if from_unit == to_unit {
        time
    } else {
        let from_hertz = from_unit.hertz();
        let to_hertz = to_unit.hertz();

        if from_hertz <= to_hertz {
            time * ((to_hertz / from_hertz) as i32)
        } else {
            // mimic behavior of erts_napi_convert_time_unit, so that rounding is the same
            let denominator = (from_hertz / to_hertz) as i32;

            if 0 <= time {
                time / denominator
            } else {
                (time - (denominator - 1)) / denominator
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Unit {
    Hertz(usize),
    Second,
    Millisecond,
    Microsecond,
    Nanosecond,
    Native,
    PerformanceCounter,
}

impl Unit {
    const MILLISECOND_HERTZ: usize = 1_000;

    pub fn try_from(term: Term, process: &mut Process) -> Result<Unit, BadArgument> {
        match term.tag() {
            SmallInteger => {
                let hertz: usize = term.try_into()?;

                Ok(Unit::Hertz(hertz))
            }
            Boxed => {
                let unboxed: &Term = term.unbox_reference();

                match unboxed.tag() {
                    BigInteger => {
                        let big_integer: &big::Integer = term.unbox_reference();
                        let hertz: usize = big_integer.inner.to_usize().ok_or(BadArgument)?;

                        Ok(Unit::Hertz(hertz))
                    }
                    _ => Err(BadArgument),
                }
            }
            Atom => {
                let term_string = term.atom_to_string(process);
                let mut result = Err(BadArgument);

                for (s, unit) in [
                    ("second", Unit::Second),
                    ("seconds", Unit::Second),
                    ("millisecond", Unit::Millisecond),
                    ("milli_seconds", Unit::Millisecond),
                    ("microsecond", Unit::Microsecond),
                    ("micro_seconds", Unit::Microsecond),
                    ("nanosecond", Unit::Nanosecond),
                    ("nano_seconds", Unit::Nanosecond),
                    ("native", Unit::Native),
                    ("perf_counter", Unit::PerformanceCounter),
                ]
                .iter()
                {
                    if term_string == *s {
                        result = Ok(*unit);
                        break;
                    }
                }

                result
            }
            _ => Err(BadArgument),
        }
    }

    pub fn hertz(&self) -> usize {
        match self {
            Unit::Hertz(hertz) => *hertz,
            Unit::Second => 1,
            Unit::Millisecond => Self::MILLISECOND_HERTZ,
            Unit::Microsecond => 1_000_000,
            Unit::Nanosecond => 1_000_000_000,
            // As a side-channel protection browsers limit most counters to 1 millisecond resolution
            Unit::Native => Self::MILLISECOND_HERTZ,
            Unit::PerformanceCounter => Self::MILLISECOND_HERTZ,
        }
    }
}