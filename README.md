Tic tac toe
===========

A [smart contract] written in [Rust].
It is a game for a creatures with a single neuron. In which you should put 3 X or O in a row before your opponent.
Why would I implement such simple game? Because I'm also a single cell organism.


Quick Start
===========

1. Before you compile this code, you will need to install Rust with [correct target]

2. Before initializing the game you must deploy Roke.to smart contract. It's account Id will be passed to game contract on start.

3. Then you must deploy the game contract.

4. Then you must add streaming contract via `connect_streaming_contract`.

4. Then first player must deposit any amount of any token to the game contract account. And the second player must do exactly the same: with the same token, with the same amount.

5. Then you must call `start` method with account id of Roke.to contract. It will start stream of tokens back to the second player's account. The faster the first player will make it's turn, the less tokens will the second sucker recieve and vice versa.

6. One by on make turns with `make_turn` method (who would believe).

7. Once any player have reached winning combination, all remaining tokens on stream contract will be transferred to the winner as a reward for it's miserable life.


Interacting With The Contract
=============================
## `connect_streaming_contract(streaming_id: AccountId)`
Connects game contract with streaming contract.

## `start()`
Starts the game with given streaming account id. 
```sh
$ near call tic-tac-near.YOURNAME.testnet start\
    '{"streaming_id": "STREAMING_ID"}' \
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
Example youput:
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
