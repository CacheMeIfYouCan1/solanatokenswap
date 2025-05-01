use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::{Keypair, Signer, read_keypair_file};
use solana_sdk::transaction::Transaction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::system_program;
use borsh::{BorshSerialize}; 
use clap::{Parser};
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

	}

#[derive(BorshSerialize, Debug)]
struct SolQuantity {
	amount: u64,
	}

fn invoke_smart_contract(
	client: &RpcClient,
	payer_keypair: &Keypair,
	sol_recipient: Pubkey,
	sol_quantity: SolQuantity) -> Result<(), Box<dyn std::error::Error>> {

	let opts: Opts = Opts::parse();	
    
    let program_id = opts.program_id_arg.parse().unwrap();

    let accounts = vec![
		AccountMeta::new(payer_keypair.pubkey(), true),
		AccountMeta::new(sol_recipient, false),
		AccountMeta::new_readonly(system_program::id(), false),
    ]; // Add relevant accounts

    // Create the instruction
    let instruction = Instruction {
        program_id,
        accounts,
        data: sol_quantity.amount.to_le_bytes().to_vec(), // Convert the amount to bytes
    };

    // Create the transaction
    let mut transaction = Transaction::new_with_payer(&[instruction], Some(&payer_keypair.pubkey()));
	
	println!("{}", payer_keypair.pubkey());
    // Sign the transaction
    let recent_blockhash = client.get_latest_blockhash().unwrap();
    transaction.sign(&[payer_keypair], recent_blockhash);

    // Send the transaction
    let signature = client.send_and_confirm_transaction(&transaction).unwrap();
    println!("Transaction signature: {:?}", signature);

    Ok(())
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
	let opts: Opts = Opts::parse();
    
    let rpc_url = opts.rpc_url_arg;
    let client = RpcClient::new(rpc_url);

    let payer_keypair = read_keypair_file(opts.keypair_file);
   
    let amount_lamp = opts.amount;
    let sol_recipient = Pubkey::from_str(&opts.receiver);

    invoke_smart_contract(&client, &payer_keypair?, sol_recipient?, SolQuantity { amount: amount_lamp });

Ok(())


}




