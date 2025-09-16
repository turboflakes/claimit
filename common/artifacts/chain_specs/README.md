## Generated files from subxt-cli

Generate relay chain specs from subxt cli to be used for lightclient

```bash
cargo run --features chain-spec-pruning --bin subxt chain-spec --url wss://rpc.turboflakes.io:443/kusama --output-file common/artifacts/chain_specs/kusama.json --state-root-hash --remove-substitutes
cargo run --features chain-spec-pruning --bin subxt chain-spec --url wss://rpc.turboflakes.io:443/polkadot --output-file common/artifacts/chain_specs/polkadot.json --state-root-hash --remove-substitutes
cargo run --features chain-spec-pruning --bin subxt chain-spec --url wss://rpc.turboflakes.io:443/paseo --output-file common/artifacts/chain_specs/paseo.json --state-root-hash --remove-substitutes
```
