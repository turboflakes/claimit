## Supported Runtimes
  - Polkadot AssetHub

## Generated files from subxt-cli

Download metadata from a substrate node, for use with `subxt` codegen.

```bash
subxt metadata --url wss://sys.turboflakes.io:443/asset-hub-polkadot -f bytes > polkadot_asset_hub/artifacts/metadata/polkadot_asset_hub_metadata.scale
subxt metadata --url wss://sys.turboflakes.io:443/asset-hub-polkadot --pallets System,Utility,Bounties,ChildBounties -f bytes > polkadot_asset_hub/artifacts/metadata/polkadot_asset_hub_metadata_small.scale
```
