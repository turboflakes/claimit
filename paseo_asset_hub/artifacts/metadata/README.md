## Supported Runtimes
  - Kusama

## Generated files from subxt-cli

Download metadata from a substrate node, for use with `subxt` codegen.

```bash
subxt metadata --url wss://sys.turboflakes.io:443/asset-hub-paseo -f bytes > paseo_asset_hub/artifacts/metadata/paseo_asset_hub_metadata.scale
subxt metadata --url wss://sys.turboflakes.io:443/asset-hub-paseo --pallets System,Utility,Bounties,ChildBounties -f bytes > paseo_asset_hub/artifacts/metadata/paseo_asset_hub_metadata_small.scale
```
