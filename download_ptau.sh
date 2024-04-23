#!/bin/bash

if ! [ -f "./tests/powersOfTau28_hez_final_20.ptau" ]; then
    # Verify https://github.com/iden3/snarkjs?tab=readme-ov-file#7-prepare-phase-2
    wget https://storage.googleapis.com/zkevm/ptau/powersOfTau28_hez_final_20.ptau -P ./tests
fi

# Hash listed at https://github.com/iden3/snarkjs?tab=readme-ov-file#7-prepare-phase-2
EXPECTED_HASH="89a66eb5590a1c94e3f1ee0e72acf49b1669e050bb5f93c73b066b564dca4e0c7556a52b323178269d64af325d8fdddb33da3a27c34409b821de82aa2bf1a27b"

echo "$EXPECTED_HASH ./tests/powersOfTau28_hez_final_20.ptau" | b2sum --check
