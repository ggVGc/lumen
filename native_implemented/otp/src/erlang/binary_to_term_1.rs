#[cfg(all(not(target_arch = "wasm32"), test))]
mod test;

use liblumen_alloc::erts::exception;
use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::prelude::Term;

use crate::erlang::binary_to_term_2;

#[native_implemented::function(erlang:binary_to_term/1)]
pub fn result(process: &Process, binary: Term) -> exception::Result<Term> {
    binary_to_term_2::result(process, binary, Term::NIL)
}
