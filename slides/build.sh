#!/bin/sh

set -eu

rm -rf dist/

# loop over all md files in this directory
for file in *.md; do
  # unit without extension
  base_filename=$(basename "$file" .md)

  # extract the module and unit numbers from the base filename
  unit_module=$(echo "$base_filename" | grep -oE '[0-9]+_[0-9]+' || true)

  # build the slidev presentation
  npx slidev build --out "dist/$unit_module" --base "/slides/$unit_module/" "$file" > /dev/null
done
