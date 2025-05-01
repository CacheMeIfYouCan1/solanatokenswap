import asyncio
import json
from solana.rpc.api import Client
from solders.pubkey import Pubkey
from solders.signature import Signature
import websockets
import subprocess

class TransactionProcessing:

	def __init__(self, transaction_details, tx_signature_str):

		self.transaction_details = transaction_details
		self.tx_signature_str = tx_signature_str

	
		with open('config.json', 'r') as file:
			config = json.load(file)

		self.send_token_program_id = config["send_token_program_id"]
		self.send_sol_program_id = config["send_sol_program_id"]
		self.commission_wallet = config["commission_wallet"]
		self.mint_account = config["mint_account"]
		self.keypair_file = config["keypair_file"]
		self.rpc_client_str = config["rpc_client_str"]
		self.send_sol_handler = config["send_sol_handler"]
		self.send_token_handler = config["send_token_handler"]
		self.commission_rate = config["commission_rate"]
		self.token_account_str = config["token_acc_str"]
		self.swap_rate = config["swap_rate"]

		transaction_info = self.transaction_details.value.transaction.transaction
		self.account_keys = transaction_info.message.account_keys
		self.instructions = transaction_info.message.instructions
		



	async def process_sol_to_token(self, sol_receiver_wallet):
		
		if self.transaction_details and self.transaction_details.value:
					
			for instruction in self.instructions:
				sender_index = instruction.accounts[0]  
				sender_account = self.account_keys[sender_index]

				commission_wallet = self.commission_wallet
				

				sender_account_str = str(sender_account)

				if sender_account_str != self.token_account_str:
					if sender_account_str != sol_receiver_wallet:

						pre_balances = self.transaction_details.value.transaction.meta.pre_balances
						post_balances = self.transaction_details.value.transaction.meta.post_balances


						lamports_sent = pre_balances[0]-post_balances[0]
						sol_to_send_lamports_int = int(lamports_sent*float(self.commission_rate))
						tokens_to_send_lamports = lamports_sent*int(self.swap_rate)
					
						print("Token purchased, executing token transfer")
						call_token_smartcontract = [
							self.send_token_handler,
							"--receiver", str(sender_account),
							"--amount", str(tokens_to_send_lamports).lstrip('-'),
							"--program-id-arg", self.send_token_program_id,
							"--rpc-url-arg", self.rpc_client_str,
							"--keypair-file", self.keypair_file,
							"--mint-account-arg", self.mint_account,
							"--owner-arg", self.commission_wallet
							]
						result_token = subprocess.run(call_token_smartcontract, capture_output= True, text= True)
						print(f"result token tx: {result_token.stdout}")
						print(f"{result_token.stderr}")

						print("executing transfer of commission")
						call_sol_smartcontract = [
								self.send_sol_handler,
								"--receiver", self.commission_wallet,
								"--amount", str(sol_to_send_lamports_int).lstrip('-'),
								"--program-id-arg", self.send_sol_program_id,
								"--rpc-url-arg", self.rpc_client_str,
								"--keypair-file", self.keypair_file
								]	
						result_sol = subprocess.run(call_sol_smartcontract, capture_output= True, text= True)
						print(f"result sol tx: {result_sol.stdout}")
						print(f" {result_sol.stderr}")
						
					else:
						print("")
						print("Sender is receiver, skipping")
						print("")
						break
						
				else:
					print("")
					print("token transaction, skipping")
					print("")
					
		else:
			print("")
			print("Transaction details not found after multiple retries, skipping")
			print("")


	async def process_token_to_sol(self, token_receiver_wallet):
		
		if self.transaction_details and self.transaction_details.value:
			
			for instruction in self.instructions:

				if instruction.accounts:
					sender_index = instruction.accounts[0]
					sender_associated_account_owner = self.account_keys[0]
	
					for balance_meta in self.transaction_details.value.transaction.meta.pre_token_balances:
						if balance_meta.account_index == sender_index:
							sender_associated_account_owner = balance_meta.owner
							break
							
					else:
						sender_associated_account_owner = None
						
					for pre_balance_meta in self.transaction_details.value.transaction.meta.pre_token_balances:
						if pre_balance_meta.account_index == sender_index:
							pre_token_balance = pre_balance_meta.ui_token_amount.amount
							break
							
					else:
						pre_token_balance = None		
							
					for post_balance_meta in self.transaction_details.value.transaction.meta.post_token_balances:
						if post_balance_meta.account_index == sender_index:
							post_token_balance = post_balance_meta.ui_token_amount.amount
							break
					else:
						post_token_balance = None
								
					if sender_associated_account_owner is not None:
						lamports_sent = int(pre_token_balance) - int(post_token_balance) 

						sol_to_send_lamports_int = int(lamports_sent/int(self.swap_rate))
						sol_to_send_lamports_str = str(sol_to_send_lamports_int)

						sender_associated_account_owner_str = str(sender_associated_account_owner)
						if sender_associated_account_owner != token_receiver_wallet:
										
							print("Token sold: executing SOL transfer")

							call_sol_smartcontract = [
								self.send_sol_handler,
								"--receiver", str(sender_associated_account_owner),
								"--amount", sol_to_send_lamports_str,
								"--program-id-arg", self.send_sol_program_id,
								"--rpc-url-arg", self.rpc_client_str,
								"--keypair-file", self.keypair_file
							]								
							result_sol = subprocess.run(call_sol_smartcontract, capture_output= True, text= True)
							print(f"result sol: {result_sol.stdout}")
							print(f"error sol: {result_sol.stderr}")
								
							
							break
						
						else:
							print("")	
							print("Sender is receiver, skipping")
							print("")
							break

				else:
					print("")
					print("instruction is empty, skipping")
					print("")
					
			else:
				print("")
				print("token purchased, skipping")
				print("")
				
		else:
			print("")
			print("Transaction details not found after multiple retries.")
			print("")
