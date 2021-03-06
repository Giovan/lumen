use std::convert::TryInto;
use std::sync::Arc;

use liblumen_alloc::erts::process::code::stack::frame::{Frame, Placement};
use liblumen_alloc::erts::process::{code, Process};
use liblumen_alloc::erts::term::{atom_unchecked, Boxed, Tuple};
use liblumen_alloc::ModuleFunctionArity;

pub fn place_frame(process: &Process, placement: Placement) {
    process.place_frame(frame(), placement);
}

// Private

/// ```elixir
/// # label 1
/// # pushed to stack: ()
/// # returned from call: {:ok, document}
/// # full stack: ({:ok, document})
/// # returns: {:ok, body} | :error
/// body_tuple = Lumen.Web.Document.body(document)
/// Lumen.Web.Wait.with_return(body_tuple)
/// ```
fn code(arc_process: &Arc<Process>) -> code::Result {
    arc_process.reduce();

    let ok_document = arc_process.stack_pop().unwrap();
    assert!(
        ok_document.is_tuple(),
        "ok_document ({:?}) is not a tuple",
        ok_document
    );
    let ok_document_tuple: Boxed<Tuple> = ok_document.try_into().unwrap();
    assert_eq!(ok_document_tuple.len(), 2);
    assert_eq!(ok_document_tuple[0], atom_unchecked("ok"));
    let document = ok_document_tuple[1];
    assert!(document.is_resource_reference());

    lumen_web::document::body_1::place_frame_with_arguments(
        arc_process,
        Placement::Replace,
        document,
    )?;

    Process::call_code(arc_process)
}

fn frame() -> Frame {
    let module_function_arity = Arc::new(ModuleFunctionArity {
        module: super::module(),
        function: super::function(),
        arity: 0,
    });

    Frame::new(module_function_arity, code)
}
