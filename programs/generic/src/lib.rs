use core::mem::MaybeUninit;

use pinocchio::{
    entrypoint,
    error::ProgramError,
    sysvars::rent::{Rent, DEFAULT_LAMPORTS_PER_BYTE_YEAR},
    AccountView, Address, ProgramResult,
};

// Declares the entrypoint of the program.
entrypoint!(process_instruction);

#[allow(unused_variables, deprecated)]
/// Instruction processor
pub fn process_instruction(
    _program_id: &Address,
    _accounts: &[AccountView],
    _instruction_data: &[u8],
) -> ProgramResult {
    let mut var = MaybeUninit::<Rent>::uninit();
    let var_addr = var.as_mut_ptr() as *mut _ as *mut u8;

    // Use the generic `sol_get_sysvar` syscall to load the rent sysvar.
    // It fixes the size to 17 since the byte layout follows bincode
    // serialization. Note that there are 24 bytes allocated, so the
    // remaining bytes are just padding.
    #[cfg(target_os = "solana")]
    unsafe {
        pinocchio::syscalls::sol_get_sysvar(
            &pinocchio::sysvars::rent::RENT_ID as *const _ as *const u8,
            var_addr,
            0,
            17,
        );
        // Make sure all bytes are initialized.
        var_addr.add(17).write_bytes(0, 7)
    }

    let rent = unsafe { var.assume_init() };

    if rent.lamports_per_byte_year != DEFAULT_LAMPORTS_PER_BYTE_YEAR {
        return Err(ProgramError::InvalidAccountData);
    }

    Ok(())
}
