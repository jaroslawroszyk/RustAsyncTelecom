#!/bin/bash

CLIENT_PATH="../target/debug/client"

INSTANCES=$1

if [ -z "$INSTANCES" ]; then
  echo "Please provide the number of instances to run."
  exit 1
fi

for ((i = 1; i <= INSTANCES; i++)); do
  echo "Running instance $i of the client..."
  $CLIENT_PATH &
done

echo "Started $INSTANCES instances of the client."
