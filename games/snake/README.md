rustup target add wasm32-unknown-emscripten

git clone https://github.com/emscripten-core/emsdk.git && cd emsdk
./emsdk install latest
./emsdk activate latest
source ./emsdk_env.sh

export EMCC_CFLAGS="-s USE_SDL=2"

cargo build --target wasm32-unknown-emscripten --release

emrun --no_browser --port 8000 .
