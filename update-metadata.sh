#!/bin/bash
#
# > make a file executable
# chmod +x ./update-metadata.sh
#
# > subxt-cli must be installed to update metadata
# cargo install subxt-cli --force
#
# Relay Chains
# subxt metadata --url wss://rpc.turboflakes.io:443/paseo -f bytes > paseo/artifacts/metadata/paseo_metadata.scale
# subxt metadata --url wss://rpc.turboflakes.io:443/paseo --pallets System -f bytes > paseo/artifacts/metadata/paseo_metadata_small.scale
# subxt metadata --url wss://rpc.turboflakes.io:443/kusama -f bytes > kusama/artifacts/metadata/kusama_metadata.scale
# subxt metadata --url wss://rpc.turboflakes.io:443/kusama --pallets System,Utility,Bounties,ChildBounties -f bytes > kusama/artifacts/metadata/kusama_metadata_small.scale
# subxt metadata --url wss://rpc.turboflakes.io:443/polkadot -f bytes > polkadot/artifacts/metadata/polkadot_metadata.scale
# subxt metadata --url wss://rpc.turboflakes.io:443/polkadot --pallets System,Utility,Bounties,ChildBounties -f bytes > polkadot/artifacts/metadata/polkadot_metadata_small.scale
#
# People Chains
# subxt metadata --url wss://sys.turboflakes.io:443/people-paseo -f bytes > paseo_people/artifacts/metadata/paseo_people_metadata.scale
subxt metadata --url wss://sys.turboflakes.io:443/people-paseo --pallets Identity -f bytes > paseo_people/artifacts/metadata/paseo_people_metadata_small.scale
# subxt metadata --url wss://sys.turboflakes.io:443/people-kusama -f bytes > kusama_people/artifacts/metadata/kusama_people_metadata.scale
subxt metadata --url wss://sys.turboflakes.io:443/people-kusama --pallets Identity -f bytes > kusama_people/artifacts/metadata/kusama_people_metadata_small.scale
# subxt metadata --url wss://sys.turboflakes.io:443/people-polkadot -f bytes > polkadot_people/artifacts/metadata/polkadot_people_metadata.scale
subxt metadata --url wss://sys.turboflakes.io:443/people-polkadot --pallets Identity -f bytes > polkadot_people/artifacts/metadata/polkadot_people_metadata_small.scale

#
# AssetHub Chains
# subxt metadata --url wss://sys.turboflakes.io:443/asset-hub-paseo -f bytes > paseo_asset_hub/artifacts/metadata/paseo_asset_hub_metadata.scale
subxt metadata --url wss://sys.turboflakes.io:443/asset-hub-paseo --pallets System,Utility,Bounties,ChildBounties -f bytes > paseo_asset_hub/artifacts/metadata/paseo_asset_hub_metadata_small.scale
# subxt metadata --url wss://sys.turboflakes.io:443/asset-hub-kusama -f bytes > kusama_asset_hub/artifacts/metadata/kusama_asset_hub_metadata.scale
subxt metadata --url wss://sys.turboflakes.io:443/asset-hub-kusama --pallets System,Utility,Bounties,ChildBounties -f bytes > kusama_asset_hub/artifacts/metadata/kusama_asset_hub_metadata_small.scale
# subxt metadata --url wss://sys.turboflakes.io:443/asset-hub-polkadot -f bytes > polkadot_asset_hub/artifacts/metadata/polkadot_asset_hub_metadata.scale
subxt metadata --url wss://sys.turboflakes.io:443/asset-hub-polkadot --pallets System,Utility,Bounties,ChildBounties -f bytes > polkadot_asset_hub/artifacts/metadata/polkadot_asset_hub_metadata_small.scale
