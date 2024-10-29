## Supported Runtimes
  - Paseo

## Generated files from subxt-cli

Download metadata from a substrate node, for use with `subxt` codegen.

```bash
subxt metadata --url wss://paseo.turboflakes.io -f bytes > paseo/artifacts/metadata/paseo_metadata.scale
subxt metadata --url wss://paseo.turboflakes.io --pallets System,Utility,Bounties,ChildBounties -f bytes > paseo/artifacts/metadata/paseo_metadata_small.scale
```