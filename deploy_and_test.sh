echo "<------------------------------------------------------------->"
echo "Build smart contract."
echo "<------------------------------------------------------------->"
RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release

echo "\n\n\n<------------------------------------------------------------->"
echo "Copy wasm file to out dir."
echo "<------------------------------------------------------------->"
cp ./target/wasm32-unknown-unknown/release/tic_tac_near.wasm ./out

echo "\n\n\n<------------------------------------------------------------->"
echo "Delete old contract account."
echo "<------------------------------------------------------------->"
near delete tic-tac-near.vengone.testnet vengone.testnet

echo "\n\n\n<------------------------------------------------------------->"
echo "Create new contract account."
echo "<------------------------------------------------------------->"
near create-account tic-tac-near.vengone.testnet --masterAccount vengone.testnet

echo "\n\n\n<------------------------------------------------------------->"
echo "Deploy it to the testnet."
echo "<------------------------------------------------------------->"
near deploy \
    --accountId tic-tac-near.vengone.testnet \
    --wasmFile ./out/tic_tac_near.wasm

# echo "\n\n\n<------------------------------------------------------------->"
# echo "Register game contract account."
# echo "<------------------------------------------------------------->"
# near call wrap.testnet storage_deposit \
#        '{"account_id": "tic-tac-near.vengone.testnet"}' \
#        --accountId vengone.testnet \
#        --depositYocto 12500000000000000000000



echo "\n\n\n<------------------------------------------------------------->"
echo "Deposit wNEAR to first player's account."
echo "<------------------------------------------------------------->"
near call wrap.testnet near_deposit '' \
    --accountId vengone.testnet \
    --deposit 0.3

# echo "\n\n\n<------------------------------------------------------------->"
# echo "Check game contract status before."
# echo "<------------------------------------------------------------->"
# near call tic-tac-near.vengone.testnet status \
#     --accountId vengone.testnet

echo "\n\n\n<------------------------------------------------------------->"
echo "Register first player."
echo "<------------------------------------------------------------->"
near call wrap.testnet ft_transfer_call \
    '{"receiver_id": "tic-tac-near.vengone.testnet", "amount": "300000000000000000000000", "msg": "test"}' \
    --depositYocto 1 \
    --gas 300000000000000 \
    --accountId vengone.testnet

# echo "\n\n\n<------------------------------------------------------------->"
# echo "Check game contract status after."
# echo "<------------------------------------------------------------->"
# near call tic-tac-near.vengone.testnet status \
#     --accountId vengone.testnet



echo "\n\n\n<------------------------------------------------------------->"
echo "Deposit wNEAR to second player's account."
echo "<------------------------------------------------------------->"
near call wrap.testnet near_deposit '' \
    --accountId vengone1.testnet \
    --deposit 0.3

# echo "\n\n\n<------------------------------------------------------------->"
# echo "Check game contract status before."
# echo "<------------------------------------------------------------->"
# near call tic-tac-near.vengone.testnet status \
#     --accountId vengone1.testnet

echo "\n\n\n<------------------------------------------------------------->"
echo "Register second player."
echo "<------------------------------------------------------------->"
near call wrap.testnet ft_transfer_call \
    '{"receiver_id": "tic-tac-near.vengone.testnet", "amount": "300000000000000000000000", "msg": "test"}' \
    --depositYocto 1 \
    --gas 300000000000000 \
    --accountId vengone1.testnet

echo "\n\n\n<------------------------------------------------------------->"
echo "Check wNEAR contract balance after."
echo "<------------------------------------------------------------->"
near call wrap.testnet ft_balance_of \
    '{"account_id": "tic-tac-near.vengone.testnet"}' \
    --accountId vengone1.testnet

echo "\n\n\n<------------------------------------------------------------->"
echo "Check game contract status after."
echo "<------------------------------------------------------------->"
near call tic-tac-near.vengone.testnet status \
    --accountId vengone1.testnet

# echo "\n\n\n<------------------------------------------------------------->"
# echo "Make first turn: first player"
# echo "<------------------------------------------------------------->"
# near call tic-tac-near.vengone.testnet make_turn \
#     '{"x: 1, "y": 1}' \
#     -- accountId vengone.testnet

# echo "\n\n\n<------------------------------------------------------------->"
# echo "Make first turn: second player"
# echo "<------------------------------------------------------------->"
# near call tic-tac-near.vengone.testnet make_turn \
#     '{"x: 0, "y": 0}' \
#     -- accountId vengone1.testnet

