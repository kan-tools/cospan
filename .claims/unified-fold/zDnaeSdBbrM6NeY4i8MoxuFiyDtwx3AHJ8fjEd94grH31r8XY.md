---
{
  "v": 3,
  "cid": "bafyreieczbjpjyyjrr7sup42lbdyqna2oultt4adxrdp6gtmizxzflogje",
  "sig": "8b7d5d84030f93e3a8ceb21159888eb9d4983a2be2885628b4e39a64d71b9ab2460fbb4062f73374c1077043c88b9b4dabb495cc345068507b9cb49d964a7822",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "unified-fold"
  },
  "kind": "observation",
  "cites": [],
  "rev": "223mth2gm525f",
  "seq": 0,
  "of": 10,
  "text_len": 186,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2FsbHVuaWZpZWQtZm9sZGlhcnRpZmFjdHOBoWZDb21taXR4KGRmZjc4MDQyZGFlZjlmMjM3MTgwMjRjMmI0NTZkOWE3ZmVkNWE3MjFpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDgzNWY4N2FkZTQwYmRjOGRmMjJjM2JiZDI3OGQ1N2UwNDBmYWEwOGNjNmU0MTVkNmJkMDM4NGViOWE3ZTUzY2VrcmVjb3JkZWRfYXQbAAZZaBkhf+s="
}
---

design doc .design/unified-fold.md checked against the live design-doc schema: validation: 9 check(s), 0 failed, 1 warning(s), 0 unchecked, 0 open question(s) [doc 5855:aee3c790077505b0]
***8<***
---
{
  "v": 3,
  "cid": "bafyreidmsck3exu4hljn3ev46gtbilrdt3emtqpou4mylturdu5sn7syde",
  "sig": "1f998d0ffb2074d7fec15c34ac1ec3bbe82c89465bc2a088fad31fd3f70fe0d87ce7e32603286aedb41e0b6ed7febdc7ecd37ed2f8987bad677e0d21e0476365",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "unified-fold"
  },
  "kind": "plan",
  "cites": [
    "bafyreieczbjpjyyjrr7sup42lbdyqna2oultt4adxrdp6gtmizxzflogje"
  ],
  "rev": "223mth2gmcx5s",
  "seq": 1,
  "of": 10,
  "text_len": 622,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4HYKlglAAFxEiCCyFL04wmMfyo/mlhHiDQadRc58AO8Rv8abEZvkq3GSWZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZVNkQmJyTTZOZVk0aThNb3h1Rml5RHR3eDNBSEo4ZmpFZDk0Z3JIMzFyOFhZZWFnZW509mdzdWJqZWN0oWVMb2NhbGx1bmlmaWVkLWZvbGRpYXJ0aWZhY3RzgaFmQ29tbWl0eChkZmY3ODA0MmRhZWY5ZjIzNzE4MDI0YzJiNDU2ZDlhN2ZlZDVhNzIxaXdvcmtzcGFjZaFpV29ya3NwYWNleEA4MzVmODdhZGU0MGJkYzhkZjIyYzNiYmQyNzhkNTdlMDQwZmFhMDhjYzZlNDE1ZDZiZDAzODRlYjlhN2U1M2Nla3JlY29yZGVkX2F0GwAGWWgZJHQE"
}
---

unified-fold design (.design/unified-fold.md): Replace cospan's many kan process spawns — `kan status` for the subject list plus a `kan show <subject>` per selection plus separate `--all` folds for cites and the process snapshot — with a **single `kan show --all --json` fold per tick**. Subjects, each subject's claims, the cid index, and the atom/telos/tension structure all derive from that one fold, held in memory and rebuilt only when the log changes. This removes the per-keystroke shell-out that makes the browser feel sluggish. [validation: 9 check(s), 0 failed, 1 warning(s), 0 unchecked, 0 open question(s)]
***8<***
---
{
  "v": 3,
  "cid": "bafyreiazo54jai3bk5pioj63wprf2tccjm4a6bhol3oxdhb65i7lbovtj4",
  "sig": "1e6a3695efbec1e798f00e569983164d1afe3ee36e36b72fa2c17383fb8f47bd61d3265dc13fe6ee6556fefb4ef25a594c5a52058e1ee5ded6b3ac4ff9f9d106",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "unified-fold"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223mth2gmewd2",
  "seq": 2,
  "of": 10,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXgsRmVhdHVyZTogT25lIGNhY2hlZCAtLWFsbCBmb2xkIChwZXJmb3JtYW5jZSlsc3ViamVjdF9raW5kZElkZWFlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2FsbHVuaWZpZWQtZm9sZGlhcnRpZmFjdHOBoWZDb21taXR4KGRmZjc4MDQyZGFlZjlmMjM3MTgwMjRjMmI0NTZkOWE3ZmVkNWE3MjFpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDgzNWY4N2FkZTQwYmRjOGRmMjJjM2JiZDI3OGQ1N2UwNDBmYWEwOGNjNmU0MTVkNmJkMDM4NGViOWE3ZTUzY2VrcmVjb3JkZWRfYXQbAAZZaBklcLc="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreihibd7vdxnfpxg2k45nybsoadubin7axzl5tujs3sq4m3lmazaiye",
  "sig": "3e9cde3eae079f7a15605b060841d075a9a3bb1a69dd4bbb55fcfd7abfda7b7941d2971c292cd3867c928c633031213bb53bbe31603bd3109064b0c5a2b08658",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "unified-fold"
  },
  "kind": "decision",
  "cites": [
    "bafyreidmsck3exu4hljn3ev46gtbilrdt3emtqpou4mylturdu5sn7syde"
  ],
  "rev": "223mth2gmkqwo",
  "seq": 3,
  "of": 10,
  "text_len": 220,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgbJCVsl6cOtLdkrzxphQuI57IycHupxmFzpEdOyb+WBlmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxsdW5pZmllZC1mb2xkaWFydGlmYWN0c4GhZkNvbW1pdHgoZGZmNzgwNDJkYWVmOWYyMzcxODAyNGMyYjQ1NmQ5YTdmZWQ1YTcyMWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAODM1Zjg3YWRlNDBiZGM4ZGYyMmMzYmJkMjc4ZDU3ZTA0MGZhYTA4Y2M2ZTQxNWQ2YmQwMzg0ZWI5YTdlNTNjZWtyZWNvcmRlZF9hdBsABlloGShbHA=="
}
---

RQ-1: The whole app derives from one `kan show --all --json` per tick (kan#123 makes this one spawn cheaper than N per-subject spawns); `day status` remains a second spawn only because day has no machine-readable output.
***8<***
---
{
  "v": 3,
  "cid": "bafyreifuiycac6xgqritvi7kakctehr2se33vyc4khsoua7jeud7ous3my",
  "sig": "30dc5e555ed6655fcb79b1c0fcb6b822d16c3ffa47d7ed19b2ba153c3ed35752715baeab32cd55867a2689c0e70b3f3771e1d34dfeae047eb0dc67e95cc9189c",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "unified-fold"
  },
  "kind": "decision",
  "cites": [
    "bafyreidmsck3exu4hljn3ev46gtbilrdt3emtqpou4mylturdu5sn7syde"
  ],
  "rev": "223mth2gmqqnc",
  "seq": 4,
  "of": 10,
  "text_len": 158,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgbJCVsl6cOtLdkrzxphQuI57IycHupxmFzpEdOyb+WBlmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxsdW5pZmllZC1mb2xkaWFydGlmYWN0c4GhZkNvbW1pdHgoZGZmNzgwNDJkYWVmOWYyMzcxODAyNGMyYjQ1NmQ5YTdmZWQ1YTcyMWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAODM1Zjg3YWRlNDBiZGM4ZGYyMmMzYmJkMjc4ZDU3ZTA0MGZhYTA4Y2M2ZTQxNWQ2YmQwMzg0ZWI5YTdlNTNjZWtyZWNvcmRlZF9hdBsABlloGStZ/A=="
}
---

RQ-2: The `subject` CLI subcommand keeps `subject_claims` (one subject, one spawn); only the TUI moves to the whole-log fold, where the amortization pays off.
***8<***
---
{
  "v": 3,
  "cid": "bafyreihrh2wx7rp4zttuq4hgobmrgl3ofujtju2rras5wkn5r6nbz6b5be",
  "sig": "2ee9742075f0c48aa125362a6e36b728d39461b014179486d53623ad2c7eb5c473a6c870041d724eaaa3f8e5489b207c2e28340393b457ca574c188d85ed2f21",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "unified-fold"
  },
  "kind": "decision",
  "cites": [
    "bafyreidmsck3exu4hljn3ev46gtbilrdt3emtqpou4mylturdu5sn7syde"
  ],
  "rev": "223mth3lx2x62",
  "seq": 5,
  "of": 10,
  "text_len": 1542,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgbJCVsl6cOtLdkrzxphQuI57IycHupxmFzpEdOyb+WBlmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxsdW5pZmllZC1mb2xkaWFydGlmYWN0c4GhZkNvbW1pdHgoZGZmNzgwNDJkYWVmOWYyMzcxODAyNGMyYjQ1NmQ5YTdmZWQ1YTcyMWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAODM1Zjg3YWRlNDBiZGM4ZGYyMmMzYmJkMjc4ZDU3ZTA0MGZhYTA4Y2M2ZTQxNWQ2YmQwMzg0ZWI5YTdlNTNjZWtyZWNvcmRlZF9hdBsABlloY9B0FA=="
}
---

adversarial review of unified-fold: APPROVE-WITH-FOLLOW-UPS — adversarial review of unified-fold: APPROVE-WITH-FOLLOW-UPS — the perf refactor to one kan show --all --json fold per tick is faithful and correct. Verified live: subject set parity kan status vs --all is exactly 34=34 identical (no subject dropped by removing kan_status); per-subject claim ordering parity (fold sorts newest-first via the same sort_newest_first used by both paths, identical sequence for a revisioned subject); one kan+one day spawn per changed tick and ZERO on a quiet tick (fold() only at --once/init/inside the should_refold mtime gate, never on draw or key path); kan-is-truth preserved (reads only, no writes); refold reset+by-name selection preservation is byte-for-byte the old behavior; by_cid indexes every claim across subjects; Fold::default degrades gracefully on kan/day failure (no panic). cargo test 44/44, clippy -D warnings clean (no dead code from the removals), build + fmt clean. Follow-ups (test rigor, none undermining the north star): (1) AC-1 newest-first is only transitively covered on the --all path — fold_indexes_every_claim_by_cid_across_subjects asserts claims_for len but not order; (2) AC-2 sessions() has no dedicated test with real agents/handoff subjects and namespace_counts is only tested indirectly via rows; (3) pre-existing: fold.errors is never surfaced in any view, so a kan/day failure renders as empty '(none)' indistinguishable from a genuinely empty repo (carried over from Dashboard.errors, not introduced).
***8<***
---
{
  "v": 3,
  "cid": "bafyreiecg2izta2fdmzpbiadko3mndweumeybelgn7nhu6ri32m66wmzoi",
  "sig": "ed9d8ab2802dd2b3161d1d2c845096470f603e1a7c54715ebe9f0c7c39f77cc943ba4be60d589c0b8f69090a5e3ab1518f109c03909b1f080cf9ec65eeba486c",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "unified-fold"
  },
  "kind": "observation",
  "cites": [
    "bafyreihrh2wx7rp4zttuq4hgobmrgl3ofujtju2rras5wkn5r6nbz6b5be"
  ],
  "rev": "223mth3mjic6e",
  "seq": 6,
  "of": 10,
  "text_len": 358,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIg8T6tf8X8zOdIcOZwWRMvbi0TNNNRiCXbKb2Pmhz4PQlmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxsdW5pZmllZC1mb2xkaWFydGlmYWN0c4GhZkNvbW1pdHgoZGZmNzgwNDJkYWVmOWYyMzcxODAyNGMyYjQ1NmQ5YTdmZWQ1YTcyMWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAODM1Zjg3YWRlNDBiZGM4ZGYyMmMzYmJkMjc4ZDU3ZTA0MGZhYTA4Y2M2ZTQxNWQ2YmQwMzg0ZWI5YTdlNTNjZWtyZWNvcmRlZF9hdBsABlloZPcgGA=="
}
---

AC-1 newest-first per subject is only transitively verified on the --all fold path: fold_indexes_every_claim_by_cid_across_subjects (substrate.rs:684) asserts claims_for("telos/b").len()==2 but not the newest-first order; it relies on sort_newest_first being covered by claims_from_json tests. A direct order assertion on populate_fold output would close it.
***8<***
---
{
  "v": 3,
  "cid": "bafyreibqgq7mgrnc4iuo4sngr3srxqkc7rxhqdx6lnpyibvp6byriaemsm",
  "sig": "8719a4ee8f203730f94b1b21a621b90f5bafbd5081077601f056aae810a8dfd019e350b12d5fde45581cafff2fdc2028ea79f86a77325f8ba648203670890d0d",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "unified-fold"
  },
  "kind": "observation",
  "cites": [
    "bafyreihrh2wx7rp4zttuq4hgobmrgl3ofujtju2rras5wkn5r6nbz6b5be"
  ],
  "rev": "223mth3mjpftt",
  "seq": 7,
  "of": 10,
  "text_len": 302,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIg8T6tf8X8zOdIcOZwWRMvbi0TNNNRiCXbKb2Pmhz4PQlmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxsdW5pZmllZC1mb2xkaWFydGlmYWN0c4GhZkNvbW1pdHgoZGZmNzgwNDJkYWVmOWYyMzcxODAyNGMyYjQ1NmQ5YTdmZWQ1YTcyMWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAODM1Zjg3YWRlNDBiZGM4ZGYyMmMzYmJkMjc4ZDU3ZTA0MGZhYTA4Y2M2ZTQxNWQ2YmQwMzg0ZWI5YTdlNTNjZWtyZWNvcmRlZF9hdBsABlloZPquxA=="
}
---

AC-2 partially covered: no dedicated unit test asserts sessions() (the agents/handoff registry) against a Fold that actually contains agents/handoff/* subjects, and namespace_counts() is only exercised indirectly through rebuild_rows. Both are pure map lookups but the AC-named coverage is superficial.
***8<***
---
{
  "v": 3,
  "cid": "bafyreidfxr6uadykdapnrdeoi3n6rjso4vb4bm4zej67hseltnxcurcjue",
  "sig": "9fd1d941919b81cc4d774ba38bdc09a81be7c53da9ae9af1f4ca51e0a5de78693a1f35a7952f36d2caaeeece2a9d503162f3c2b0eb8f2a98fdbabec4a9047e1b",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "unified-fold"
  },
  "kind": "observation",
  "cites": [
    "bafyreihrh2wx7rp4zttuq4hgobmrgl3ofujtju2rras5wkn5r6nbz6b5be"
  ],
  "rev": "223mth3mjvghk",
  "seq": 8,
  "of": 10,
  "text_len": 371,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIg8T6tf8X8zOdIcOZwWRMvbi0TNNNRiCXbKb2Pmhz4PQlmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxsdW5pZmllZC1mb2xkaWFydGlmYWN0c4GhZkNvbW1pdHgoZGZmNzgwNDJkYWVmOWYyMzcxODAyNGMyYjQ1NmQ5YTdmZWQ1YTcyMWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAODM1Zjg3YWRlNDBiZGM4ZGYyMmMzYmJkMjc4ZDU3ZTA0MGZhYTA4Y2M2ZTQxNWQ2YmQwMzg0ZWI5YTdlNTNjZWtyZWNvcmRlZF9hdBsABlloZP2xOw=="
}
---

Pre-existing (carried from Dashboard.errors, not introduced by unified-fold): Fold.errors is populated on kan/day spawn failure (substrate.rs:377,381) but never rendered in plain_frame or draw, so a failed 'kan show --all' renders as CLAIMS (none) indistinguishable from a genuinely empty repo — a mild telos/honest-ambiguity gap. Fold otherwise degrades without panic.
***8<***
---
{
  "v": 3,
  "cid": "bafyreig5xsbhuwge2u5f2434pscekcr2w6ejag6bzwopnrbcgoup7xz424",
  "sig": "7596838b6a7b3e8363bce639a9f3f8a9636c41dd22a50f1bbe649232644b486a388c92ac55dc07810d6786f421715361461724b99c7783c46db4f5470c0b538b",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "unified-fold"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mth3ra7pu2",
  "seq": 9,
  "of": 10,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxsdW5pZmllZC1mb2xkaWFydGlmYWN0c4GhZkNvbW1pdHgoZGZmNzgwNDJkYWVmOWYyMzcxODAyNGMyYjQ1NmQ5YTdmZWQ1YTcyMWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAODM1Zjg3YWRlNDBiZGM4ZGYyMmMzYmJkMjc4ZDU3ZTA0MGZhYTA4Y2M2ZTQxNWQ2YmQwMzg0ZWI5YTdlNTNjZWtyZWNvcmRlZF9hdBsABllobmLW3A=="
}
---
