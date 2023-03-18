curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64
chmod +x tailwindcss-linux-x64
mv tailwindcss-linux-x64 tailwindcss

curl https://sh.rustup.rs -sSf | sh -s -- -y
PATH=$PATH:/vercel/.cargo/bin

rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
cargo install --locked trunk

