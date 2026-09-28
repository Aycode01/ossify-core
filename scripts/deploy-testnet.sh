#!/bin/bash
# Deploys the registry, reference RWA, and toy lending pool to the Stellar testnet.
#
# Requires the Stellar CLI (v25.2.0+) to be installed and available on PATH as
# `stellar`. See https://developers.stellar.org/docs/tools/stellar-cli
set -e

echo "Building contracts..."
stellar contract build

echo "Setting up deployer account..."
stellar keys generate --network testnet deployer || true
stellar keys fund --network testnet deployer || true

echo "Deploying Registry..."
REGISTRY_ID=$(stellar contract deploy --wasm target/wasm32v1-none/release/ossify_registry.wasm --network testnet --source deployer)
echo "Registry ID: $REGISTRY_ID"

echo "Deploying Reference RWA..."
RWA_ID=$(stellar contract deploy --wasm target/wasm32v1-none/release/ossify_reference_rwa.wasm --network testnet --source deployer)
echo "Reference RWA ID: $RWA_ID"

echo "Deploying Toy Lending Pool..."
POOL_ID=$(stellar contract deploy --wasm target/wasm32v1-none/release/ossify_toy_lending_pool.wasm --network testnet --source deployer)
echo "Toy Lending Pool ID: $POOL_ID"

echo "Wiring Reference RWA to Registry..."
stellar contract invoke --id $RWA_ID --network testnet --source deployer -- register_self --registry $REGISTRY_ID

echo "=== DEPLOYMENT SUMMARY ==="
echo "Registry: $REGISTRY_ID"
echo "Reference RWA: $RWA_ID"
echo "Toy Lending Pool: $POOL_ID"
