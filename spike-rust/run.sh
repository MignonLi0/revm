#!/bin/bash

set -e

#BIN_DIR="src/bin"
#TIME_CMD="/usr/bin/time -l"
#
#for file in "$BIN_DIR"/*.rs; do
#    BIN_NAME=$(basename "$file" .rs)
#    
#    echo "=== Running binary: $BIN_NAME ==="
#    $TIME_CMD cargo run --release --bin "$BIN_NAME"
#    echo ""
#done

echo precompile_kzg_point; /usr/bin/time -l ../../target/release/precompile_kzg_point
echo precompile_blake2   ; /usr/bin/time -l ../../target/release/precompile_blake2
echo sigverify           ; /usr/bin/time -l ../../target/release/sigverify
echo ecrecover           ; /usr/bin/time -l ../../target/release/ecrecover
echo keccak              ; /usr/bin/time -l ../../target/release/keccak
echo sha256              ; /usr/bin/time -l ../../target/release/sha256
echo div                 ; /usr/bin/time -l ../../target/release/div
echo mul                 ; /usr/bin/time -l ../../target/release/mul
echo precompile_sha256   ; /usr/bin/time -l ../../target/release/precompile_sha256
echo precompile_ecrecover; /usr/bin/time -l ../../target/release/precompile_ecrecover
echo precompile_ripemd160; /usr/bin/time -l ../../target/release/precompile_ripemd160
echo precompile_ec_mul   ; /usr/bin/time -l ../../target/release/precompile_ec_mul
echo precompile_modexp   ; /usr/bin/time -l ../../target/release/precompile_modexp
echo precompile_ec_add   ; /usr/bin/time -l ../../target/release/precompile_ec_add
echo precompile_ec_pair  ; /usr/bin/time -l ../../target/release/precompile_ec_pair
