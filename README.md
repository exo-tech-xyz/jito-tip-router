# Jito MEV Tip Distribution NCN

## Testing Setup

### Prerequisites
1. Set up test-ledger: `./tip-router-operator-cli/scripts/setup-test-ledger.sh`
    * NOTE: This script fails on the edge version of Solana. Currently it's being ran 
    with `1.18.26`.

2. Build the tip router program: `cargo build-sbf -- -p jito-tip-router-program`
    * NOTE: Given the current state of Cargo.lock, you must use a version of cargo-build-sbf that 
    has a rust toolchain higher than 1.74.0. For now, switch to the edge version to build this.

3. Copy the program to fixtures: `cp target/deploy/jito_tip_router_program.so integration_tests/tests/fixtures`

3. Run tests: `SBF_OUT_DIR=integration_tests/fixtures cargo test`