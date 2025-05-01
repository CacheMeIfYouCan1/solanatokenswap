use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::sysvar::slot_history::ProgramError;
use solana_program::account_info::next_account_info;
use solana_program::system_instruction;
use solana_program::program::invoke;
use spl_token::instruction as token_instruction;
use spl_associated_token_account::instruction as associated_token_account_instruction;


#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct transferSolArgs {
    sol_quantity: u64,
}

entrypoint!(transfer_sol_with_cpi);

fn transfer_sol_with_cpi(
		program_id: &Pubkey,
		accounts: &[AccountInfo],
		instruction_data: &[u8],
	) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let payer = next_account_info(accounts_iter)?;
    let sol_recipient = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

	let data = transferSolArgs::try_from_slice(instruction_data).map_err(|_| ProgramError::InvalidInstructionData)?;


    invoke(
        &system_instruction::transfer(payer.key, sol_recipient.key, data.sol_quantity),
        &[payer.clone(), sol_recipient.clone(), system_program.clone()],
    )?;

    Ok(())
}



