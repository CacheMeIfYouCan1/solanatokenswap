import asyncio
import json
from solana.rpc.api import Client
from solders.pubkey import Pubkey
from solders.signature import Signature
import websockets
import subprocess

from resources.transaction_processing import TransactionProcessing

class TransactionHandling:

	def __init__(self, receiver_wallet, transaction_type):

		self.receiver_wallet = receiver_wallet
		self.transaction_type = transaction_type
	
		self.transaction_queue = asyncio.Queue()
		self.rate_limit = asyncio.Semaphore(5)

		with open('config.json', 'r') as file:
			config = json.load(file)
		
		self.client = Client(config["rpc_client_str"])
		self.socket = config["ws_client_str"]
		
		self.send_token_program_id = config["send_token_program_id"]
		self.send_sol_program_id = config["send_sol_program_id"]

		self.last_tx_id = "0";


	async def listen_for_transactions(self):

		async with websockets.connect(self.socket) as websocket:
		
			subscription_request = {
				"jsonrpc": "2.0",
				"id": 1,
				"method": "logsSubscribe",
				"params": [
					{
						"mentions": [f"{self.receiver_wallet}"]
					},
					{
						"commitment": "finalized"
					}
				]
			}

			await websocket.send(json.dumps(subscription_request))
			
			while True:
				response = await websocket.recv()

				self.tx_signature_raw = json.loads(response)
				print("Transaction log received:", self.tx_signature_raw)

				await self.transaction_queue.put(self.tx_signature_raw)
				print(f"Transaction added to queue: {self.tx_signature_raw}")


	async def fetch_transaction_details(self, tx_signature_str):

		tx_signature = Signature.from_string(tx_signature_str)

		for i in range(10): 
			transaction = self.client.get_transaction(tx_signature)
			if transaction.value:
				return transaction

			await asyncio.sleep(1)  

		return None


	async def fetch_transaction_data_raw(self):
		
		print(f"processing transaction: {self.tx_signature_raw}")
		await asyncio.sleep(1)

		if "method" in self.tx_signature_raw and self.tx_signature_raw["method"] == "logsNotification":
			print("if = true")
			tx_signature_str = self.tx_signature_raw["params"]["result"]["value"]["signature"]
			print(f"Transaction Signature: {tx_signature_str}")

			transaction_details = await self.fetch_transaction_details(tx_signature_str)

			transaction_processing = TransactionProcessing(transaction_details, tx_signature_str)

			if transaction_details != None:
				if self.last_tx_id != tx_signature_str:
					self.last_tx_id = tx_signature_str
					
					if self.transaction_type == "sol_to_token":
						await transaction_processing.process_sol_to_token(self.receiver_wallet)

					if self.transaction_type == "token_to_sol":
						await transaction_processing.process_token_to_sol(self.receiver_wallet)

				else:	
					print("")
					print("already done, skipping")
					print("")

			else:
				print("")
				print("transaction details could not be fetched, skipping")
				print("")
		
		else:
			print("transaction is empty")


	async def process_transactions(self):

			while True:
				self.tx_signature_raw = await self.transaction_queue.get()
				print(f"Dequeued transaction: {self.tx_signature_raw}")
				await self.rate_limit.acquire()
				
				try:
					await self.fetch_transaction_data_raw()
				finally:
					self.rate_limit.release()
					self.transaction_queue.task_done()

