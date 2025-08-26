<!-- For Web -->

RUSTFLAGS='--cfg getrandom_backend="wasm_js"' cargo build --release --target wasm32-unknown-unknown

wasm-bindgen --no-typescript --target web \
 --out-dir ./flappybird/ \
 --out-name "flappybird" \
 ./target/wasm32-unknown-unknown/release/flappy-bevy.wasm
