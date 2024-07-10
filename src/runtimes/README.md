## Supported Runtimes
  - Polkadot
  - Kusama
  - Asset Hub Polkadot
  - Asset Hub Kusama

## Generated files from subxt-cli

Download metadata from a substrate node, for use with `subxt` codegen.

```bash
subxt metadata --url wss://rpc.turboflakes.io:443/polkadot -f bytes > artifacts/metadata/polkadot_metadata.scale
subxt metadata --url wss://rpc.turboflakes.io:443/polkadot --pallets System,Utility,Bounties,ChildBounties -f bytes > artifacts/metadata/polkadot_metadata_small.scale
subxt metadata --url wss://rpc.turboflakes.io:443/kusama -f bytes > artifacts/metadata/kusama_metadata.scale
subxt metadata --url wss://rpc.turboflakes.io:443/kusama --pallets System,Utility,Bounties,ChildBounties -f bytes > artifacts/metadata/kusama_metadata_small.scale
```

Generate runtime API client code from metadata.

```bash
subxt codegen --url wss://rpc.turboflakes.io:443/polkadot | rustfmt --edition=2018 --emit=stdout > polkadot_runtime.rs
subxt codegen --url wss://rpc.turboflakes.io:443/kusama | rustfmt --edition=2018 --emit=stdout > kusama_runtime.rs
```

Generate chain-specs
```
cargo run --features chain-spec-pruning --bin subxt chain-spec --url wss://rpc.turboflakes.io:443/kusama --output-file artifacts/demo_chain_specs/kusama.json --state-root-hash --remove-substitutes
```