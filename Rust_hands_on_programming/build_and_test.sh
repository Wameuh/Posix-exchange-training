#!/usr/bin/sh
set -x
rm -r -f output
mkdir output

cargo clean --manifest-path ipc_copy_file/Cargo.toml
cargo clean --manifest-path ipc_receivefile/Cargo.toml
cargo clean --manifest-path ipc_sendfile/Cargo.toml

cargo test --lib --manifest-path ipc_copy_file/Cargo.toml  --message-format human --color always > output/test.log
cargo llvm-cov --html --ignore-filename-regex test --manifest-path  ipc_copy_file/Cargo.toml
cargo build --manifest-path ipc_receivefile/Cargo.toml
cargo build --manifest-path ipc_sendfile/Cargo.toml






cp ipc_receivefile/target/debug/ipc_receivefile output
cp ipc_sendfile/target/debug/ipc_sendfile output
cp -r ipc_copy_file/target/llvm-cov/html/* output
cargo clean --manifest-path ipc_copy_file/Cargo.toml
cargo clean --manifest-path ipc_receivefile/Cargo.toml
cargo clean --manifest-path ipc_sendfile/Cargo.toml
set +x