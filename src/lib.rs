use borsh::{BorshSerialize, BorshDeserialize};

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    entrypoint
};

#[derive(BorshDeserialize, BorshSerialize)]
enum InstructionType {
    Increment(u32), 
    Decrement(u32)
}

#[derive(BorshDeserialize, BorshSerialize)]
struct Counter {
    count: u32
}

entrypoint!(counter_contract);

pub fn counter_contract(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    let acc = next_account_info(&mut accounts.iter())?;
    let instruction_type = InstructionType::try_from_slice(&instruction_data)?;
    let mut acc_data = Counter::try_from_slice(&mut acc.data.borrow())?;
    match instruction_type {
        InstructionType::Increment(val) => acc_data.count += val,
        InstructionType::Decrement(val) => acc_data.count -= val
    }
    acc_data.serialize(&mut *acc.data.borrow_mut())?;
    msg!("Counter updated to {}", acc_data.count);
    Ok(())
}