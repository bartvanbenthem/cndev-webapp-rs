#SKIP_DOCKER=true ./scripts/init_db.sh

#cargo check
cargo fmt
cargo test -- --nocapture
#cargo tarpaulin --ignore-tests
#cargo clippy
#cargo audit

#cargo build
RUST_LOG=trace cargo run