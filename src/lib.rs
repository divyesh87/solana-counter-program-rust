use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

pub mod instruction;
use instruction::UserInstruction;

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct CounterAccount {
    pub counter: u32,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    if account.owner != program_id {
        msg!("Account doesnt belong to this program!");
        return Err(ProgramError::InvalidInstructionData);
    }

    let mut counter_account = CounterAccount::try_from_slice(&account.data.borrow())?;

    let operation = UserInstruction::unpack(instruction_data).unwrap();

    match operation {
        UserInstruction::Decrement => counter_account.counter -= 1,
        UserInstruction::Increment => counter_account.counter += 1,
        UserInstruction::Set(val) => counter_account.counter = val,
    }

    counter_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    Ok(())
}
