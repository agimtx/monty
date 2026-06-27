//! Adapter for the `aq` module.
//!
//! The actual module behavior lives in the sibling `aq` crate; this file only
//! registers that behavior with Monty's built-in module system and converts
//! Rust return values into VM values.

use crate::{
    args::ArgValues,
    bytecode::VM,
    exception_private::RunResult,
    heap::{HeapData, HeapId},
    intern::StaticStrings,
    modules::ModuleFunctions,
    resource::{ResourceError, ResourceTracker},
    types::{Module, str::allocate_string},
    value::Value,
};

/// Functions exposed by the `aq` module.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, serde::Serialize, serde::Deserialize)]
#[strum(serialize_all = "lowercase")]
pub(crate) enum AqFunctions {
    /// `aq.hello()` returns a fixed greeting string.
    Hello,
}

/// Creates the `aq` module and allocates it on the heap.
pub fn create_module(vm: &mut VM<'_, impl ResourceTracker>) -> Result<HeapId, ResourceError> {
    let mut module = Module::new(StaticStrings::Aq);
    module.set_attr(
        StaticStrings::Hello,
        Value::ModuleFunction(ModuleFunctions::Aq(AqFunctions::Hello)),
        vm,
    );
    vm.heap.allocate(HeapData::Module(module))
}

/// Dispatches an `aq` module function call.
///
/// Returns `Value` directly because the module currently has no host-side
/// effects and does not need to yield out of the VM.
pub(super) fn call(vm: &mut VM<'_, impl ResourceTracker>, function: AqFunctions, args: ArgValues) -> RunResult<Value> {
    match function {
        AqFunctions::Hello => hello(vm, args),
    }
}

/// `aq.hello()` returns a fixed greeting string.
fn hello(vm: &mut VM<'_, impl ResourceTracker>, args: ArgValues) -> RunResult<Value> {
    args.check_zero_args("aq.hello", vm.heap)?;
    Ok(allocate_string(aq::hello(), vm.heap)?)
}
