#![feature(test)]

extern crate test;

use std::vec;

use mollusk_svm::Mollusk;
use mollusk_svm_bencher::MolluskComputeUnitBencher;
use solana_account::Account;
use solana_address::Address;
use solana_instruction::Instruction;
use test::Bencher;

const DEFAULT_LOADER_KEY: Address = solana_sdk_ids::bpf_loader_upgradeable::ID;

/// Create a new Mollusk instance for the given program ID and name.
pub fn setup(program_id: &Address, name: &'static str) -> Mollusk {
    unsafe {
        std::env::set_var("SBF_OUT_DIR", "../target/deploy");
    }
    solana_logger::setup();

    Mollusk::new(program_id, name)
}

/// Create an instruction and associated accounts for testing.
pub fn instruction(program_id: &Address) -> (Instruction, Vec<(Address, Account)>) {
    let accounts = vec![];
    (
        Instruction {
            program_id: *program_id,
            accounts: vec![],
            data: vec![],
        },
        accounts,
    )
}

#[cfg(test)]
#[bench]
fn run(_bencher: &mut Bencher) {
    // current
    let current_id = Address::from_str_const("Current111111111111111111111111111111111111");
    let mut mollusk = setup(&current_id, "current_program");

    // generic
    let generic_id = Address::from_str_const("Generic111111111111111111111111111111111111");
    mollusk.add_program(&generic_id, "generic_program", &DEFAULT_LOADER_KEY);
    let mut bencher = MolluskComputeUnitBencher::new(mollusk)
        .must_pass(true)
        .out_dir("../target/benches");

    let (ix, accounts) = instruction(&current_id);
    bencher = bencher.bench(("current::sysvar_get", &ix, &accounts));

    let (ix, accounts) = instruction(&generic_id);
    bencher = bencher.bench(("generic::sysvar_get", &ix, &accounts));

    bencher.execute();
}
