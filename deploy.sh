# Make sure all targets are downloaded using:
# rustup target list --installed
# Add targets by:
# rustup target add x86_64-unknown-linux-gnu
# rustup target add x86_64-pc-windows-gnu
cargo build -r --frozen --target x86_64-unknown-linux-gnu
cargo build -r --frozen --target x86_64-pc-windows-gnu
