bin := "main"

check:
    cargo check --all --all-targets --all-features
clippy:
    cargo clippy --all --all-targets --all-features

run bin *pkg:
    cargo run --bin {{bin}}
