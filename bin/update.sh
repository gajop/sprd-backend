#!/bin/bash

export SLEEP_TIME=300

echo "Starting updater script..."

echo "Updating rapid(registry)..."
./bin/sprd --root-folder /rapid download-registry
echo "Updating rapid(repo)..."
./bin/sprd --root-folder /rapid download-repo

echo "Updating database..."
./target/debug/update --root-folder /rapid
echo "Done. Sleeping for ${SLEEP_TIME} seconds..."

sleep $SLEEP_TIME