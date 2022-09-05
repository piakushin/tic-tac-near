RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release &&
cp target/wasm32-unknown-unknown/release/tic_tac_near.wasm out/