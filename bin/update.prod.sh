#!/bin/bash

export SLEEP_TIME=60

echo "Starting updater script..."

echo "Updating rapid(registry)..."
/app/sprd --root-folder /rapid download-registry
echo "Updating rapid(repo)..."
/app/sprd --root-folder /rapid download-repo

echo "Updating database..."
/app/update --root-folder /rapid
echo "Done. Sleeping for ${SLEEP_TIME} seconds..."

sleep $SLEEP_TIME