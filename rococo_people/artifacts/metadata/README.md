## Supported Runtimes
  - Rococo People

## Generated files from subxt-cli

Download metadata from a substrate node, for use with `subxt` codegen.

```bash
subxt metadata --url wss://rococo-people-rpc.polkadot.io -f bytes > rococo_people/artifacts/metadata/rococo_people_metadata.scale
subxt metadata --url wss://rococo-people-rpc.polkadot.io --pallets Identity -f bytes > rococo_people/artifacts/metadata/rococo_people_metadata_small.scale