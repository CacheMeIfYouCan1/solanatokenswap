import json
import asyncio
import json
from solana.rpc.api import Client
from solders.pubkey import Pubkey
from solders.signature import Signature
import websockets
import subprocess

from resources.transaction_handling import TransactionHandling 

async def main():
	try:
		with open('config.json', 'r') as file:
			config = json.load(file)

		transaction_handling = TransactionHandling(
			config["token_receiver_wallet"],
			"token_to_sol"
		)

		listener_task = asyncio.create_task(transaction_handling.listen_for_transactions())
		processor_task = asyncio.create_task(transaction_handling.process_transactions())

		await asyncio.gather(listener_task, processor_task)

	except Exception as err:
		print("an error occured: ", err)

asyncio.run(main())

