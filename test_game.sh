near view streaming-roketo.vengone.testnet get_stream \
    "{\"stream_id\": \"$1\"}" \
    --accountId vengone.testnet

near view streaming-roketo.vengone.testnet get_stream \
    "{\"stream_id\": \"$2\"}" \
    --accountId vengone.testnet

near call tic-tac-near.vengone.testnet start\
    --accountId vengone.testnet \
    --gas 300000000000000

near call tic-tac-near.vengone.testnet make_turn \
    '{"x": 1, "y": 1}' \
    --accountId vengone.testnet \
    --gas 300000000000000

# near call tic-tac-near.vengone.testnet get_field --accountId vengone1.testnet

near call tic-tac-near.vengone.testnet make_turn \
    '{"x": 0, "y": 0}' \
    --accountId vengone1.testnet \
    --gas 300000000000000

# near call tic-tac-near.vengone.testnet get_field --accountId vengone1.testnet

near call tic-tac-near.vengone.testnet make_turn \
    '{"x": 1, "y": 0}' \
    --accountId vengone.testnet \
    --gas 300000000000000

# near call tic-tac-near.vengone.testnet get_field --accountId vengone1.testnet

near call tic-tac-near.vengone.testnet make_turn \
    '{"x": 0, "y": 1}' \
    --accountId vengone1.testnet \
    --gas 300000000000000

# near call tic-tac-near.vengone.testnet get_field --accountId vengone1.testnet

echo "\n\n\n Get streams before winning turn:\n\n\n"

near view streaming-roketo.vengone.testnet get_stream \
    "{\"stream_id\": \"$1\"}" \
    --accountId vengone.testnet

near view streaming-roketo.vengone.testnet get_stream \
    "{\"stream_id\": \"$2\"}" \
    --accountId vengone.testnet


near call tic-tac-near.vengone.testnet make_turn \
    '{"x": 1, "y": 2}' \
    --accountId vengone.testnet \
    --gas 300000000000000

near call tic-tac-near.vengone.testnet get_field --accountId vengone1.testnet


near view streaming-roketo.vengone.testnet get_stream \
    "{\"stream_id\": \"$1\"}" \
    --accountId vengone.testnet

near view streaming-roketo.vengone.testnet get_stream \
    "{\"stream_id\": \"$2\"}" \
    --accountId vengone.testnet
