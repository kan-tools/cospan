---
{
  "v": 3,
  "cid": "bafyreigcrsbenhiiefqaek4gzjxlfxznm2fzjzo433oa5vfkthaavnxqou",
  "sig": "0a28e61cd78efc8806d9b735441c1255de59d5d1952d5d6835d5b90ef68503e54f4825e23254e5a5e8dd9e69302fb66cda586c7ff759fe6cba8b2dbadd7bd12c",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "ratatui-interactive-tui"
  },
  "kind": "observation",
  "cites": [],
  "rev": "223mtfh7rsksj",
  "seq": 0,
  "of": 9,
  "text_len": 197,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2Fsd3JhdGF0dWktaW50ZXJhY3RpdmUtdHVpaWFydGlmYWN0c4GhZkNvbW1pdHgoYjljZmYzNjBhNjU2MzVhMTdiZmJlMzZiZjc1NmQ4YzIwODQ0YzQzYml3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAODM1Zjg3YWRlNDBiZGM4ZGYyMmMzYmJkMjc4ZDU3ZTA0MGZhYTA4Y2M2ZTQxNWQ2YmQwMzg0ZWI5YTdlNTNjZWtyZWNvcmRlZF9hdBsABllbS3xCow=="
}
---

design doc .design/ratatui-interactive-tui.md checked against the live design-doc schema: validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s) [doc 8120:825715d10b85dbe7]
***8<***
---
{
  "v": 3,
  "cid": "bafyreibtcchtu4eelybfchchidpuglfog6qicjou4vht4rlqzbwx6go6oi",
  "sig": "d745a47b92d64ce9a988f09d718d54148000cc6696a1af2513a51a1748b06d67416792192cbdaa3d55b39d8425dcd1f0dbce92290e9c188648c45104d1a8815f",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "ratatui-interactive-tui"
  },
  "kind": "plan",
  "cites": [
    "bafyreigcrsbenhiiefqaek4gzjxlfxznm2fzjzo433oa5vfkthaavnxqou"
  ],
  "rev": "223mtfh7rvoy4",
  "seq": 1,
  "of": 9,
  "text_len": 629,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4HYKlglAAFxEiDCjIJGnQghYAIrhspust8tZouU5dze3A7UqpnACrbwdWZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZVNkQmJyTTZOZVk0aThNb3h1Rml5RHR3eDNBSEo4ZmpFZDk0Z3JIMzFyOFhZZWFnZW509mdzdWJqZWN0oWVMb2NhbHdyYXRhdHVpLWludGVyYWN0aXZlLXR1aWlhcnRpZmFjdHOBoWZDb21taXR4KGI5Y2ZmMzYwYTY1NjM1YTE3YmZiZTM2YmY3NTZkOGMyMDg0NGM0M2Jpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDgzNWY4N2FkZTQwYmRjOGRmMjJjM2JiZDI3OGQ1N2UwNDBmYWEwOGNjNmU0MTVkNmJkMDM4NGViOWE3ZTUzY2VrcmVjb3JkZWRfYXQbAAZZW0t901o="
}
---

ratatui-interactive-tui design (.design/ratatui-interactive-tui.md): Turn the `watch-repo` print-loop into a real interactive terminal UI built on ratatui + crossterm: a poll-driven app loop that re-folds the substrate only when `.kan/log/HEAD` changes, redraws without flicker, and lets you move a selection over the repo's subjects with `j`/`k` and quit with `q`. The existing P0 dashboard is ported into a single ratatui view — no new panes yet. It is Step 2 of the P0 arc and the foothold every later view builds on. Serves `telos/p0-spine`. [validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s)]
***8<***
---
{
  "v": 3,
  "cid": "bafyreifickddaukojui2enu27vk5kbe5z3z2ffa32hzgwpxguhw76vuqly",
  "sig": "15bdc13c1ac6da3c070704b2331ef79464f6a5413c18cb17cef6045d86966c165dff0bbd5fbac95a8d85e70fc74f1607d13a8ebc579377e4c66d85297107ed2f",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "ratatui-interactive-tui"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223mtfh7rwqrp",
  "seq": 2,
  "of": 9,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXgvRmVhdHVyZTogSW50ZXJhY3RpdmUgcmF0YXR1aSBUVUkgZm9yIHdhdGNoLXJlcG9sc3ViamVjdF9raW5kZElkZWFlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2Fsd3JhdGF0dWktaW50ZXJhY3RpdmUtdHVpaWFydGlmYWN0c4GhZkNvbW1pdHgoYjljZmYzNjBhNjU2MzVhMTdiZmJlMzZiZjc1NmQ4YzIwODQ0YzQzYml3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAODM1Zjg3YWRlNDBiZGM4ZGYyMmMzYmJkMjc4ZDU3ZTA0MGZhYTA4Y2M2ZTQxNWQ2YmQwMzg0ZWI5YTdlNTNjZWtyZWNvcmRlZF9hdBsABllbS35akg=="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreic2r4m7l6jmppry7rciv362nckqmpqwxb5qbjwge36jwa5spxxn7u",
  "sig": "50e07fcd2d4bd8058194b1cd10c44ec69709ece4d7f3031e785cf9e0388d3f3c3fe6505a20182bad6ec5144498e61597a5185568b8ba137375e416c7b8f9771a",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "ratatui-interactive-tui"
  },
  "kind": "decision",
  "cites": [
    "bafyreibtcchtu4eelybfchchidpuglfog6qicjou4vht4rlqzbwx6go6oi"
  ],
  "rev": "223mtfh7s3mfl",
  "seq": 3,
  "of": 9,
  "text_len": 228,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgMxCPOnCEXgJRHEdA30MsrjeggSXU5U8+RXDIbX8Z3nJmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx3cmF0YXR1aS1pbnRlcmFjdGl2ZS10dWlpYXJ0aWZhY3RzgaFmQ29tbWl0eChiOWNmZjM2MGE2NTYzNWExN2JmYmUzNmJmNzU2ZDhjMjA4NDRjNDNiaXdvcmtzcGFjZaFpV29ya3NwYWNleEA4MzVmODdhZGU0MGJkYzhkZjIyYzNiYmQyNzhkNTdlMDQwZmFhMDhjYzZlNDE1ZDZiZDAzODRlYjlhN2U1M2Nla3JlY29yZGVkX2F0GwAGWVtLgMkI"
}
---

RQ-1: `j`/`k` moves the selection over the grouped subject list, not the namespace count rows — a selected subject is exactly what Step 3's claim-detail pane will open, so the selection has a downstream meaning from the start.
***8<***
---
{
  "v": 3,
  "cid": "bafyreiasetvxildcfpk3um66kcmyqwrld67kdcqbo4himw5k2wovl3nb4q",
  "sig": "218df22d379063c642ff17844a80da81c06521eadcb5a001b64b65a19bf0891816bf53236d6d58b1fc2936d6f294f3b4ab02dc1530f28f126f1ad14d6da41a45",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "ratatui-interactive-tui"
  },
  "kind": "decision",
  "cites": [
    "bafyreibtcchtu4eelybfchchidpuglfog6qicjou4vht4rlqzbwx6go6oi"
  ],
  "rev": "223mtfh7s6rdt",
  "seq": 4,
  "of": 9,
  "text_len": 210,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgMxCPOnCEXgJRHEdA30MsrjeggSXU5U8+RXDIbX8Z3nJmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx3cmF0YXR1aS1pbnRlcmFjdGl2ZS10dWlpYXJ0aWZhY3RzgaFmQ29tbWl0eChiOWNmZjM2MGE2NTYzNWExN2JmYmUzNmJmNzU2ZDhjMjA4NDRjNDNiaXdvcmtzcGFjZaFpV29ya3NwYWNleEA4MzVmODdhZGU0MGJkYzhkZjIyYzNiYmQyNzhkNTdlMDQwZmFhMDhjYzZlNDE1ZDZiZDAzODRlYjlhN2U1M2Nla3JlY29yZGVkX2F0GwAGWVtLglzG"
}
---

RQ-2: `--once` stays a non-interactive plain-text single frame (scriptable, CI-friendly, and the seam that keeps the dashboard render unit-testable); the interactive default enters the ratatui alternate screen.
***8<***
---
{
  "v": 3,
  "cid": "bafyreidhzkxd3dkcsbwzgevvkr6xqu4pohwsjv5gnpboik3ojbp2mnxeym",
  "sig": "cd3b7719f740973a292d8eca29305d1666901cde07e5402f9846c29ad7b324645f1e0837bbc658d5481506730d54e9ff39e9b0b055b1626fee5f95a5e62f4ce3",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "ratatui-interactive-tui"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mtfhajthoq",
  "seq": 5,
  "of": 9,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx3cmF0YXR1aS1pbnRlcmFjdGl2ZS10dWlpYXJ0aWZhY3RzgaFmQ29tbWl0eChiOWNmZjM2MGE2NTYzNWExN2JmYmUzNmJmNzU2ZDhjMjA4NDRjNDNiaXdvcmtzcGFjZaFpV29ya3NwYWNleEA4MzVmODdhZGU0MGJkYzhkZjIyYzNiYmQyNzhkNTdlMDQwZmFhMDhjYzZlNDE1ZDZiZDAzODRlYjlhN2U1M2Nla3JlY29yZGVkX2F0GwAGWVtM/LYv"
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreidq3ijchktwrf3eanx2kkqlwxogzvzv3bzqfkykqqmy73wjzt6d74",
  "sig": "641d98fac37d5097d4fb88a115f52837b1431b90eb79f52f97cdf4cbba81d7c0431540ca7e949c1623804e873676eb04849777e729504e52aaa3571fe84c6c0b",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "ratatui-interactive-tui"
  },
  "kind": "decision",
  "cites": [
    "bafyreibtcchtu4eelybfchchidpuglfog6qicjou4vht4rlqzbwx6go6oi"
  ],
  "rev": "223mtfjn63kuj",
  "seq": 6,
  "of": 9,
  "text_len": 563,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgMxCPOnCEXgJRHEdA30MsrjeggSXU5U8+RXDIbX8Z3nJmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx3cmF0YXR1aS1pbnRlcmFjdGl2ZS10dWlpYXJ0aWZhY3RzgaFmQ29tbWl0eChiOWNmZjM2MGE2NTYzNWExN2JmYmUzNmJmNzU2ZDhjMjA4NDRjNDNiaXdvcmtzcGFjZaFpV29ya3NwYWNleEA4MzVmODdhZGU0MGJkYzhkZjIyYzNiYmQyNzhkNTdlMDQwZmFhMDhjYzZlNDE1ZDZiZDAzODRlYjlhN2U1M2Nla3JlY29yZGVkX2F0GwAGWVvmQMLm"
}
---

adversarial review of ratatui-interactive-tui: APPROVE-WITH-FOLLOW-UPS — adversarial review of Step 2 (ratatui interactive TUI): build/18 tests/clippy green, --once verbatim & exits 0, terminal restore + panic hook verified in dep source, poll-and-fold honored. Follow-ups: interactive process pane Constraint::Max(9) silently clips day's 19-line status with no overflow cue (brushes honest-ambiguity, interactive-only); AC-3 refold-when-gone jumps to first not a numeric clamp and its single-subject test can't distinguish. No correctness/invariant violations.
***8<***
---
{
  "v": 3,
  "cid": "bafyreic2pwfyufhkikrowtmws72i2tmcbgdmobhkyfi3nfor7lvggbr4ym",
  "sig": "5a8c8c232f7f33c815deb7710659addcea17723f634de12aff3909dbd4c4ef4227f373a8a0db0a4b64ca7895adff6915b9d8df009a0c4c57d1305dab03d60977",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "ratatui-interactive-tui"
  },
  "kind": "observation",
  "cites": [
    "bafyreidq3ijchktwrf3eanx2kkqlwxogzvzv3bzqfkykqqmy73wjzt6d74"
  ],
  "rev": "223mtfjnizc2v",
  "seq": 7,
  "of": 9,
  "text_len": 360,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgcNoSI6p2iXZANvpSoLtdxs1zXYcwKrCoQZj+7JzPw/9mYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx3cmF0YXR1aS1pbnRlcmFjdGl2ZS10dWlpYXJ0aWZhY3RzgaFmQ29tbWl0eChiOWNmZjM2MGE2NTYzNWExN2JmYmUzNmJmNzU2ZDhjMjA4NDRjNDNiaXdvcmtzcGFjZaFpV29ya3NwYWNleEA4MzVmODdhZGU0MGJkYzhkZjIyYzNiYmQyNzhkNTdlMDQwZmFhMDhjYzZlNDE1ZDZiZDAzODRlYjlhN2U1M2Nla3JlY29yZGVkX2F0GwAGWVvm75+w"
}
---

FINDING (medium, honest-ambiguity): interactive draw puts day status in Constraint::Max(9) pane (tui.rs:258-260) with Wrap and no scroll/overflow indicator; day status is 19 lines so ~12 lines including Off-sequence/next candidates are silently clipped in the primary interactive view. --once renders all lines verbatim; interactive-only, unverifiable by test.
***8<***
---
{
  "v": 3,
  "cid": "bafyreicrfg7tor2db3xrdvf5iatasu3ztgte5qvpvwb7zui6lufx3aboam",
  "sig": "cd94ac197c559843470eee4ffddc9e52272ab9ed1512101adda4f8fa30416a547755809eecb9cdf188a2879333e8a9958c2e27552e2d2c0b53428b80aabb6f0a",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "ratatui-interactive-tui"
  },
  "kind": "observation",
  "cites": [
    "bafyreidq3ijchktwrf3eanx2kkqlwxogzvzv3bzqfkykqqmy73wjzt6d74"
  ],
  "rev": "223mtfjnj4lwy",
  "seq": 8,
  "of": 9,
  "text_len": 332,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgcNoSI6p2iXZANvpSoLtdxs1zXYcwKrCoQZj+7JzPw/9mYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx3cmF0YXR1aS1pbnRlcmFjdGl2ZS10dWlpYXJ0aWZhY3RzgaFmQ29tbWl0eChiOWNmZjM2MGE2NTYzNWExN2JmYmUzNmJmNzU2ZDhjMjA4NDRjNDNiaXdvcmtzcGFjZaFpV29ya3NwYWNleEA4MzVmODdhZGU0MGJkYzhkZjIyYzNiYmQyNzhkNTdlMDQwZmFhMDhjYzZlNDE1ZDZiZDAzODRlYjlhN2U1M2Nla3JlY29yZGVkX2F0GwAGWVvm8Uc3"
}
---

FINDING (low): refold fallback when selected subject is gone jumps to first_subject_index (tui.rs:121-128), not the numeric clamp AC-3/REQ-5 wording implies; safe/in-range but AC-3 test uses a single remaining subject (tui.rs:394-396) so it cannot distinguish jump-to-first from clamp-to-nearest -- AC-3 only superficially verified.
