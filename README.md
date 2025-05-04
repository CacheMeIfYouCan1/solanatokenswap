# Solana Token Swap
<center>
_This documentation is made for Debian based systems, also it looks best on a wide screen_
</center>
<br><br>
<center>
_repo: https://github.com/CacheMeIfYouCan1/SolanaTokenSwap_

</center>





## Table of Contents:

1. [Introduction](#introduction)
2. [Setup](#setup)
3. [Usage](#usage)
4. [Documentation](#documentation)
   1. [Structure](#structure)
   2. [Monitoring](#monitoring)	
   3. [Handling](#handling)
   4. [Smart Contracts](#smart_contracts)
5. [License](#license)


-----------------------------------------------------------------------------------------------

## Introduction:

<br>

This is a Proof of Concept for swapping custom tokens on the Solana blockchain for SOL and
vice-versa without an exchange. This solution provides the opportunity to quickly
perform token swaps directly on the blockchain, eliminating the barriers and complexities typically associated with centralized exchanges.

<br>
It consists of two handlers and two smart contracts written in Rust as well as two monitoring applications written in Python.
The python applications continuously monitor the Solana blockchain for relevant swap transactions.
Upon detecting a swap transaction, they trigger a corresponding Rust handler that interacts with a smart contract to perform the token swap.
<br>

**The current Python applications are only used for demonstation. If the Solana Token Swap system needs to be included into a productive
 environment, it is necessary to create a custom solution.**

<br>

### Addidtional considerations regarding security:

This system acts as a proof of concept, means it is designed to run locally, without generating costs. Therefore the private keys are stored locally and passed
as arguments to the handlers. **This is not secure.** Do not use this method with funded wallets on the mainnet.

There are secure ways to manage private keys for these scenarios, some of them are Cloud KMS solutions or private Keystore servers, where the keys are encrypted.
Storing private keys in environmental variables can also be done safely **if its done correctly**.

##### Please feel free to contact me, if assistance in developing a custom solution and/or customization of the given system is needed. 

<br>
<br>

-----------------------------------------------------------------------------------------------

## Setup:

### Clone the project:

<br>

initially we need to clone the gihub repository by running:
<br>
<br>

<pre><code>

$ git clone https://github.com/CacheMeIfYouCan1/SolanaTokenSwap/

</code></pre>

<br>
<br>


### Rust base configuration

Rust needs to be correctly set up for developing smart contracts on the solana blockchain. To ensure this, build-essentials need to be installed.
to do this, update your system and run:
<br>
<br>

<pre><code>$ sudo apt install -y build-essential curl</code></pre>

<br>

Afterwards we need to install rustup. Rustup is a tool to manage Rust versions and components. To Install rustup, run:
<br>
<br>

<pre><code>$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh</code></pre>

<br>
Finally we can source the environment:
<br><br>

<pre><code>$ source $HOME/.cargo/env
</code></pre>

<br>
<br>

Now we can check if the installation was successfull:
<br><br>

<pre><code>$ rustc --version</code></pre>

<br>


### Solana configuration
<br><br>

First we need to install the solana CLI. To do this we can run following command:
<br>
<br>

<pre><code>$ sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"</code></pre>

<br> 
afterwards we need to update the PATH:
<br><br>

<pre><code>$ echo 'export PATH="$HOME/.local/share/solana/install/active_release/bin:$PATH"' >> ~/.bashrc


$ source ~/.bashrc
</code></pre>  

Now we can check if it is properly installed:
<br>
<br>

<pre><code>$ solana --version</code></pre>

<br>

If solana is properly configured, we need to install the rustup component

<br>

<pre><code>$rustup component add rust-src</code></pre>

<br>

and also install spl-token:
<br>

<pre><code>$ cargo install spl-token-cli
</code></pre>

<br>

### Python configuration

Make sure Python is installed and properly configured. Afterwards we need to install virtualvenv by running the following command.
<br>
<br>

<pre><code>$ pip install virtualenv </code></pre>

<br>

now we can create a virtual environment for the monitoring systems and source it by running this from /monitoring/
<br>
<br>

<pre><code>$ python3 -m venv SolanaTokenSwapVenv </code></pre>
<pre><code>$ source SolanaTokenSwapVenv/bin/activate </code></pre>

<br>

and finally install the required libraries by running following command
<br>
<br>


<pre><code>$ pip install -r requirements.txt </code></pre>

<br>

### compilation:

To compile all parts of this system you can run following command from SolanaTokenSwap/ directory:

<pre><code>
$ bash build.sh

</code></pre>


### Test validator and wallet configuration

To run the local demo, we need to configure the local environment and propagate the global variables used in the monitoring scripts and passed to the handlers.

To do so, we need to start the test validator first:

<pre><code>$ solana-test-validator</code></pre>

<br>

in addition to that we need to configure solana to use the test validator:

<pre><code>$ solana config set --url http://127.0.0.1:8899</code></pre>

<br>

now we need to create a wallet, which will manage the token, make it the main wallet and airdrop it some SOL:

<pre><code>$ solana-keygen new --outfile ~/token-manager.json

$ solana config set --keypair ~/token-manager.json

$ solana airdrop 100

</code></pre>

<br>

next we need a wallet which will act as a user which swaps SOL to our token and our token to SOL. This wallet also needs some SOL.

<pre><code>$ solana-keygen new --outfile ~/user-wallet.json

$ solana airdrop 100 <WALLET\_PUBLIC\_KEY>

</code></pre>

next we need a wallet which will act as a comission wallet, collecting the fees.

<pre><code>$ solana-keygen new --outfile ~/comission-wallet.json

$ solana airdrop 100 <WALLET\_PUBLIC\_KEY>

</code></pre>

<br>

now we are set to create our demo-token:

<pre><code>$ spl-token create-token --owner ~/token-manager.json
</code></pre>

<br>

<br>

after creating a token, we need to create an associated token account, which will hold our token-liquidity:

<pre><code>$ spl-token create-account <TOKEN_ADDRESS> --owner ~/token-manager.json
</code></pre>

<br>

<br>

finally we have to mint some tokens as a base supply:

<pre><code>$ spl-token mint <TOKEN_ADDRESS> <QUANTITY>
</code></pre>

<br>


### deploying smart contracts:

At this point we need to deploy our smart contracts. Our system relies on two smart contracts which are :

##### send_sol:
This smart contract is called if a client swaps tokens back to SOL, or if fees are paid.

It can be deployed with following command from ./smart\_contracts/send\_sol/target/deploy/:


<pre><code>$  solana program deploy send_sol.so
</code></pre>

<br>

##### send_token:
This smart contract is called if a client swaps SOL to tokens

It can be deployed with following command from ./smart\_contracts/send\_token/target/deploy/:


<pre><code>$  solana program deploy send_token.so
</code></pre>

<br>



##### important note: make sure to save the program ID's


### final steps

Now we are finished and can populate the global variables with the values from our previous setp. The global variables are stored in /config.json and are used by our proof of concept.
Below you can see an example config.json with the values that have been created for the setup used to create this documentation. The commission_rate and the swap_rate can be individually chosen
and are not derived from the setup.

_fyi: The setup runs fully local, no actual secrets were shared_

<br><br>

<pre><code>
{
  "send_sol_handler": "/home/main/solanaTokenSwap/handling/send_sol_handler/target/debug/send_sol_handler",
  "send_token_handler": "/home/main/solanaTokenSwap/handling/send_token_handler/target/debug/send_token_handler",
  "send_sol_program_id": "BPP6ULzktGPJJNdXJAjv5o1uAAeUgFYWVEeZisyCDJmo",
  "send_token_program_id": "u8STJAr3h6hqb7XnYMiNqvzJrULgzfMxLLftGmCzGVy",
  "keypair_file": "/home/main/token-manager.json",
  "mint_account": "28wGsufhHpuQDBkeTXpNVLQz2zPUBoLc1gDpu7wBtejg", 
  "owner": "58ZEW3nzhg3CQ8xYrqavsV71r6ErKX7a4PyJrrxc4pbE",
  "token_receiver_wallet": "3vSighS9MqPBcZJbYWnw8AWvYw3NnvY58nf9QbNmUPnr",
  "sol_receiver_wallet": "58ZEW3nzhg3CQ8xYrqavsV71r6ErKX7a4PyJrrxc4pbE", 
  "commission_wallet": "58ZEW3nzhg3CQ8xYrqavsV71r6ErKX7a4PyJrrxc4pbE",  
  "token_acc_str": "28wGsufhHpuQDBkeTXpNVLQz2zPUBoLc1gDpu7wBtejg", 
  "rpc_client_str": "http://127.0.0.1:8899",
  "ws_client_str": "ws://127.0.0.1:8900",
  "commission_rate": "0.1",
  "swap_rate": "200"
}
</code></pre>



-----------------------------------------------------------------------------------------------

## Usage:

To swap SOL for the custom, token using our local ledger, we will first start the two monitoring scripts, in two seperate terminals:


<br>

<pre><code>

~/SolanaTokenSwap/monitoring $ SolanaTokenSwapVenv/bin/python3 scripts/token\_monitor.py


~/SolanaTokenSwap/monitoring $ SolanaTokenSwapVenv/bin/python3 scripts/sol\_monitor.py

</code></pre>

<br>
<br>

before any transactions happen, the monitoring should look like this:

<br>

<center>
![[sts_init.png]](/projects/4/postPage/sts_init.png)
</center>

<br><br>
<br><br>

Now we can check the initial SOL and token balances of our user wallet:

<br>


<center>
![[sts_wallet_init_balance.png]](/projects/4/postPage/sts_wallet_init_balance.png)
</center>

<br>
<br>
<br>

To swap SOL for our custom token, we can transfer SOL from our user wallet to our token management wallet. The monitoring systems
will register the transaction and execute the corresponding handlers, which will call the smart contracts to perform the token swap.

<br><br>

<center>

![[sts_sol_tx_user_wallet.png]](/projects/4/postPage/sts_sol_tx_user_wallet.png)

<br><br>

![[sts_sol_tx_monitoring.png]](/projects/4/postPage/sts_sol_tx_monitoring.png)

</center>

<br>
<br>
<br>

after the transactions are finished we can check the new balances of our user wallet:

<br>
<br>

<center>
![[sts_user_wallet_updated_balance_sol_tx.png]](/projects/4/postPage/sts_user_wallet_updated_balance_sol_tx.png)

</center>


<br>
<br>
<br>

We can also swap our custom token to SOL by sending the custom token to the associated token account of our token management wallet.
The monitoring systems will also register this transaction and perform the swap by calling the corresponding smart contracts.


<br><br>

<center>

![[sts_tx_token_user_wallet.png]](/projects/4/postPage/sts_tx_token_user_wallet.png)

<br><br>

![[sts_updated_monitoring_token_tx.png]](/projects/4/postPage/sts_updated_monitoring_token_tx.png)

</center>

<br>
<br>
<br>

afterwards we can also check the updated balances of our user wallet:

<br>
<center>
![[sts_updated_balance_token_tx.png]](/projects/4/postPage/sts_updated_balance_token_tx.png)

</center>


<br>
<br>
<br>





-----------------------------------------------------------------------------------------------

## Documentation:

### Structure

<pre>
<code>
SolanaTokenSwap/
|- build.sh
|- config.json
|- handling/
| |- send\_sol\_handler/
| | |- Cargo.toml
| | |- src/
| | | |- main.rs
| |- send\_token\_handler
| | |- Cargo.toml
| | |- src/
| | | |- main.rs
|- smart\_contracts/
| |- send\_sol/
| | |- Cargo.toml
| | |- src/
| | | |- lib.rs
| |- send\_token/
| | |- Cargo.toml
| | |- src/
| | | |-lib.rs
|- monitoring/
| |- requirements.txt
| |- scripts/
| | |- token\_monitor.py
| | |- sol\_monitor.py
| | |- resources/
| | | |- transaction_processing.py
| | | |- transaction_handling.py
| |- SolanaTokenSwapVenv/
| | |- (...)
</code>
</pre>

------------------------------------------------------------------------------------------------

### Monitoring

<br>

The monitoring system is used to demonstrate the functionality of this Proof of concept by managing
the interactions between the Rust handlers and the smart contracts. It consists of four components,
 to monitor the transactions, as well as hantle swapping from SOL to the custom token, as well as
 swapping the custom token to SOL.

<br><br><br>

#### Initialisation:

sol\_monitor.py and token\_monitor.py are used to initialize the monitoring systems. They import the TransactionHandling
class and are structured as following: 

<br><br>

##### sol\_monitor.py:

<br>

<pre><code>

async def main():
	try:
		with open('config.json', 'r') as file:
			config = json.load(file)

		transaction\_handling = TransactionHandling(
			config["sol\_receiver\_wallet"],
			"sol\_to\_token"
		)

		listener\_task = asyncio.create\_task(transaction\_handling.listen\_for\_transactions())
		processor\_task = asyncio.create\_task(transaction\_handling.process\_transactions())

		await asyncio.gather(listener\_task, processor\_task)

	except Exception as err:
		print("ann error occured: ", err)
	
asyncio.run(main())



</code></pre>

<br><br>


This script initializes the monitoring and processing of swap transactions from Solana (SOL) to a custom token.
It loads configuration from a config.json file, including the receiver wallet and transaction type, then creates a TransactionHandling object.

Two asynchronous tasks are created:
<br>
<br>


+ listen\_for\_transactions(): Monitors incoming transactions.
+ process\_transactions(): Processes the transactions.


<br>
Both tasks run concurrently using asyncio.gather(), allowing real-time handling of transactions.



<br><br>

##### token\_monitor.py:

<br>

<pre><code>
async def main():
	try:
		with open('config.json', 'r') as file:
			config = json.load(file)

		transaction_handling = TransactionHandling(
			config["token\_receiver\_wallet"],
			"token\_to\_sol"
		)

		listener\_task = asyncio.create\_task(transaction\_handling.listen\_for\_transactions())
		processor\_task = asyncio.create\_task(transaction\_handling.process\_transactions())

		await asyncio.gather(listener\_task, processor\_task)

	except Exception as err:
		print("an error occured: ", err)

asyncio.run(main())

</code></pre>

<br><br>

This script works similar to sol_monitor.py, but instead of initialising the monitoring and processing of swap transactions from SOL to a custom token,
it initializes the monitoring and processing of swap transactions from a custom token to SOL.

<br><br><br>

#### Transaction Handling:

<br><br>

#####transaction\_handling.py:

<br>

Handling of the transaction is managed through the TransactionHandling class. This involves listening for incoming transactions,
fetching the transaction details as well as processing the swap, based on the given transaction type. It takes the receiver wallet and the transaction types as arguments.


<br><br>

##### \_\_innit\_\_():

<pre>
<code>

def \_\_init\_\_(self, receiver\_wallet, transaction\_type):

		self.receiver\_wallet = receiver\_wallet
		self.transaction\_type = transaction\_type
	
		self.transaction\_queue = asyncio.Queue()
		self.rate\_limit = asyncio.Semaphore(5)

		with open('config.json', 'r') as file:
			config = json.load(file)
		
		self.client = Client(config["rpc\_client\_str"])
		self.socket = config["ws\_client\_str"]
		
		self.send\_token\_program\_id = config["send\_token\_program\_id"]
		self.send\_sol\_program\_id = config["send\_sol\_program\_id"]

		self.last\_tx\_id = "0";

</code></pre>

<br><br>

Initializes the class with essential parameters and sets up internal resources.

<br>

###### __Parameters:__

+ receiver\_wallet: The Solana wallet address that will receive the transaction, initializing the swap.

+ transaction\_type: Defines the transaction direction, either "sol_to_token" or "token_to_sol".

<br>

###### __Key Actions:__
<br> 
Creates an asyncio.Queue (transaction_queue) to store incoming transactions, sets up a rate limit using an asyncio.Semaphore
to manage concurrent requests (maximum of 5 simultaneous requests) and Loads configuration from config.json to retrieve:

+ RPC client connection string (rpc\_client\_str).

+ WebSocket URL (ws\_client\_str) for listening to transaction logs.

+ Program IDs for token and SOL transactions (send\_token\_program\_id, send\_sol\_program\_id).

+ Initializes the last\_tx\_id to track the last processed transaction.


<br><br><br>

##### listen\_for\_transactions():

<pre><code>

async def listen\_for\_transactions(self):

		async with websockets.connect(self.socket) as websocket:
		
			subscription\_request = {
				"jsonrpc": "2.0",
				"id": 1,
				"method": "logsSubscribe",
				"params": [
					{
						"mentions": [f"{self.receiver\_wallet}"]
					},
					{
						"commitment": "finalized"
					}
				]
			}

			await websocket.send(json.dumps(subscription\_request))
			
			while True:
				response = await websocket.recv()

				self.tx\_signature\_raw = json.loads(response)
				print("Transaction log received:", self.tx\_signature\_raw)

				await self.transaction\_queue.put(self.tx\_signature\_raw)
				print(f"Transaction added to queue: {self.tx\_signature\_raw}")

</code></pre>

<br><br>

Listens for incoming transactions involving the receiver wallet by subscribing to the blockchain logs.

<br>

###### __Key Actions:__

<br>

This function runs indefinitely in an asynchronous loop to monitor real-time transactions and Continuously listens for
incoming log data, by establishing a WebSocket connection to the configured socket URL, sending a subscription request to the blockchain logs and
filtering for transactions involving the receiver_wallet. When a transaction is detected, it adds the transaction details (tx_signature_raw) to the transaction_queue.


<br><br><br>
##### process\_transactions():

<pre><code>

async def process\_transactions(self):

		while True:
			self.tx\_signature\_raw = await self.transaction\_queue.get()
			print(f"Dequeued transaction: {self.tx\_signature\_raw}")
			await self.rate\_limit.acquire()
			
			try:
				await self.fetch\_transaction\_data\_raw()
			finally:
				self.rate\_limit.release()
				self.transaction\_queue.task\_done()

</code></pre>

<br><br>

Continuously fetches transactions from the queue and processes them with rate-limiting controls.

<br>

###### __Key Actions:__

<br>

This function runs in an infinite loop, ensuring that the system continuously processes transactions as they are received. It
asynchronously dequeues a transaction from transaction_queue, Acquires a Semaphore to ensure no
more than 5 transactions are processed at once. Calls fetch_transaction_data_raw() for each transaction in the queue
and releases the rate limit (Semaphore) after processing the transaction to allow the next one.

        
<br>
<br>
<br>


##### fetch\_transaction\_data\_raw():
<br>

<pre><code>

async def fetch_transaction_data_raw(self):
	
	print(f"processing transaction: {self.tx\_signature_raw}")
	await asyncio.sleep(1)

	if "method" in self.tx\_signature\_raw and self.tx\_signature\_raw["method"] == "logsNotification":
		print("if = true")
		tx\_signature\_str = self.tx\_signature\_raw["params"]["result"]["value"]["signature"]
		print(f"Transaction Signature: {tx\_signature\_str}")

		transaction\_details = await self.fetch\_transaction\_details(tx\_signature\_str)

		transaction\_processing = TransactionProcessing()

		if transaction\_details != None:
			if self.last\_tx\_id != tx\_signature\_str:
				self.last\_tx\_id = tx\_signature\_str
				
				if self.transaction\_type == "sol\_to\_token":
					await transaction\_processing.process\_sol\_to\_token(transaction\_details, tx\_signature\_str, self.receiver\_wallet)

				if self.transaction\_type == "token\_to\_sol":
					await transaction\_processing.process\_token\_to\_sol(transaction\_details, tx\_signature\_str, self.receiver\_wallet)

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
		
</code></pre>

<br><br>


Processes a raw transaction log and extracts the relevant details for further processing.

<br>

###### __Key Actions:__
<br>
If the transaction is valid and contains a method (logsNotification), it extracts the transaction signature (tx\_signature\_str)
and then calls fetch\_transaction\_details() to retrieve the full transaction details.
<br>
After fetchung the full transaction details, it initializes the TransactionProcessing class to handle the specific
processing logic based on the transaction type. If the full transaction details can not be fetched, the processing is skipped.

<br>

###### __Transaction Types:__
<br>

+ sol\_to\_token: processes the transaction using process\_sol\_to\_token().

+ token\_to\_sol: processes the transaction using process\_token\_to\_sol().

<br>
Processing is skipped, if the transaction has already been processed (if the signature matches last\_tx\_id). This ensures that each
transaction is processed only once, preventing duplication.

<br>
<br>
<br>

##### fetch\_transaction\_details():


<pre><code>
async def fetch\_transaction\_details(self, tx\_signature\_str):

		tx_signature = Signature.from\_string(tx\_signature\_str)

		for i in range(10): 
			transaction = self.client.get\_transaction(tx\_signature)
			if transaction.value:
				return transaction

			await asyncio.sleep(1)  

		return None
</code></pre>


<br><br>

Fetches detailed information for a given transaction using its signature.

<br>
###### __Key Actions:__
<br>

Converts the provided transaction signature (tx\_signature\_str) into a Signature object and a
ttempts to fetch the transaction details using the client (self.client.get\_transaction(tx\_signature)), retrying up to 10 times with a
1-second delay between each attempt. If transaction details are found, it returns the transaction; if not, it returns None after 10 attempts.
<br>
<br>
This ensures the system can handle potential network issues or delays when fetching data from the blockchain.


#### transaction\_handling.py:

<br>

This class handles the processing of Solana transactions, including the conversion of SOL to tokens and tokens back to SOL.
It uses data fetched from transaction logs and interacts with smart contracts to facilitate the conversion and manage commissions.

<br>

interaction with smart contracts is done by using subprocesses, calling external handlers (e.g., send\_sol\_handler, send\_token\_handler)
for token transfers and SOL transfers.Commission rates and swap rates are applied to calculate the amounts to be sent or received.
Transactions are skipped if they have already been processed (based on the transaction signature)

<br>
<br>


##### __init__():

<br>

The constructor initializes the TransactionProcessing class with transaction details and transaction signature.
It also loads configuration settings from a config.json file, which are used to interact with the smart contracts and manage wallet addresses.

<br><br>

###### __Key Actions:__
<br>

Loads configuration settings from config.json, extracts transaction information like account keys and instructions and initializes
necessary fields, such as the wallet addresses and program IDs. 

<br><br>

#### async process\_sol\_to\_token():

<pre><code>

async def process\_sol\_to\_token(self, sol\_receiver\_wallet):
	
	if self.transaction\_details and self.transaction\_details.value:
				
		for instruction in self.instructions:
			sender\_index = instruction.accounts[0]  
			sender\_account = self.account\_keys[sender\_index]

			commission\_wallet = self.commission\_wallet
			
			sender\_account\_str = str(sender\_account)

			if sender\_account\_str != self.token\_account\_str:
				if sender\_account\_str != sol\_receiver\_wallet:

					pre\_balances = self.transaction\_details.value.transaction.meta.pre\_balances
					post\_balances = self.transaction\_details.value.transaction.meta.post\_balances

					lamports\_sent = pre\_balances[0]-post\_balances[0]
					sol\_to\_send\_lamports\_int = int(lamports\_sent*float(self.commission\_rate))
					tokens\_to\_send\_lamports = lamports\_sent*int(self.swap\_rate)
				
					print("Token purchased, executing token transfer")
					call\_token\_smartcontract = [
						self.send\_token\_handler,
						"--receiver", str(sender\_account),
						"--amount", str(tokens\_to\_send\_lamports).lstrip('-'),
						"--program-id-arg", self.send\_token\_program\_id,
						"--rpc-url-arg", self.rpc\_client\_str,
						"--keypair-file", self.keypair\_file,
						"--mint-account-arg", self.mint\_account,
						"--owner-arg", self.commission\_wallet
						]
					result\_token = subprocess.run(call\_token\_smartcontract, capture\_output= True, text= True)
					print(f"result token tx: {result\_token.stdout}")
					print(f"{result\_token.stderr}")

					print("executing transfer of commission")
					call_\sol\_smartcontract = [
							self.send\_sol\_handler,
							"--receiver", self.commission\_wallet,
							"--amount", str(sol\_to\_send\_lamports\_int).lstrip('-'),
							"--program-id-arg", self.send\_sol\_program\_id,
							"--rpc-url-arg", self.rpc\_client\_str,
							"--keypair-file", self.keypair\_file
							]	
					result\_sol = subprocess.run(call\_sol\_smartcontract, capture\_output= True, text= True)
					print(f"result sol tx: {result\_sol.stdout}")
					print(f" {result\_sol.stderr}")
					
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

</code></pre>

<br>
<br>
<br>

Processes a transaction where SOL is sent and exchanged for tokens. If the transaction is valid and the sender is not the receiver,
it executes a smart contract to transfer tokens. The method also ensures that a commission is sent to a specified commission wallet.

<br>
<br>

###### __Key Actions:__

<br>

Loops through the transaction's instructions to identify the sender andhecks if the sender is the receiver; if not, it calculates the amount of SOL to convert into tokens.
Then executes a smart contract to transfer tokens using the send\_token\_handler and sends a commission to the commission wallet via another smart contract.
If the sender is the receiver or if it’s already a token transaction, the transaction gets skipped.

<br>
<br>

#### async process\_token\_to\_sol():
<br>

<pre><code>


async def process\_token\_to\_sol(self, token\_receiver\_wallet):
	
	if self.transaction\_details and self.transaction\_details.value:
		
		for instruction in self.instructions:

			if instruction.accounts:
				sender\_index = instruction.accounts[0]
				sender\_associated\_account\_owner = self.account\_keys[0]

				for balance\_meta in self.transaction\_details.value.transaction.meta.pre\_token\_balances:
					if balance\_meta.account\_index == sender\_index:
						sender\_associated\_account\_owner = balance\_meta.owner
						break
						
				else:
					sender\_associated\_account\_owner = None
					
				for pre\_balance\_meta in self.transaction\_details.value.transaction.meta.pre\_token\_balances:
					if pre\_balance\_meta.account\_index == sender\_index:
						pre\_token\_balance = pre\_balance\_meta.ui\_token\_amount.amount
						break
						
				else:
					pre\_token\_balance = None		
						
				for post\_balance\_meta in self.transaction\_details.value.transaction.meta.post\_token\_balances:
					if post\_balance\_meta.account\_index == sender\_index:
						post\_token\_balance = post\_balance\_meta.ui\_token\_amount.amount
						break
				else:
					post\_token\_balance = None
							
				if sender\_associated\_account\_owner is not None:
					lamports\_sent = int(pre\_token\_balance) - int(post\_token\_balance) 

					sol\_to\_send\_lamports\_int = int(lamports\_sent/int(self.swap\_rate))
					sol\_to\_send\_lamports\_str = str(sol\_to\_send\_lamports\_int)

					sender\_associated\_account\_owner\_str = str(sender\_associated\_account\_owner)
					if sender\_associated\_account\_owner != token\_receiver\_wallet:
									
						print("Token sold: executing SOL transfer")

						call\_sol\_smartcontract = [
							self.send\_sol\_handler,
							"--receiver", str(sender\_associated\_account\_owner),
							"--amount", sol\_to\_send\_lamports\_str,
							"--program-id-arg", self.send\_sol\_program\_id,
							"--rpc-url-arg", self.rpc\_client\_str,
							"--keypair-file", self.keypair\_file
						]								
						result\_sol = subprocess.run(call\_sol\_smartcontract, capture\_output= True, text= True)
						print(f"result sol: {result\_sol.stdout}")
						print(f"error sol: {result\_sol.stderr}")
							
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

</code></pre>

<br>
<br>
<br>

Processes a transaction where tokens are sold and converted back into SOL. It identifies the sender’s associated account and calculates the amount of SOL to transfer.
A smart contract is executed to send the SOL to the sender’s wallet.

<br>
<br>

###### __Key Actions:__

<br>

Loops through the transaction’s token balances to determine the amount of tokens transferred and calculates the corresponding amount of SOL to be sent to the sender.
Then executes a smart contract to send the SOL to the sender. If the sender is the receiver or if no valid token balances are found, the transaction is skipped.
<br><br>

------------------------------------------------------------------------------------------------

### Handling

<br>

The Rust handler programs serve as the core components of the system, facilitating interactions with Solana's smart contracts to manage SOL-to-token and token-to-SOL transactions.
They are responsible for constructing, signing, and sending Solana transactions that either convert SOL to tokens or vice versa.

<br>
<br>

#### send\_sol\_handler:
<br>

#### invoke\_smart\_contract():

<br>

<pre><code>

fn invoke\_smart\_contract(
	client: &RpcClient,
	payer\_keypair: &Keypair,
	sol\_recipient: Pubkey,
	sol\_quantity: SolQuantity) -> Result\<(), Box\<dyn std::error::Error\>\> {

	let opts: Opts = Opts::parse();	
    
    let program\_id = opts.program\_id\_arg.parse().unwrap();

    let accounts = vec![
		AccountMeta::new(payer\_keypair.pubkey(), true),
		AccountMeta::new(sol\_recipient, false),
		AccountMeta::new\_readonly(system\_program::id(), false),
    ]; // Add relevant accounts

    // Create the instruction
    let instruction = Instruction {
			program\_id,
			accounts,
			data: sol\_quantity.amount.to\_le\_bytes().to\_vec(), // Convert the amount to bytes
		};

		// Create the transaction
		let mut transaction = Transaction::new\_with\_payer(&[instruction], Some(&payer\_keypair.pubkey()));
		
		println!("{}", payer\_keypair.pubkey());
		// Sign the transaction
		let recent\_blockhash = client.get\_latest\_blockhash().unwrap();
		transaction.sign(&[payer\_keypair], recent\_blockhash);

		// Send the transaction
		let signature = client.send\_and\_confirm\_transaction(&transaction).unwrap();
		println!("Transaction signature: {:?}", signature);

		Ok(())
	}
}

</code></pre>

<br>
<br>
<br>

This function constructs and sends a Solana transaction invoking a smart contract to transfer SOL from the payer to a recipient.
It uses the provided RPC client, payer keypair, recipient’s public key, and amount of SOL to create and send the transaction.

<br>
<br>

###### __Key Actions:__

<br>

Parses command-line arguments using Opts and builds a transaction instruction to transfer SOL from the payer to the recipient.
Then Signs the transaction with the payer's keypair and sends it to the Solana network afterwards prints the transaction signature upon successful submission.

<br><br>

###### __Parameters:__

<br>

* client: The RPC client to interact with the Solana network.

* payer\_keypair: The payer’s keypair, which is used to sign the transaction.

* sol\_recipient: The recipient’s public key (Solana address) where the SOL will be sent.

* sol\_quantity: The amount of SOL to be transferred.

<br>
<br>

###### __Return:__

<br>

Result\<(), Box\<dyn std::error::Error\>\>: Returns Ok(()) on success, or an error if the transaction fails.

<br>
<br>

###### __Transaction Flow:__

<br>

1. The program ID is parsed from command-line arguments.

2. The instruction is constructed with the relevant accounts and data (SOL amount).

3. A transaction is created, signed, and sent to the Solana network using the provided RPC client.

<br>
<br>

#### main():

<pre><code>

fn main() -\> Result\<(), Box\<dyn std::error::Error\>\> {
	let opts: Opts = Opts::parse();
    
    let rpc\_url = opts.rpc\_url\_arg;
    let client = RpcClient::new(rpc\_url);

    let payer\_keypair = read\_keypair\_file(opts.keypair\_file);
   
    let amount\_lamp = opts.amount;
    let sol\_recipient = Pubkey::from\_str(&opts.receiver);

    invoke\_smart\_contract(&client, &payer\_keypair?, sol\_recipient?, SolQuantity { amount: amount\_lamp });

Ok(())


}

</code></pre>

<br>
<br>
<br>

The entry point for the program. It parses the command-line arguments, sets up the RPC client,
reads the payer's keypair from a file, and invokes the smart contract to transfer SOL to a recipient.

<br>

###### __Key Actions:__
<br>

Parses the command-line arguments using Opts and creates an RpcClient with the provided RPC URL. Reads the payer’s keypair from the specified file and
calls invoke\_smart\_contract to initiate the SOL transfer.

<br>
<br>

###### __Return:__
<br>

Result\<(), Box\<dyn std::error::Error\>\>: Returns Ok(()) if the program executes successfully, or an error if any operation fails.
<br>
<br>


#### send\_token\_handler:
<br>


#### invoke\_smart\_contract():

<br>

<pre><code>
fn invoke\_smart\_contract(
	client: &RpcClient,
	mint\_account: Pubkey,
	from\_associated\_token\_account: Pubkey,
	to\_associated\_token\_account: Pubkey,
	owner: Pubkey, //owner is the from wallet which owns the token acc which sends
	recipient: Pubkey, //recipient is the sol wallet which owns the token acc which will receive balance
	payer\_keypair: &Keypair,
	token\_amount: TokenAmount) -\> Result\<(), Box\<dyn std::error::Error\>\> {
	let opts: Opts = Opts::parse();
   
    let program\_id = opts.program\_id\_arg.parse().unwrap();

    let accounts = vec![
		
		AccountMeta::new(mint\_account, false),
		AccountMeta::new(from\_associated\_token\_account, false),
		AccountMeta::new(to\_associated\_token\_account, false),
		AccountMeta::new(owner, false),
		AccountMeta::new(recipient, false),
		AccountMeta::new(payer\_keypair.pubkey(), true),	
		AccountMeta::new\_readonly(system\_program::id(), false),
		AccountMeta::new\_readonly(TOKEN\_PROGRAM\_ID, false),
		AccountMeta::new\_readonly(ASSOCIATED\_TOKEN\_PROGRAM\_ID, false),
    ]; // Add relevant accounts
	println!("starting program");
	
	let amount = token\_amount.token\_amount; 
	println!("Create the instruction");
	
    // Create the instruction
    let instruction = Instruction {
        program\_id,
        accounts,
        data: amount.to\_le\_bytes().to\_vec(), 
    };
	println!("successfully created instruction");
	println!("createing tx");

    // Create the transaction
    let mut transaction = Transaction::new\_with\_payer(&[instruction], Some(&payer\_keypair.pubkey()));
	println!("get latest blockhash");
	
    // Sign the transaction
    let recent\_blockhash = client.get\_latest\_blockhash().unwrap();
	println!("got blockhash");
	println!("sign tx");
	let signer = [payer\_keypair];
	println!("{}", payer\_keypair.pubkey());
    transaction.sign(&[payer\_keypair], recent\_blockhash);
	println!("signed tx");
	
    // Send the transaction
    let signature = client.send\_and\_confirm\_transaction(&transaction).unwrap();
    println!("Transaction signature: {:?}", signature);

    Ok(())
}


</code></pre>

<br>
<br>
<br>

This function creates and sends a transaction that interacts with a Solana smart contract to transfer tokens from one associated token account to another.
 It is designed to handle transfers of a specified token between the owner (sender) and the recipient (receiver) on the Solana blockchain.

<br>
<br>

###### __Key Actions:__

<br>

Parses the command-line arguments using Opts and constructs a list of relevant accounts needed for the transaction, including the mint account,
sender’s and recipient’s associated token accounts, and the payer's keypair. Creates a Solana transaction instruction with the appropriate accounts and data (token amount).
Signs the transaction and sends it to the Solana network using the provided RPC client.

<br><br>

###### __Parameters:__

<br>

* client: The RPC client used to interact with the Solana network.

* mint\_account: The public key of the token’s mint account, which specifies the token type being transferred.

* from\_associated\_token\_account: The sender’s associated token account.

* to\_associated\_token\_account: The recipient’s associated token account.

* owner: The public key of the wallet that owns the token account from which tokens are being sent.

* recipient: The public key of the wallet receiving the tokens.

* payer\_keypair: The payer’s keypair used to sign the transaction.

* token\_amount: The amount of tokens to be transferred.

<br>
<br>

###### __Return:__

<br>

Result\<(), Box\<dyn std::error::Error\>\>: Returns Ok(()) on successful transaction submission, or an error if the transaction fails.

<br>
<br>

###### __Transaction Flow:__

<br>

1. Command-line arguments are parsed to obtain necessary parameters like program ID, token amounts, and wallet keys.

2. A transaction instruction is created to transfer tokens from the sender to the recipient.

3. The transaction is signed using the payer’s keypair and sent to the Solana network.

4. The transaction signature is printed upon success.

<br>
<br>

#### main():

<pre><code>


fn main() -\> Result\<(), Box\<dyn std::error::Error\>\> {
	let opts: Opts = Opts::parse();

	let rpc\_url = opts.rpc\_url\_arg;
    let client = RpcClient::new(rpc\_url);
    
    // create variables
    let mint\_account = Pubkey::from\_str(&opts.mint\_account\_arg)?;
  	let owner = Pubkey::from\_str(&opts.owner\_arg)?;//same as payer in our case
	let recipient = Pubkey::from\_str(&opts.receiver).expect("Invalid Key provided");
	let payer\_keypair = read\_keypair\_file(opts.keypair\_file);
	let from\_associated\_token\_account = spl\_associated\_token\_account::get\_associated\_token\_address(&owner, &mint\_account); 
    let to\_associated\_token\_account = spl\_associated\_token\_account::get\_associated\_token\_address(&recipient, &mint\_account);
	let amount\_lamp = opts.amount;
	
    invoke\_smart\_contract(
		&client,
		mint\_account,
		from\_associated\_token\_account,
		to\_associated\_token\_account,
		owner,
		recipient,
		&payer\_keypair?,
		TokenAmount { token\_amount: amount\_lamp }
		);

Ok(())

}

</code></pre>

<br>
<br>
<br>

The entry point for the program. This function sets up the RPC client, reads the payer’s keypair from the file,
and invokes the invoke\_smart\_contract function to transfer tokens between two Solana wallets.

<br>

###### __Key Actions:__
<br>

Parses the command-line arguments using Opts and creates an RpcClient with the provided RPC URL. Reads the payer’s keypair from the specified file and
calls invoke_smart_contract to initiate the SOL transfer.

<br>
<br>

###### __Return:__
<br>

Result\<(), Box\<dyn std::error::Error\>\>: Returns Ok(()) if the program executes successfully, or an error if any operation fails.
<br>
<br>

------------------------------------------------------------------------------------------------

### Smart Contracts:

<br><br>

These contracts handle the logic for transferring SOL and custom tokens between accounts. 

<br>

#### send_sol:
<br>

<pre><code>

fn transfer\_sol\_with\_cpi(
		program\_id: &Pubkey,
		accounts: &[AccountInfo],
		instruction\_data: &[u8],
	) -> ProgramResult {
    let accounts\_iter = &mut accounts.iter();
    let payer = next\_account\_info(accounts\_iter)?;
    let sol\_recipient = next\_account\_info(accounts\_iter)?;
    let system\_program = next\_account\_info(accounts\_iter)?;

	let data = transferSolArgs::try\_from\_slice(instruction\_data).map\_err(|\_| ProgramError::InvalidInstructionData)?;


    invoke(
        &system\_instruction::transfer(payer.key, sol\_recipient.key, data.sol\_quantity),
        &[payer.clone(), sol\_recipient.clone(), system\_program.clone()],
    )?;

    Ok(())
}

</code></pre>

<br>
<br>

<br>

This smart contract facilitates the transfer of SOL between accounts on the Solana blockchain,
utilizing a Cross-Program Invocation (CPI) to execute the transfer via the Solana system program.
The contract accepts an instruction that specifies the amount of SOL to be transferred and invokes the system_instruction::transfer function to perform the transaction.

<br>
<br>
The main components of the smart contract include:
<br>
<br>

* __transferSolArgs:__ A struct used to deserialize the instruction data, which contains the quantity of SOL to transfer.

* __CPI Execution:__ calls the system program via CPI to perform the actual SOL transfer between the payer (sender) and the recipient.

* __Account Validation:__ checks for the presence of necessary accounts, including the payer's account, the recipient's account, and the system program account.

* __Error Handling:__ If the instruction data is invalid or if the transfer cannot be executed, the program returns an appropriate error message.

<br>

By leveraging CPI, this smart contract enables secure transfers of SOL within the Solana ecosystem.

<br>
<br>
<br>


#### send_token:
<br>

<pre><code>

fn transfer\_sol\_with\_cpi(
		program\_id: &Pubkey,
		accounts: &[AccountInfo],
		instruction\_data: &[u8],
	) -> ProgramResult {
    let accounts\_iter = &mut accounts.iter();
    let payer = next\_account\_info(accounts\_iter)?;
    let sol\_recipient = next\_account\_info(accounts\_iter)?;
    let system\_program = next\_account\_info(accounts\_iter)?;

	let data = transferSolArgs::try\_from\_slice(instruction\_data).map\_err(|\_| ProgramError::InvalidInstructionData)?;


    invoke(
        &system\_instruction::transfer(payer.key, sol\_recipient.key, data.sol\_quantity),
        &[payer.clone(), sol\_recipient.clone(), system\_program.clone()],
    )?;

    Ok(())
}

</code></pre>

<br>
<br>

<br>

This smart contract facilitates the transfer of a custom token (such as an SPL token) between accounts on the Solana blockchain.
If the recipient does not have an associated token account for the specific token, the contract will automatically create one.
After ensuring the recipient has a valid associated token account, the contract proceeds to transfer the specified amount of tokens f
rom the sender's associated token account to the recipient's associated token account.
<br>
<br>
The main components of the smart contract include:
<br>
<br>
* __transferTokenArgs:__ A struct used to deserialize the instruction data, which contains the quantity of tokens to be transferred.

* __Associated Token Account Creation:__ Before transferring tokens, the contract checks whether the recipient has an associated token account.
If not, the contract invokes the create\_associated\_token\_account instruction to create one.

* __Token Transfer:__ The contract utilizes the spl\_token::instruction::transfer function to perform the token transfer from the sender to the recipient.

* __Account Validation:__ The contract checks the presence and validity of the necessary accounts, including the mint account,
 token accounts (both sender and recipient), the payer, and relevant program accounts (such as the system program, token program, and associated token program).

* __Error Handling:__ If the associated token account does not exist, it is created. If any errors occur during the transfer, they are propagated to the caller.

<br> 

By leveraging CPI (Cross-Program Invocation) and the associated token account functionality, this contract ensures a secure
 token transfer process, even if the recipient doesn't have an existing associated token account.

<br><br>

------------------------------------------------------------------------------------------------

## License:

Creative Commons Legal Code

CC0 1.0 Universal

    CREATIVE COMMONS CORPORATION IS NOT A LAW FIRM AND DOES NOT PROVIDE
    LEGAL SERVICES. DISTRIBUTION OF THIS DOCUMENT DOES NOT CREATE AN
    ATTORNEY-CLIENT RELATIONSHIP. CREATIVE COMMONS PROVIDES THIS
    INFORMATION ON AN "AS-IS" BASIS. CREATIVE COMMONS MAKES NO WARRANTIES
    REGARDING THE USE OF THIS DOCUMENT OR THE INFORMATION OR WORKS
    PROVIDED HEREUNDER, AND DISCLAIMS LIABILITY FOR DAMAGES RESULTING FROM
    THE USE OF THIS DOCUMENT OR THE INFORMATION OR WORKS PROVIDED
    HEREUNDER.

Statement of Purpose

The laws of most jurisdictions throughout the world automatically confer
exclusive Copyright and Related Rights (defined below) upon the creator
and subsequent owner(s) (each and all, an "owner") of an original work of
authorship and/or a database (each, a "Work").

Certain owners wish to permanently relinquish those rights to a Work for
the purpose of contributing to a commons of creative, cultural and
scientific works ("Commons") that the public can reliably and without fear
of later claims of infringement build upon, modify, incorporate in other
works, reuse and redistribute as freely as possible in any form whatsoever
and for any purposes, including without limitation commercial purposes.
These owners may contribute to the Commons to promote the ideal of a free
culture and the further production of creative, cultural and scientific
works, or to gain reputation or greater distribution for their Work in
part through the use and efforts of others.

For these and/or other purposes and motivations, and without any
expectation of additional consideration or compensation, the person
associating CC0 with a Work (the "Affirmer"), to the extent that he or she
is an owner of Copyright and Related Rights in the Work, voluntarily
elects to apply CC0 to the Work and publicly distribute the Work under its
terms, with knowledge of his or her Copyright and Related Rights in the
Work and the meaning and intended legal effect of CC0 on those rights.

1. Copyright and Related Rights. A Work made available under CC0 may be
protected by copyright and related or neighboring rights ("Copyright and
Related Rights"). Copyright and Related Rights include, but are not
limited to, the following:

  i. the right to reproduce, adapt, distribute, perform, display,
     communicate, and translate a Work;
 ii. moral rights retained by the original author(s) and/or performer(s);
iii. publicity and privacy rights pertaining to a person's image or
     likeness depicted in a Work;
 iv. rights protecting against unfair competition in regards to a Work,
     subject to the limitations in paragraph 4(a), below;
  v. rights protecting the extraction, dissemination, use and reuse of data
     in a Work;
 vi. database rights (such as those arising under Directive 96/9/EC of the
     European Parliament and of the Council of 11 March 1996 on the legal
     protection of databases, and under any national implementation
     thereof, including any amended or successor version of such
     directive); and
vii. other similar, equivalent or corresponding rights throughout the
     world based on applicable law or treaty, and any national
     implementations thereof.

2. Waiver. To the greatest extent permitted by, but not in contravention
of, applicable law, Affirmer hereby overtly, fully, permanently,
irrevocably and unconditionally waives, abandons, and surrenders all of
Affirmer's Copyright and Related Rights and associated claims and causes
of action, whether now known or unknown (including existing as well as
future claims and causes of action), in the Work (i) in all territories
worldwide, (ii) for the maximum duration provided by applicable law or
treaty (including future time extensions), (iii) in any current or future
medium and for any number of copies, and (iv) for any purpose whatsoever,
including without limitation commercial, advertising or promotional
purposes (the "Waiver"). Affirmer makes the Waiver for the benefit of each
member of the public at large and to the detriment of Affirmer's heirs and
successors, fully intending that such Waiver shall not be subject to
revocation, rescission, cancellation, termination, or any other legal or
equitable action to disrupt the quiet enjoyment of the Work by the public
as contemplated by Affirmer's express Statement of Purpose.

3. Public License Fallback. Should any part of the Waiver for any reason
be judged legally invalid or ineffective under applicable law, then the
Waiver shall be preserved to the maximum extent permitted taking into
account Affirmer's express Statement of Purpose. In addition, to the
extent the Waiver is so judged Affirmer hereby grants to each affected
person a royalty-free, non transferable, non sublicensable, non exclusive,
irrevocable and unconditional license to exercise Affirmer's Copyright and
Related Rights in the Work (i) in all territories worldwide, (ii) for the
maximum duration provided by applicable law or treaty (including future
time extensions), (iii) in any current or future medium and for any number
of copies, and (iv) for any purpose whatsoever, including without
limitation commercial, advertising or promotional purposes (the
"License"). The License shall be deemed effective as of the date CC0 was
applied by Affirmer to the Work. Should any part of the License for any
reason be judged legally invalid or ineffective under applicable law, such
partial invalidity or ineffectiveness shall not invalidate the remainder
of the License, and in such case Affirmer hereby affirms that he or she
will not (i) exercise any of his or her remaining Copyright and Related
Rights in the Work or (ii) assert any associated claims and causes of
action with respect to the Work, in either case contrary to Affirmer's
express Statement of Purpose.

4. Limitations and Disclaimers.

 a. No trademark or patent rights held by Affirmer are waived, abandoned,
    surrendered, licensed or otherwise affected by this document.
 b. Affirmer offers the Work as-is and makes no representations or
    warranties of any kind concerning the Work, express, implied,
    statutory or otherwise, including without limitation warranties of
    title, merchantability, fitness for a particular purpose, non
    infringement, or the absence of latent or other defects, accuracy, or
    the present or absence of errors, whether or not discoverable, all to
    the greatest extent permissible under applicable law.
 c. Affirmer disclaims responsibility for clearing rights of other persons
    that may apply to the Work or any use thereof, including without
    limitation any person's Copyright and Related Rights in the Work.
    Further, Affirmer disclaims responsibility for obtaining any necessary
    consents, permissions or other rights required for any use of the
    Work.
 d. Affirmer understands and acknowledges that Creative Commons is not a
    party to this document and has no duty or obligation with respect to
    this CC0 or use of the Work.


