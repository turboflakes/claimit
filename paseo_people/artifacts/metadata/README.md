## Supported Runtimes
  - Paseo People

## Generated files from subxt-cli

Download metadata from a substrate node, for use with `subxt` codegen.

```bash
subxt metadata --url wss://people-paseo.turboflakes.io -f bytes > paseo_people/artifacts/metadata/paseo_people_metadata.scale
subxt metadata --url wss://people-paseo.turboflakes.io --pallets Identity -f bytes > paseo_people/artifacts/metadata/paseo_people_metadata_small.scale
```