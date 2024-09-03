## Supported Runtimes
  - Kusama People

## Generated files from subxt-cli

Download metadata from a substrate node, for use with `subxt` codegen.

```bash
subxt metadata --url wss://sys.turboflakes.io:443/people-kusama -f bytes > kusama_people/artifacts/metadata/kusama_people_metadata.scale
subxt metadata --url wss://sys.turboflakes.io:443/people-kusama --pallets Identity -f bytes > kusama_people/artifacts/metadata/kusama_people_metadata_small.scale