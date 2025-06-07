#!/bin/bash

set -e
BIN_DIR="src/bin"

rm -f output.log

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

    echo "=== $BIN_NAME ===" | tee -a output.log
    cargo build --release --bin "$BIN_NAME"
    /home/orangepi/linux-6.6.63/tools/perf/perf stat ../../target/release/"$BIN_NAME" >> output.log 2>&1

    echo ""
done
