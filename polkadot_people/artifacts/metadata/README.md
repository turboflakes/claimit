## Supported Runtimes
  - Polkadot People

## Generated files from subxt-cli

Download metadata from a substrate node, for use with `subxt` codegen.

```bash
subxt metadata --url wss://sys.turboflakes.io:443/people-polkadot -f bytes > polkadot_people/artifacts/metadata/polkadot_people_metadata.scale
subxt metadata --url wss://sys.turboflakes.io:443/people-polkadot --pallets Identity -f bytes > polkadot_people/artifacts/metadata/polkadot_people_metadata_small.scale