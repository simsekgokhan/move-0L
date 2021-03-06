// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0
use vdf::{VDFParams, VDF};
// use diem_types::transaction::authenticator::AuthenticationKey;
// use std::convert::TryInto;
use move_core_types::vm_status::StatusCode;
use move_vm_runtime::native_functions::NativeContext;
use move_vm_types::{
    gas_schedule::NativeCostIndex,
    loaded_data::runtime_types::Type,
    natives::function::{native_gas, NativeResult},
    pop_arg,
    values::{Reference, Value},
};
use std::collections::VecDeque;
use move_binary_format::errors::{PartialVMError, PartialVMResult};
// use hex;
use diem_global_constants::VDF_SECURITY_PARAM;
// const SECURITY_PARAM: u16 = 2048;
use smallvec::smallvec;

/// Rust implementation of Move's `native public fun verify(challenge: vector<u8>, 
/// difficulty: u64, alleged_solution: vector<u8>): bool`
pub fn native_verify(
    context: &mut NativeContext,
    _ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    if arguments.len() != 4 {
        let msg = format!(
            "wrong number of arguments for vdf_verify expected 4 found {}",
            arguments.len()
        );
        return Err(PartialVMError::new(StatusCode::UNREACHABLE).with_message(msg));
    }

    // pop the arguments (reverse order).
    let security = pop_arg!(arguments, Reference).read_ref()?.value_as::<u64>()?;
    let difficulty = pop_arg!(arguments, Reference).read_ref()?.value_as::<u64>()?;
    let solution = pop_arg!(arguments, Reference).read_ref()?.value_as::<Vec<u8>>()?;
    let challenge = pop_arg!(arguments, Reference).read_ref()?.value_as::<Vec<u8>>()?;

    // refuse to try anything with a security parameter above 2048 for DOS risk.
    if security > 2048 {
        return Err(
            PartialVMError::new(StatusCode::UNREACHABLE).with_message(
              "VDF security parameter above threshold".to_string()
            )
        );
    }

    // TODO change the `cost_index` when we have our own cost table.
    let cost = native_gas(context.cost_table(), NativeCostIndex::VDF_VERIFY, 1);

    let v = vdf::PietrzakVDFParams(security as u16).new();
    let result = v.verify(&challenge, difficulty, &solution);

    let return_values = smallvec![Value::bool(result.is_ok())];
    Ok(NativeResult::ok(cost, return_values))
}

// // 0L todo diem-1.4.1: Cyclic dependency problem
// Extracts the first 32 bits of the vdf challenge which is the auth_key
// Auth Keys can be turned into an AccountAddress type, to be serialized to a move address type.
pub fn native_extract_address_from_challenge(
    context: &mut NativeContext,
    _ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    // let cost = native_gas(context.cost_table(), NativeCostIndex::VDF_PARSE, 1);

    // let challenge_vec = pop_arg!(arguments, Reference)
    //     .read_ref()?
    //     .value_as::<Vec<u8>>()?;

    // let auth_key_vec = &challenge_vec[..32];
    // let auth_key = AuthenticationKey::new(auth_key_vec.try_into().expect("Check length"));
    // let address = auth_key.derived_address();
    // let return_values = smallvec![
    //     Value::address(address), Value::vector_u8(auth_key_vec[..16].to_owned())
    // ];
    // Ok(NativeResult::ok(cost, return_values))

    todo!();
}