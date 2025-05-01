use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::{Keypair, Signer, read_keypair_file};
use solana_sdk::transaction::Transaction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::system_program;
use borsh::{BorshSerialize}; 
use clap::{Parser};
use spl_token::ID as TOKEN_PROGRAM_ID;
use spl_associated_token_account::ID as ASSOCIATED_TOKEN_PROGRAM_ID;
use std::str::FromStr;



#[derive(Parser)]
struct Opts {
	#[arg(long)]
	receiver: String,

	#[arg(long)]
	amount: u64,

	#[arg(long)]
	program_id_arg: String,

	#[arg(long)]
	rpc_url_arg: String,

	#[arg(long)]
	keypair_file: String,

	#[arg(long)]
	mint_account_arg: String,

	#[arg(long)]
	owner_arg: String,

	}


#[derive(BorshSerialize, Debug)]
struct TokenAmount {
	token_amount: u64,
	}

fn invoke_smart_contract(
	client: &RpcClient,
	mint_account: Pubkey,
	from_associated_token_account: Pubkey,
	to_associated_token_account: Pubkey,
	owner: Pubkey, //owner is the from wallet which owns the token acc which sends
	recipient: Pubkey, //recipient is the sol wallet which owns the token acc which will receive balance
	payer_keypair: &Keypair,
	token_amount: TokenAmount) -> Result<(), Box<dyn std::error::Error>> {
	let opts: Opts = Opts::parse();
   
    let program_id = opts.program_id_arg.parse().unwrap();

    let accounts = vec![
		
		AccountMeta::new(mint_account, false),
		AccountMeta::new(from_associated_token_account, false),
		AccountMeta::new(to_associated_token_account, false),
		AccountMeta::new(owner, false),
		AccountMeta::new(recipient, false),
		AccountMeta::new(payer_keypair.pubkey(), true),	
		AccountMeta::new_readonly(system_program::id(), false),
		AccountMeta::new_readonly(TOKEN_PROGRAM_ID, false),
		AccountMeta::new_readonly(ASSOCIATED_TOKEN_PROGRAM_ID, false),
    ]; // Add relevant accounts
	println!("starting program");
	
	let amount = token_amount.token_amount; 
	println!("Create the instruction");
	
    // Create the instruction
    let instruction = Instruction {
        program_id,
        accounts,
        data: amount.to_le_bytes().to_vec(), 
    };
	println!("successfully created instruction");
	println!("createing tx");

    // Create the transaction
    let mut transaction = Transaction::new_with_payer(&[instruction], Some(&payer_keypair.pubkey()));
	println!("get latest blockhash");
	
    // Sign the transaction
    let recent_blockhash = client.get_latest_blockhash().unwrap();
	println!("got blockhash");
	println!("sign tx");
	let signer = [payer_keypair];
	println!("{}", payer_keypair.pubkey());
    transaction.sign(&[payer_keypair], recent_blockhash);
	println!("signed tx");
	
    // Send the transaction
    let signature = client.send_and_confirm_transaction(&transaction).unwrap();
    println!("Transaction signature: {:?}", signature);

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let opts: Opts = Opts::parse();

	let rpc_url = opts.rpc_url_arg;
    let client = RpcClient::new(rpc_url);
    
    // create variables
    let mint_account = Pubkey::from_str(&opts.mint_account_arg)?;
  	let owner = Pubkey::from_str(&opts.owner_arg)?;//same as payer in our case
	let recipient = Pubkey::from_str(&opts.receiver).expect("Invalid Key provided");
	let payer_keypair = read_keypair_file(opts.keypair_file);
	let from_associated_token_account = spl_associated_token_account::get_associated_token_address(&owner, &mint_account); 
    let to_associated_token_account = spl_associated_token_account::get_associated_token_address(&recipient, &mint_account);
	let amount_lamp = opts.amount;
	
    invoke_smart_contract(
		&client,
		mint_account,
		from_associated_token_account,
		to_associated_token_account,
		owner,
		recipient,
		&payer_keypair?,
		TokenAmount { token_amount: amount_lamp }
		);

Ok(())

}


