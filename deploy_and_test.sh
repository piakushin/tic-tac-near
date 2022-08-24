echo "Build smart contract."
RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release

echo "Copy wasm file to out dir."
cp ./target/wasm32-unknown-unknown/release/tic_tac_near.wasm ./out

echo "Deploy it to the testnet."
near deploy \
    --accountId tic-tac-near.vengone.testnet \
    --wasmFile ./out/tic_tac_near.wasm

echo "Register game contract account."
near call wrap.testnet storage_deposit \
       '{"account_id": "tic-tac-near.vengone.testnet"}' \
       --accountId vengone.testnet \
       --depositYocto 12500000000000000000000



echo "Deposit wNEAR to first player's account."
near call wrap.testnet near_deposit '' \
    --accountId vengone.testnet \
    --deposit 0.3

echo "Check wNEAR contract balance before."
near call wrap.testnet ft_balance_of \
    '{"account_id": "tic-tac-near.vengone.testnet"}' \
    --accountId vengone.testnet

echo "Check game contract status before."
near call tic-tac-near.vengone.testnet status \
    --accountId vengone.testnet

echo "Register first player."
near call wrap.testnet ft_transfer_call \
    '{"receiver_id": "tic-tac-near.vengone.testnet", "amount": "300000000000000000000000", "msg": "test"}' \
    --depositYocto 1 \
    --gas 200000000000000 \
    --accountId vengone.testnet

echo "Check wNEAR contract balance after."
near call wrap.testnet ft_balance_of \
    '{"account_id": "tic-tac-near.vengone.testnet"}' \
    --accountId vengone.testnet

echo "Check game contract status after."
near call tic-tac-near.vengone.testnet status \
    --accountId vengone.testnet



echo "Deposit wNEAR to second player's account."
near call wrap.testnet near_deposit '' \
    --accountId vengone1.testnet \
    --deposit 0.3

echo "Check wNEAR contract balance before."
near call wrap.testnet ft_balance_of \
    '{"account_id": "tic-tac-near.vengone.testnet"}' \
    --accountId vengone1.testnet

echo "Check game contract status before."
near call tic-tac-near.vengone.testnet status \
    --accountId vengone1.testnet

echo "Register second player."
near call wrap.testnet ft_transfer_call \
    '{"receiver_id": "tic-tac-near.vengone.testnet", "amount": "300000000000000000000000", "msg": "test"}' \
    --depositYocto 1 \
    --gas 200000000000000 \
    --accountId vengone1.testnet

echo "Check wNEAR contract balance after."
near call wrap.testnet ft_balance_of \
    '{"account_id": "tic-tac-near.vengone.testnet"}' \
    --accountId vengone1.testnet

echo "Check game contract status after."
near call tic-tac-near.vengone.testnet status \
    --accountId vengone1.testnet
