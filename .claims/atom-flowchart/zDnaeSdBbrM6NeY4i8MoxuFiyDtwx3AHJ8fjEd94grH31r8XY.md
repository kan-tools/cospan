---
{
  "v": 3,
  "cid": "bafyreigxepypwyxbn27vymovbhxbrvjz3nlxidwtps3ylhjdwqmntptxhq",
  "sig": "e8e2d7476e0b42352f5474f6060feec4bdebbe69b27af7bebda21b0a7a4bb8c920360b4006f89c21288f2d11bb072285c6c8417d61dc25424b6b19b0bbfc76b9",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "atom-flowchart"
  },
  "kind": "observation",
  "cites": [],
  "rev": "223mtjy4ysxu4",
  "seq": 0,
  "of": 10,
  "text_len": 188,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2FsbmF0b20tZmxvd2NoYXJ0aWFydGlmYWN0c4GhZkNvbW1pdHgoZGJiNWNmYWZiZGMzZWFjMmM3NWZhYmVmZGI0ZWYzNjY5YTQzMjk5NWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABll/hexzrw=="
}
---

design doc .design/atom-flowchart.md checked against the live design-doc schema: validation: 9 check(s), 0 failed, 1 warning(s), 0 unchecked, 0 open question(s) [doc 6284:c646f341c98bb9f6]
***8<***
---
{
  "v": 3,
  "cid": "bafyreib7hm7op5u5ylqeuwrehu3gotaatk7wl4vhipikj4wbvdjrr5xmf4",
  "sig": "7c70a5f2257b2e6b5657dfbb9effab7eefc500e48a18d6adc2be5c37d070f98203901a6b9daf2c693c45ab0e351dbcb1799a7ef61049892c6d371ad64bbe66db",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "atom-flowchart"
  },
  "kind": "plan",
  "cites": [
    "bafyreigxepypwyxbn27vymovbhxbrvjz3nlxidwtps3ylhjdwqmntptxhq"
  ],
  "rev": "223mtjy4z4y4k",
  "seq": 1,
  "of": 10,
  "text_len": 614,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4HYKlglAAFxEiDXI/D7YuFuv1wx1QnuGNU521d0DtN8t4WdI7QY2b53PGZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZVNkQmJyTTZOZVk0aThNb3h1Rml5RHR3eDNBSEo4ZmpFZDk0Z3JIMzFyOFhZZWFnZW509mdzdWJqZWN0oWVMb2NhbG5hdG9tLWZsb3djaGFydGlhcnRpZmFjdHOBoWZDb21taXR4KGRiYjVjZmFmYmRjM2VhYzJjNzVmYWJlZmRiNGVmMzY2OWE0MzI5OTVpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZZf4Xxd9g="
}
---

atom-flowchart design (.design/atom-flowchart.md): Reshape the Process tab's atoms sub-pane from a flat list into an ASCII box-and-arrow **flowchart**: each atom is a labelled box, laid out in columns by its depth in the atom DAG, with `──▶` arrows along the `next` edges. A selected box is highlighted (double border), and Enter drills into that atom's full detail (in/out/next/done/revisits). Edges the 2D layout cannot cleanly route (back-edges, row jumps) are listed textually rather than faked (`telos/honest-ambiguity`). [validation: 9 check(s), 0 failed, 1 warning(s), 0 unchecked, 0 open question(s)]
***8<***
---
{
  "v": 3,
  "cid": "bafyreiekl6tfcvodxowkded2poa3rpss5y45t2y4rcrirb3d5afhjnj2dq",
  "sig": "09c8ac3ca73fc11ae80715ee2d0c4da6adbcfe1043b144057eeb97ce52430a6271ec5bfc3df5d2d6ecbb82768af8d116f27a458817cf00467c57a7b0c9a9c9fb",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "atom-flowchart"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223mtjy4za6uj",
  "seq": 2,
  "of": 10,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXgpRmVhdHVyZTogdGhlIGF0b20gZmxvd2NoYXJ0IChQcm9jZXNzIHRhYilsc3ViamVjdF9raW5kZElkZWFlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2FsbmF0b20tZmxvd2NoYXJ0aWFydGlmYWN0c4GhZkNvbW1pdHgoZGJiNWNmYWZiZGMzZWFjMmM3NWZhYmVmZGI0ZWYzNjY5YTQzMjk5NWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABll/hfMS4A=="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreia4mbcvikekq2r5uvt75xmvwi2ey7zdnmvq4hja6x5bjfyfyc7rda",
  "sig": "3d55daefae124894581ad3f0724869f54813f7aa25d3896c845cc6382b6790ca43eedf53e3b9c89febbcbc85af202ea046adc685849f99a99d52f3ead435d59d",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "atom-flowchart"
  },
  "kind": "decision",
  "cites": [
    "bafyreib7hm7op5u5ylqeuwrehu3gotaatk7wl4vhipikj4wbvdjrr5xmf4"
  ],
  "rev": "223mtjy4zk5t4",
  "seq": 3,
  "of": 10,
  "text_len": 212,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgPzs+5/adwuBKWiQ9NmdMAJq/ZfKnQ9Ck8sGo0xj27C9mYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxuYXRvbS1mbG93Y2hhcnRpYXJ0aWZhY3RzgaFmQ29tbWl0eChkYmI1Y2ZhZmJkYzNlYWMyYzc1ZmFiZWZkYjRlZjM2NjlhNDMyOTk1aXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWX+F+A6s"
}
---

RQ-1: Layout is by longest-path column with bounded relaxation (cycle-safe) rather than a full graph-layout library; day's atom graph is small and mostly linear, so this renders cleanly and stays dependency-free.
***8<***
---
{
  "v": 3,
  "cid": "bafyreiek2bzjjk53pdltw7s5ypb4ccbhmzyhty2ippcgoko6qoqf5ntumi",
  "sig": "7c7c2fea4eca2c97a6bd7e91fbd12c25a42d9151da919ab5a4d5d81c1cd1b5b4200cd9f48113bc85453d097e1bd349a68a6b1c85bd3d601b69dad1c976d711e4",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "atom-flowchart"
  },
  "kind": "decision",
  "cites": [
    "bafyreib7hm7op5u5ylqeuwrehu3gotaatk7wl4vhipikj4wbvdjrr5xmf4"
  ],
  "rev": "223mtjy4ztym5",
  "seq": 4,
  "of": 10,
  "text_len": 201,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgPzs+5/adwuBKWiQ9NmdMAJq/ZfKnQ9Ck8sGo0xj27C9mYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxuYXRvbS1mbG93Y2hhcnRpYXJ0aWZhY3RzgaFmQ29tbWl0eChkYmI1Y2ZhZmJkYzNlYWMyYzc1ZmFiZWZkYjRlZjM2NjlhNDMyOTk1aXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWX+F/PnN"
}
---

RQ-2: Edges the grid cannot route on a straight same-row arrow are listed as text (`from ⇢ to`) rather than drawn with approximate/curved ASCII, keeping the picture honest (`telos/honest-ambiguity`).
***8<***
---
{
  "v": 3,
  "cid": "bafyreihjny74k2vretbswwfp6xk63fotebvhpyymafdr4vpfar3ruf3plq",
  "sig": "d24ac3edf7dd37a9453a4e17253c41a48ed5790d28d86769f9e912b65be4a2ac0f2fc25b179cb85509b9fe5b02514533c4b65c934f8bebbda470affa1e07e28e",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "atom-flowchart"
  },
  "kind": "decision",
  "cites": [
    "bafyreib7hm7op5u5ylqeuwrehu3gotaatk7wl4vhipikj4wbvdjrr5xmf4"
  ],
  "rev": "223mtjy525ti3",
  "seq": 5,
  "of": 10,
  "text_len": 159,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgPzs+5/adwuBKWiQ9NmdMAJq/ZfKnQ9Ck8sGo0xj27C9mYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxuYXRvbS1mbG93Y2hhcnRpYXJ0aWZhY3RzgaFmQ29tbWl0eChkYmI1Y2ZhZmJkYzNlYWMyYzc1ZmFiZWZkYjRlZjM2NjlhNDMyOTk1aXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWX+GAeVI"
}
---

RQ-3: The selected box is shown by a double border (a different box-drawing style) rather than a color span, so the char-grid render needs no per-cell styling.
***8<***
---
{
  "v": 3,
  "cid": "bafyreieuexshrivfb5p2btaayv5beom3trugsnty2g7xvll45rds4kkak4",
  "sig": "7d95aa4dca357c3a60d633e5edeac6c4c198dea289ae9a4c8d92b555f486b0005f1a3549ad90241451047c836885f365a737a1aadeb74caa78c140751170a7c5",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "atom-flowchart"
  },
  "kind": "decision",
  "cites": [
    "bafyreia4mbcvikekq2r5uvt75xmvwi2ey7zdnmvq4hja6x5bjfyfyc7rda"
  ],
  "rev": "223mtjysz4eph",
  "seq": 6,
  "of": 10,
  "text_len": 1357,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgHGBFVCiKhqPaVn/t2VsjRMfyNrKw4dIPX6FJcFwL8RhmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxuYXRvbS1mbG93Y2hhcnRpYXJ0aWZhY3RzgaFmQ29tbWl0eChkYmI1Y2ZhZmJkYzNlYWMyYzc1ZmFiZWZkYjRlZjM2NjlhNDMyOTk1aXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWX+x8Sdo"
}
---

adversarial review of atom-flowchart: SHIP. Independent Opus review traced every usize subtraction and grid index in atom_flowchart: no reachable panic. grid_h = n_rows*ROW_H-1 (n_rows>=1 so >=3); every box write in bounds (box right edge < col_x[c]+col_w[c] <= grid_w), including the cycle case with empty leading columns; arrow x1-x0 >= GAP=5 so x1-1>=4 (no underflow), guarded x1>x0, never drops a same-row/next-col edge or overpaints a box; rows unique per column (per-column counter) so no collision/overpaint; selected>=n renders no highlight, no panic; n=1/empty-slug/disconnected/fan-in/self-loop all safe; empty atoms early-returns. Layout: bounded relaxation terminates on cycles; unknown next targets filtered and surfaced as ⇢(unknown). State machine: stale atom_selected after a shrinking re-fold is harmless (.get guards); Enter in Telos inert (correct). Honest-ambiguity honored (unroutable/back edges listed ⇢/↻, never faked); pure projection. Closed two non-blocking notes: process_detail now resets on a pane switch (was a recoverable stranded-detail wart); added a ⇢ off-row-edge assertion (AC-2 coverage). Wide-char box misalignment left as cosmetic/unreachable (ASCII slugs). Real DAG renders cleanly (boxes + ────▶ + double-border selection + back-edge listed). 90 tests, clippy -D warnings, fmt clean. Verdict: SHIP.
***8<***
---
{
  "v": 3,
  "cid": "bafyreiay5s6xuifwtjglkkiwhjigserbwkcdym7nggjtblbrnumaqexmdq",
  "sig": "baac7dda5d661c3c20d79822157c138e77c7bb3e341e03ebdb89b28aa34d0cb86fb7674613cabb8f26243aa49f0c78ec75a75c2231aba62bff66209178bce92f",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "atom-flowchart"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mtjyszgsws",
  "seq": 7,
  "of": 10,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxuYXRvbS1mbG93Y2hhcnRpYXJ0aWZhY3RzgaFmQ29tbWl0eChkYmI1Y2ZhZmJkYzNlYWMyYzc1ZmFiZWZkYjRlZjM2NjlhNDMyOTk1aXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWX+x9mMi"
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreig6mmbuudjlysvk3qrs7n5kn3niqhofl7l7iymxh3wcon3nn7pcsy",
  "sig": "8b3c1bc43b84e40cdcc55d94ac654bdb28feaf2cdd39698a568e55b14fb4eaa95b6f40e8ffda540530c638f839d7a0769e4346b56685f24119772bf5672f4fed",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "atom-flowchart"
  },
  "kind": "decision",
  "cites": [
    "bafyreia4mbcvikekq2r5uvt75xmvwi2ey7zdnmvq4hja6x5bjfyfyc7rda"
  ],
  "rev": "223mtk3sjly2s",
  "seq": 8,
  "of": 10,
  "text_len": 1055,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgHGBFVCiKhqPaVn/t2VsjRMfyNrKw4dIPX6FJcFwL8RhmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxuYXRvbS1mbG93Y2hhcnRpYXJ0aWZhY3RzgaFmQ29tbWl0eChjZjU4NjQ2NjY1ZDdlN2JkMDY1Mjk0MzY5ZmQxNjVjZDQ4MTdmZmUwaXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWYBw+Pex"
}
---

adversarial review of atom-flowchart (back-edge routing addendum): SHIP. Added per user request: back-flows (revisits + off-column/off-row next) are now ROUTED as dashed arrows (┄/┆/╰/╯/▲) in stacked lanes below the boxes in a SINGLE-ROW layout, visually distinct from the solid forward ──▶; MULTI-ROW layouts keep the ⇢/↻ text list (routing across rows would cross boxes — honest-ambiguity). Independent Opus review proved the lane bounds safe: routed is non-empty only when single_row, so boxes_h=3 and grid_h=4+routed.len(); max lane_y=3+routed.len()<grid_h; box_center<grid_w strictly. The sx==tx self-edge (a self-revisiting atom) is guarded by `if lo+1<hi` and tested (loop revisits loop, no panic); distinct columns in single-row make adjacent-center inversion impossible. Classification correct (revisit never forward via !revisit guard; unknown target -> ⇢(unknown); multi-row adds no lanes). No regression (forward arrows from a collected list, identical geometry). 91 tests, clippy -D warnings, fmt clean. Verdict: SHIP.
***8<***
---
{
  "v": 3,
  "cid": "bafyreicw3hfbb2vagjrr7wb32ql2qnoh75vfuq4v6ifxvcdwr6ktdbdsmu",
  "sig": "697e58689eeafa127df3842a638ad5e2a947a493968c2a503bec8f9df87f311121c4310e43a741c42fda556108d2194b7239ae7214309e648ac738ab7d1dac47",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "atom-flowchart"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mtk3sjvdce",
  "seq": 9,
  "of": 10,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxuYXRvbS1mbG93Y2hhcnRpYXJ0aWZhY3RzgaFmQ29tbWl0eChjZjU4NjQ2NjY1ZDdlN2JkMDY1Mjk0MzY5ZmQxNjVjZDQ4MTdmZmUwaXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWYBw/aSV"
}
---
