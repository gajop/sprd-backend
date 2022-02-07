#!/bin/bash

echo "Starting updater script..."
sleep 5

echo "Updating rapid(registry)..."
./bin/sprd --root-folder /rapid download-registry
echo "Updating rapid(repo)..."
./bin/sprd --root-folder /rapid download-repo

echo "Updating database..."
./target/debug/update --root-folder /rapid
echo "Done"

sleep 55