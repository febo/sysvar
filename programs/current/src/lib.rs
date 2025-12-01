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

    #[cfg(target_os = "solana")]
    unsafe {
        pinocchio::syscalls::sol_get_rent_sysvar(var_addr);
    }

    let rent = unsafe { var.assume_init() };

    if rent.lamports_per_byte_year != DEFAULT_LAMPORTS_PER_BYTE_YEAR {
        return Err(ProgramError::InvalidAccountData);
    }

    Ok(())
}
