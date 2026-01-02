#!/bin/bash

# this should be in ~/ams-root

set -euo pipefail

cd ~/ams-root/

DATE=$(date)

echo "backup ams-prod.sqlite database ${DATE}"
git add -A

git commit -m "${DATE} backup ams-prod.sqlite database"
git push origin main
