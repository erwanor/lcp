use crate::get_store;
use crate::light_client::GlobalLightClientRegistry;
use crate::prelude::*;
use crypto::KeyManager;
use ecall_commands::{CommandResult, ECallCommand};
use ecall_handler::dispatch;
use enclave_utils::validate_const_ptr;
use log::*;
use sgx_types::sgx_status_t;

#[no_mangle]
pub unsafe extern "C" fn ecall_execute_command(
    command: *const u8,
    command_len: u32,
    output_buf: *mut u8,
    output_buf_maxlen: u32,
    output_len: &mut u32,
) -> sgx_status_t {
    info!("enter ecall_execute_command");
    validate_const_ptr!(
        command,
        command_len as usize,
        sgx_status_t::SGX_ERROR_UNEXPECTED
    );

    let cmd: ECallCommand =
        bincode::deserialize(alloc::slice::from_raw_parts(command, command_len as usize)).unwrap();

    let km = KeyManager::new(cmd.params.home.clone());
    let (status, result) =
        match dispatch::<_, GlobalLightClientRegistry>(km.get_enclave_key(), &mut get_store(), cmd)
        {
            Ok(result) => (sgx_status_t::SGX_SUCCESS, result),
            Err(e) => (
                sgx_status_t::SGX_ERROR_UNEXPECTED,
                CommandResult::CommandError(format!("{:?}", e)),
            ),
        };
    let res = bincode::serialize(&result).unwrap();
    assert!(
        output_buf_maxlen as usize >= res.len(),
        "{} >= {}",
        output_buf_maxlen as usize,
        res.len()
    );
    core::ptr::copy_nonoverlapping(res.as_ptr(), output_buf, res.len());
    *output_len = res.len() as u32;

    status
}
