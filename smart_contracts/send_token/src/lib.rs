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
pub struct transferTokenArgs {
    token_quantity: u64,
}

entrypoint!{transfer_token}


fn transfer_token(
		program_id: &Pubkey,
		accounts: &[AccountInfo],
		instruction_data: &[u8],
	) -> ProgramResult {

	let accounts_iter = &mut accounts.iter();

    let mint_account = next_account_info(accounts_iter)?;
    let from_associated_token_account = next_account_info(accounts_iter)?;
    let to_associated_token_account = next_account_info(accounts_iter)?;
    let owner = next_account_info(accounts_iter)?;
    let recipient = next_account_info(accounts_iter)?;
    let payer = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let associated_token_program = next_account_info(accounts_iter)?;

	let data = transferTokenArgs::try_from_slice(instruction_data).map_err(|_| ProgramError::InvalidInstructionData)?;


	   if to_associated_token_account.lamports() == 0 {
        msg!("Creating associated token account for recipient...");
        invoke(
            &associated_token_account_instruction::create_associated_token_account(
                payer.key,
                recipient.key,
                mint_account.key,
                token_program.key,
            ),
            &[
                mint_account.clone(),
                to_associated_token_account.clone(),
                recipient.clone(),
                payer.clone(),
                system_program.clone(),
                token_program.clone(),
                associated_token_program.clone(),
            ],
        )?;
    } else {
        msg!("Associated token account exists.");
    }
    msg!(
        "Recipient Associated Token Address: {}",
        to_associated_token_account.key
    );

    msg!("Transferring {} tokens...", data.token_quantity);
    msg!("Mint: {}", mint_account.key);
    msg!("Owner Token Address: {}", from_associated_token_account.key);
    msg!(
        "Recipient Token Address: {}",
        to_associated_token_account.key
    );
    invoke(

		
    
        &token_instruction::transfer(
            token_program.key,
            from_associated_token_account.key,
            to_associated_token_account.key,
            payer.key,
            &[],
            data.token_quantity,
        )?,
        &[
            mint_account.clone(),
            from_associated_token_account.clone(),
            to_associated_token_account.clone(),
            payer.clone(),
            token_program.clone(),
        ],
    )?;

    msg!("Tokens transferred successfully.");

    Ok(())

}
