// wasm32 proptest cannot be compiled at the same time as non-wasm32 proptest, so disable tests that
// use proptest completely for wasm32
//
// See https://github.com/rust-lang/cargo/issues/4866
#[cfg(all(not(target_arch = "wasm32"), test))]
mod test;

use std::convert::TryInto;
use std::sync::Arc;

use liblumen_alloc::erts::exception;
use liblumen_alloc::erts::exception::system::Alloc;
use liblumen_alloc::erts::process::code::stack::frame::{Frame, Placement};
use liblumen_alloc::erts::process::code::{self, result_from_exception};
use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::{Atom, Term};
use liblumen_alloc::ModuleFunctionArity;

use crate::otp::erlang::spawn_apply_3;
use crate::process::spawn::options::Options;

pub fn place_frame_with_arguments(
    process: &Process,
    placement: Placement,
    module: Term,
    function: Term,
    arguments: Term,
    options: Term,
) -> Result<(), Alloc> {
    process.stack_push(options)?;
    process.stack_push(arguments)?;
    process.stack_push(function)?;
    process.stack_push(module)?;
    process.place_frame(frame(), placement);

    Ok(())
}

// Private

pub fn code(arc_process: &Arc<Process>) -> code::Result {
    arc_process.reduce();

    let module = arc_process.stack_pop().unwrap();
    let function = arc_process.stack_pop().unwrap();
    let arguments = arc_process.stack_pop().unwrap();
    let options = arc_process.stack_pop().unwrap();

    match native(arc_process, module, function, arguments, options) {
        Ok(child_pid) => {
            arc_process.return_from_call(child_pid)?;

            Process::call_code(arc_process)
        }
        Err(exception) => result_from_exception(arc_process, exception),
    }
}

fn frame() -> Frame {
    Frame::new(module_function_arity(), code)
}

fn function() -> Atom {
    Atom::try_from_str("spawn_opt").unwrap()
}

fn module_function_arity() -> Arc<ModuleFunctionArity> {
    Arc::new(ModuleFunctionArity {
        module: super::module(),
        function: function(),
        arity: 4,
    })
}

fn native(
    process: &Process,
    module: Term,
    function: Term,
    arguments: Term,
    options: Term,
) -> exception::Result {
    let options_options: Options = options.try_into()?;

    spawn_apply_3::native(process, options_options, module, function, arguments)
}
