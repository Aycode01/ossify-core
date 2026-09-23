#!/bin/bash
set -e

echo "Building contracts..."
/home/gamp/.cargo/bin/stellar contract build

echo "Setting up deployer account..."
/home/gamp/.cargo/bin/stellar keys generate --network testnet deployer || true
/home/gamp/.cargo/bin/stellar keys fund --network testnet deployer || true

echo "Deploying Registry..."
REGISTRY_ID=$(/home/gamp/.cargo/bin/stellar contract deploy --wasm target/wasm32v1-none/release/ossify_registry.wasm --network testnet --source deployer)
echo "Registry ID: $REGISTRY_ID"

echo "Deploying Reference RWA..."
RWA_ID=$(/home/gamp/.cargo/bin/stellar contract deploy --wasm target/wasm32v1-none/release/ossify_reference_rwa.wasm --network testnet --source deployer)
echo "Reference RWA ID: $RWA_ID"

echo "Deploying Toy Lending Pool..."
POOL_ID=$(/home/gamp/.cargo/bin/stellar contract deploy --wasm target/wasm32v1-none/release/ossify_toy_lending_pool.wasm --network testnet --source deployer)
echo "Toy Lending Pool ID: $POOL_ID"

echo "Wiring Reference RWA to Registry..."
/home/gamp/.cargo/bin/stellar contract invoke --id $RWA_ID --network testnet --source deployer -- register_self --registry $REGISTRY_ID

echo "=== DEPLOYMENT SUMMARY ==="
echo "Registry: $REGISTRY_ID"
echo "Reference RWA: $RWA_ID"
echo "Toy Lending Pool: $POOL_ID"
