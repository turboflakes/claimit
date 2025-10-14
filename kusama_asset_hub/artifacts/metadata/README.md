## Supported Runtimes
  - Kusama

## Generated files from subxt-cli

Download metadata from a substrate node, for use with `subxt` codegen.

```bash
subxt metadata --url wss://sys.turboflakes.io:443/asset-hub-kusama -f bytes > kusama_asset_hub/artifacts/metadata/kusama_asset_hub_metadata.scale
subxt metadata --url wss://sys.turboflakes.io:443/asset-hub-kusama --pallets System,Utility,Bounties,ChildBounties -f bytes > kusama_asset_hub/artifacts/metadata/kusama_asset_hub_metadata_small.scale
```
