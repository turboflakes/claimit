## Supported Runtimes
  - Polkadot

## Generated files from subxt-cli

Download metadata from a substrate node, for use with `subxt` codegen.

```bash
subxt metadata --url wss://rpc.turboflakes.io:443/polkadot -f bytes > polkadot/artifacts/metadata/polkadot_metadata.scale
subxt metadata --url wss://rpc.turboflakes.io:443/polkadot --pallets System,Utility,Bounties,ChildBounties -f bytes > polkadot/artifacts/metadata/polkadot_metadata_small.scale