#!/usr/bin/env bash

# Check if the correct number of arguments is provided.
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <day>"
    exit 1
fi

day="$1"

hyperfine \
  --shell=none \
  --warmup 3 \
  --setup "nix build .#day-$day" \
  --command-name "day-$day first" \
  "./result/bin/day-$day inputs/inputs-$day first" \
  --command-name "day-$day second" \
  "./result/bin/day-$day inputs/inputs-$day second" \
  2>/dev/null

