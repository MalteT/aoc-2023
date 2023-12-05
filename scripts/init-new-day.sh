#!/usr/bin/env bash

# Check if the correct number of arguments is provided.
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <project_name>"
    exit 1
fi

project_name=$1

# Change to root directory
pushd "$(git rev-parse --show-toplevel)"
# Init new cargo project
cargo init --bin "$project_name"
cargo add --package "$project_name" --path aoc-utils
cp template/src/main.rs "$project_name/src/main.rs"
