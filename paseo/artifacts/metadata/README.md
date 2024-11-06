## Supported Runtimes
  - Kusama

## Generated files from subxt-cli

Download metadata from a substrate node, for use with `subxt` codegen.

```bash
subxt metadata --url wss://rpc.turboflakes.io:443/paseo -f bytes > paseo/artifacts/metadata/paseo_metadata.scale
subxt metadata --url wss://rpc.turboflakes.io:443/paseo --pallets System,Utility,Bounties,ChildBounties -f bytes > paseo/artifacts/metadata/paseo_metadata_small.scale
```