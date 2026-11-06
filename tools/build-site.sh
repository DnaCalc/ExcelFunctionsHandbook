#!/bin/sh
# Regenerates data/calcfunc from the pinned DnaCalc commit, then builds the site.
# Usage: DNACALC=path/to/clean/checkout/at/pin tools/build-site.sh
set -eu
root=$(cd "$(dirname "$0")/.." && pwd)
python3 "$root/tools/efh-calcfunc/regenerate.py" --dnacalc "${DNACALC:?set DNACALC to a checkout of the pinned commit}"
cargo run --release --locked --manifest-path "$root/tools/efh-site/Cargo.toml" -- build
