#!/bin/bash

set -e
BIN_DIR="src/bin"

TIME_CMD="/usr/bin/time -l"

rm -f output_mac.log


for file in "$BIN_DIR"/*.rs; do
    FNAME=$(basename "$file")

    if [[ "$FNAME" == "mod.rs" ]]; then
        echo "Skipping mod.rs"
        continue
    fi

    if [[ ! -s "$file" ]]; then
        echo "Skipping empty file: $file"
        continue
    fi
    
    BIN_NAME=$(basename "$file" .rs)

    echo "=== $BIN_NAME ===" | tee -a output_mac.log
    cargo build --release --bin "$BIN_NAME"
    $TIME_CMD ../../target/release/"$BIN_NAME" >> output_mac.log 2>&1

    echo ""
done
