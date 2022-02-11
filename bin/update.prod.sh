#!/bin/bash

export SLEEP_TIME=60

echo "Starting updater script..."

echo "Updating rapid(registry)..."
./sprd --root-folder /rapid download-registry
echo "Updating rapid(repo)..."
./sprd --root-folder /rapid download-repo

echo "Updating database..."
./update --root-folder /rapid
echo "Done. Sleeping for ${SLEEP_TIME} seconds..."

sleep $SLEEP_TIME