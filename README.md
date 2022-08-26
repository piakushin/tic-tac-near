Tic tac toe
===========

A [smart contract] written in [Rust].
It is a game even for a creatures with a single neuron. In which you should put 3 X or O in a row before your opponent.
Why would I implement such simple game? Because I'm also a single cell organism.


Quick Start
===========

1. Before you compile this code, you will need to install Rust with [correct target]

2. Before initializing the game you must deploy streaming smart contract. 
It's account Id will be passed to the game contract on start.

### Manual mode
4. Deploy the game contract.
```sh
near deploy \
    --accountId tic-tac-near.YOURNAME.testnet \
    --wasmFile ./out/tic_tac_near.wasm
```

4. Then you must connect streaming contract.
```sh
near call tic-tac-near.YOURNAME.testnet connect_streaming_contract \
    '{"streaming_id": "streaming-roketo.YOURNAME.testnet"}' \
    --accountId tic-tac-near.YOURNAME.testnet \
    --gas 300000000000000
```

4. Then first player must deposit any amount of any token to the game contract account.
Message should contain JSON map with key `tokens_per_sec` and a value as a string.
Example with wNEAR FT:
```sh
near call wrap.testnet ft_transfer_call \
    '{"receiver_id": "tic-tac-near.YOURNAME.testnet", "amount": "300000000000000000000000", "msg": "{\"tokens_per_sec\": \"1000000\"}"}' \
    --depositYocto 1 \
    --gas 300000000000000 \
    --accountId FIRST_PLAYER.testnet
```

5. And the second player must do exactly the same: with the same token, with the same amount.
Second player message unimportant, it won't be used anywhere.
Example with wNEAR FT:
```sh
near call wrap.testnet ft_transfer_call \
    '{"receiver_id": "tic-tac-near.YOURNAME.testnet", "amount": "300000000000000000000000", "msg": ""}' \
    --depositYocto 1 \
    --gas 300000000000000 \
    --accountId SECOND_PLAYER.testnet
```

6. Now you can start the game.
It will start stream of tokens back to the second player's account.
The faster the first player will make it's turn, the less tokens will the second recieve and vice versa.
```sh
near call tic-tac-near.YOURNAME.testnet start\
    --accountId YOURNAME.testnet \
    --gas 300000000000000
```

7. One by on make turns with `make_turn` method (who would believe).
```sh
near call tic-tac-near.YOURNAME.testnet make_turn \
    '{"x": 1, "y": 1}' \
    --accountId FIRST_PLAYER.testnet \
    --gas 300000000000000
```

8. Once any player have reached winning combination both streams will be stopped.

9. To claim reward, call `claim_reward` method signed by the winner.
All remaining tokens on stream contract will be transferred to the winner as a reward for it's miserable life
```sh
near call tic-tac-near.YOURNAME.testnet claim_reward \
    --accountId FIRST_PLAYER.testnet \
    --gas 300000000000000
```

### Auto mode
3. Call:
```sh
./deploy_and_test.sh
```
4. Copy Ids of streams and pass them as arguments to the next script.

5. Run:
```sh
./test_game.sh $FIRST_STREAM $SECOND_STREAM
```

6. Celebrate, you don't need to play this very complicated game.
Blockchain will do everything for you!


Interacting With The Contract
=============================
## `connect_streaming_contract(streaming_id: AccountId)`
Connects game contract with streaming contract.
```sh
near call tic-tac-near.YOURNAME.testnet connect_streaming_contract \
    '{"streaming_id": "streaming-roketo.YOURNAME.testnet"}' \
    --accountId YOURNAME.testnet \
    --gas 300000000000000
```

## `start()`
Starts the game. Now first player should be quick to make first turn.
```sh
$ near call tic-tac-near.YOURNAME.testnet start\
    --accountId YOURNAME.testnet \
    --gas 300000000000000
```

## `make_turn(x: u8, y: u8)`
Sets X or O, depends on signing account to the field. X coordinate - from left to right, Y - from up to down.
You think why it's a whole u8 for a 3 bit state? Because, live with it.
Field coordinates:
| Y/X | 0 | 1 | 2 |
|-----|---|---|---|
| 0   | _ | _ | X |
| 1   | O | X | X |
| 2   | O | O | X |
```sh
near call tic-tac-near.YOURNAME.testnet make_turn \
    '{"x": 1, "y": 1}' \
    --accountId YOURNAME.testnet \
    --gas 300000000000000
```
(you can choose your own coordinates for your moves, who would have thought).

## `claim_reward()`
Transfers remaining deposited money from both players to winner.
```sh
near call tic-tac-near.YOURNAME.testnet claim_reward \
    --accountId FIRST_PLAYER.testnet \
    --gas 300000000000000
```

## `reset()`
Completely resets game state. No refunds! (yet).

Getting contract states
=======================
## `status() -> Status`
Gets players information. If information is missing, some fields will be `null`.
```sh
near call tic-tac-near.YOURNAME.testnet status \
    --accountId YOURNAME.testnet
```
For example if both players are registered:
```sh
{
  first_player: {
    account: 'neuron0.testnet',
    deposit: '300000000000000000000000',
    stream: 'G6Z65ARQjYKsg9wATGLeqvwujWyr6yjzb61ALQ8CARgx'
  },
  second_player: {
    account: 'neuron1.testnet',
    deposit: '300000000000000000000000',
    stream: '642BygkDkPj6gXRDFU6kLRrmh6NLT6yRbffrCpzxEQEz'
  }
}
```

## `get_field() -> Field`
If you can't remember 9 3bit values, then it's your salvation.
```sh
call tic-tac-near.YOURNAME.testnet get_field
    --accountId YOURNAME.testnet
```
Example output:
```sh
{
  [
    [ 'O', 'X', 'Empty' ],
    [ 'O', 'X', 'Empty' ],
    [ 'Empty', 'X', 'Empty' ]
  ]
}
```


  [smart contract]: https://docs.near.org/develop/welcome
  [Rust]: https://www.rust-lang.org/
  [create-near-app]: https://github.com/near/create-near-app
  [correct target]: https://docs.near.org/develop/prerequisites#rust-and-wasm
  [cargo]: https://doc.rust-lang.org/book/ch01-03-hello-cargo.html
