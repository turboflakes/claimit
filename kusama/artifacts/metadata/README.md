## Supported Runtimes
  - Kusama

## Generated files from subxt-cli

Download metadata from a substrate node, for use with `subxt` codegen.

```bash
subxt metadata --url wss://rpc.turboflakes.io:443/kusama -f bytes > kusama/artifacts/metadata/kusama_metadata.scale
subxt metadata --url wss://rpc.turboflakes.io:443/kusama --pallets System,Utility,Bounties,ChildBounties -f bytes > kusama/artifacts/metadata/kusama_metadata_small.scale