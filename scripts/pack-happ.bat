REM Compile the WASM
cargo build --release --target wasm32-unknown-unknown
REM Pack DNAs
hc dna pack --output=testground.dna dna.workdir
REM Pack the Happ with everything
hc app pack --output=testground.happ dna.workdir