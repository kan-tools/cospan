#!/usr/bin/env sh
# Witness for telos/disposable — "cospan can be thrown away without losing
# anything durable; nothing it holds persists beyond an explicit human action."
#
# Operationalized as: cospan and its substrate leave no disposable or secret
# state git-tracked. `.kan/` must be gitignored in full (kan ADR-3); `.day/` is
# day's disposable render cache; comment sidecars are ephemeral by default and
# reach kan only on an explicit human action.
#
# Exit 0  = clean; the witness holds.
# Exit 1  = tracked disposable/secret state found; the telos is not met.
set -eu

junk=$(git ls-files -- .kan .day .cospan '*.cospan.jsonl' 2>/dev/null || true)

if [ -n "$junk" ]; then
  printf 'tracked disposable/secret state (should be gitignored):\n%s\n' "$junk" >&2
  exit 1
fi
exit 0
