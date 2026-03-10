#!/bin/bash

START="2025-01-01"
END="2028-01-01"

for customer_id in {1..4}; do
  current="$START"

  while [[ "$current" < "$END" ]] || [[ "$current" == "$END" ]]; do
    AMOUNT=$((RANDOM % 10 + 1))
    DATE="${current}T07:23:26"

    curl -s -X POST "http://localhost:9090/retrieve-data/create-with-date" \
      -H "accept: text/plain" \
      -H "dev: a" \
      -H "Content-Type: application/json" \
      -d "{
      \"amount\": $AMOUNT,
      \"customer_id\": $customer_id,
      \"date\": \"$DATE\"
    }"

    echo "Inserted for $current (customer $customer_id)"

    current=$(date -I -d "$current + 1 day")
  done
done
