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
subxt metadata --url wss://rococo-rpc.polkadot.io:443 -f bytes > artifacts/metadata/rococo_metadata.scale
subxt metadata --url wss://rococo-rpc.polkadot.io:443 --pallets System,Utility,Bounties,ChildBounties -f bytes > artifacts/metadata/rococo_metadata_small.scale