## Supported Runtimes
  - Kusama

## Generated files from subxt-cli

Download metadata from a substrate node, for use with `subxt` codegen.

```bash
subxt metadata --url wss://rococo-rpc.polkadot.io -f bytes > rococo/artifacts/metadata/rococo_metadata.scale
subxt metadata --url wss://rococo-rpc.polkadot.io --pallets System,Utility,Bounties,ChildBounties -f bytes > rococo/artifacts/metadata/rococo_metadata_small.scale