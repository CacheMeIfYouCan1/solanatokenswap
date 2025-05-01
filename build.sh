echo "Compiling..."
echo " "

echo "Compiling send_sol_handler"
cd handling/send_sol_handler && rm Cargo.lock && cargo build || { echo "Failed to build send_sol_handler"; exit 1; }
echo " "
echo "Compiling send_token_handler"
cd ../send_token_handler && rm Cargo.lock && cargo build || { echo "Failed to build send_token_handler"; exit 1; }


echo "Compiling send_sol smart contract"
cd ../../smart_contracts/send_sol && rm Cargo.lock && cargo build-sbf || { echo "Failed to build send_sol"; exit 1; }

echo "Compiling send_token smart contract"
cd ../send_token && rm Cargo.lock && cargo build-sbf || { echo "Failed to build send_token"; exit 1; }

echo "All projects have been compiled successfully!"
