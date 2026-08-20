---
{
  "v": 3,
  "cid": "bafyreidjf62bdvimw2ejwpiovefc2n7gvw77xizslgjbc63kdioy3wft2u",
  "sig": "036728c6dcfd1ade29d79f3a0d95b7a31175d9c8aef2e058117e0942ab04631c4656d63718645de257a1d0546c32fdf09d1c686e6d15bf5206712784e0c45d9f",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-column-reflow"
  },
  "kind": "observation",
  "cites": [],
  "rev": "223mtjfv5jt47",
  "seq": 0,
  "of": 9,
  "text_len": 195,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2FsdWNvbW1lbnQtY29sdW1uLXJlZmxvd2lhcnRpZmFjdHOBoWZDb21taXR4KGUxZGJhY2ZkY2QyOGVkYTA5OTE5ZGNlYmYwZTI3NzYwMGRhZjYyYjRpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZZevY3494="
}
---

design doc .design/comment-column-reflow.md checked against the live design-doc schema: validation: 9 check(s), 0 failed, 1 warning(s), 0 unchecked, 0 open question(s) [doc 6547:221caec513fec5d2]
***8<***
---
{
  "v": 3,
  "cid": "bafyreic7cjfrvhnaqurjids2rchjdlixwqdf5xcj56e4oqcmmxioopwarq",
  "sig": "fc23aab214d5cec6c950680a4b7558827cd15a79791b81cc6e26c113fc01857d12d862ed60fc354e6474eb6b44964f6ae648f6dbf1cd85af9e489114bf343322",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-column-reflow"
  },
  "kind": "plan",
  "cites": [
    "bafyreidjf62bdvimw2ejwpiovefc2n7gvw77xizslgjbc63kdioy3wft2u"
  ],
  "rev": "223mtjfv5rzmr",
  "seq": 1,
  "of": 9,
  "text_len": 672,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4HYKlglAAFxEiBpL7QR1Qy2iJs9DqkKLTfmrb/7ozJZkhF7ahodjdiz1WZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZVNkQmJyTTZOZVk0aThNb3h1Rml5RHR3eDNBSEo4ZmpFZDk0Z3JIMzFyOFhZZWFnZW509mdzdWJqZWN0oWVMb2NhbHVjb21tZW50LWNvbHVtbi1yZWZsb3dpYXJ0aWZhY3RzgaFmQ29tbWl0eChlMWRiYWNmZGNkMjhlZGEwOTkxOWRjZWJmMGUyNzc2MDBkYWY2MmI0aXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWXr2O/3s"
}
---

comment-column-reflow design (.design/comment-column-reflow.md): Render comments as notes in a right column beside the lines they reference, and reflow the code column — a multi-line note pushes the following code lines down so notes never overlap code or each other. This is the comment view's flagship layout from `.dropbox/05-views-ux.md` (d): "a multi-line comment expands and reflows the text column to make room (the requirement that rules out tmux panes — cospan draws this itself)." It replaces reading comments one-at-a-time in the bottom strip with seeing them all in context. [validation: 9 check(s), 0 failed, 1 warning(s), 0 unchecked, 0 open question(s)]
***8<***
---
{
  "v": 3,
  "cid": "bafyreic2pfsi5u6kioju44jequqksyszdzl3q5q2ugikmbuzidmbkpuxze",
  "sig": "7d4eac6485621e5b90eadd352cea38f997dad93f7aab3dbe475a9d002a2c084f1236a2899f88ed6ce785b050da05cee434c2110d215c893109445b348129bba0",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-column-reflow"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223mtjfv5usy3",
  "seq": 2,
  "of": 9,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXgyRmVhdHVyZTogcmlnaHQtYW5jaG9yZWQgY29tbWVudCBjb2x1bW4gd2l0aCByZWZsb3dsc3ViamVjdF9raW5kZElkZWFlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2FsdWNvbW1lbnQtY29sdW1uLXJlZmxvd2lhcnRpZmFjdHOBoWZDb21taXR4KGUxZGJhY2ZkY2QyOGVkYTA5OTE5ZGNlYmYwZTI3NzYwMGRhZjYyYjRpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZZevY9Y1o="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreic45idhrioielngtpmhui6fqg3jpcwob7wbywdoj6lbna2ycd6rxq",
  "sig": "ca712a4197e3b8d55c75d94ac931aede689e250cc116399eb89e2d0ce76f9caa723adb90eeecd482fae1b70f7109b259fb27ce3db94c77baaff12cc4aca4ff83",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-column-reflow"
  },
  "kind": "decision",
  "cites": [
    "bafyreic7cjfrvhnaqurjids2rchjdlixwqdf5xcj56e4oqcmmxioopwarq"
  ],
  "rev": "223mtjfv64z3z",
  "seq": 3,
  "of": 9,
  "text_len": 244,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgXxJLGp2ghSKUDlqIjpGtF7QGXtxJ74nHQExl0Oc+wIxmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx1Y29tbWVudC1jb2x1bW4tcmVmbG93aWFydGlmYWN0c4GhZkNvbW1pdHgoZTFkYmFjZmRjZDI4ZWRhMDk5MTlkY2ViZjBlMjc3NjAwZGFmNjJiNGl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABll69kF70g=="
}
---

RQ-1: The note column carries a compact note (capped body, reply count), not the full thread; the full thread and the unresolvable list stay in the bottom strip, so the column reads at a glance and long threads do not blow up the reflow height.
***8<***
---
{
  "v": 3,
  "cid": "bafyreifwl476vje4a2aeumcouotcbpwktb5snqgpo4dqm5caswimrzi4pi",
  "sig": "ec9efcbfaa35caac5f833e0cc5412403de61aa1b49132d75789da32ac386c8136cc9726eb96d6c3a2ffe143f0c9893063f94d18d21c595431d0b2cdaca178027",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-column-reflow"
  },
  "kind": "decision",
  "cites": [
    "bafyreic7cjfrvhnaqurjids2rchjdlixwqdf5xcj56e4oqcmmxioopwarq"
  ],
  "rev": "223mtjfv6favt",
  "seq": 4,
  "of": 9,
  "text_len": 204,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgXxJLGp2ghSKUDlqIjpGtF7QGXtxJ74nHQExl0Oc+wIxmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx1Y29tbWVudC1jb2x1bW4tcmVmbG93aWFydGlmYWN0c4GhZkNvbW1pdHgoZTFkYmFjZmRjZDI4ZWRhMDk5MTlkY2ViZjBlMjc3NjAwZGFmNjJiNGl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABll69kWbDQ=="
}
---

RQ-2: Reflow pushes code down (blank code cells) rather than overlaying notes or truncating them, matching the design's "expands the text column to make room" and avoiding overlap between nearby comments.
***8<***
---
{
  "v": 3,
  "cid": "bafyreidm63j4lne2ajnxbfwolk42gtn3gze36r4unrfycfi2xoeuetnqom",
  "sig": "de6b9e86f26003ddb8e634453a66fb56da0601bef15ab3f8ebb01625be4fd0d043ba0cb16739c5382519bde057c2d3513ccc6ec7ca4a2e5856001fc5b7b89ad0",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-column-reflow"
  },
  "kind": "decision",
  "cites": [
    "bafyreic7cjfrvhnaqurjids2rchjdlixwqdf5xcj56e4oqcmmxioopwarq"
  ],
  "rev": "223mtjfv6ngit",
  "seq": 5,
  "of": 9,
  "text_len": 124,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgXxJLGp2ghSKUDlqIjpGtF7QGXtxJ74nHQExl0Oc+wIxmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx1Y29tbWVudC1jb2x1bW4tcmVmbG93aWFydGlmYWN0c4GhZkNvbW1pdHgoZTFkYmFjZmRjZDI4ZWRhMDk5MTlkY2ViZjBlMjc3NjAwZGFmNjJiNGl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABll69kmxbw=="
}
---

RQ-3: The column is wide-layout only; narrow keeps the strip-based view rather than cramming a column into too little width.
***8<***
---
{
  "v": 3,
  "cid": "bafyreiget3i4j2wow3ge2hsf5f2o5q6edifzkroakdtfitjlk37ttgfs4q",
  "sig": "dad16f0fccff6d9f0714658a39e17414fbb58635e3ed2ec2657a63e249aec5fc2f955d9400fb926e7387209bde7641cb472df076112f9f29a31028655ef14329",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-column-reflow"
  },
  "kind": "decision",
  "cites": [
    "bafyreic7cjfrvhnaqurjids2rchjdlixwqdf5xcj56e4oqcmmxioopwarq"
  ],
  "rev": "223mtjfv6vpe7",
  "seq": 6,
  "of": 9,
  "text_len": 156,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgXxJLGp2ghSKUDlqIjpGtF7QGXtxJ74nHQExl0Oc+wIxmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx1Y29tbWVudC1jb2x1bW4tcmVmbG93aWFydGlmYWN0c4GhZkNvbW1pdHgoZTFkYmFjZmRjZDI4ZWRhMDk5MTlkY2ViZjBlMjc3NjAwZGFmNjJiNGl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABll69k3U2A=="
}
---

RQ-4: Scroll follows the selected comment (its note row is kept in view) rather than a free code-line offset, so `j`/`k` always reveals the note it selects.
***8<***
---
{
  "v": 3,
  "cid": "bafyreibzk5yb3hvlny3z4o3nwdff3fpoy45o2okkbb2ii4lcp5pqnm6wxu",
  "sig": "3e76ff8988e52527f641925eb15841e5002a9517163b93c712458aba7a7cc50c4805b538e6c6ec18aee9bc4e46cafac77804953a7205832423d4cd13aef2d3f3",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-column-reflow"
  },
  "kind": "decision",
  "cites": [
    "bafyreiget3i4j2wow3ge2hsf5f2o5q6edifzkroakdtfitjlk37ttgfs4q"
  ],
  "rev": "223mtjggk3r3a",
  "seq": 7,
  "of": 9,
  "text_len": 1128,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgxJ7RxOrOtsxNHkXpdO7DxBoLlUXAUOZUTStW/zmYsuRmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx1Y29tbWVudC1jb2x1bW4tcmVmbG93aWFydGlmYWN0c4GhZkNvbW1pdHgoZTFkYmFjZmRjZDI4ZWRhMDk5MTlkY2ViZjBlMjc3NjAwZGFmNjJiNGl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABll7GQDbuQ=="
}
---

adversarial review of comment-column-reflow: SHIP. Independent Opus review traced reflow_rows exhaustively (same-line notes, adjacent-line reflow anchoring by original code-line index so no desync, start_line past code end via the trailing loop, empty code/notes/everything) with throwaway tests: exact, panic-free, one note_rows entry per note, no dropped notes. Scroll is top-anchored to the selected note's row, both columns skip(scroll) so they stay aligned; empty rows safe via saturating_sub; a note taller than the pane scrolls (REQ-2 permits). note_block/wrap_text char-safe (chars().count(), push('…')), note height bounded (<=5 rows). Narrow branch unchanged (comment_scroll not dead), strip (thread_lines + unresolvable list) rendered in both branches, markers kept, ←/→ nav intact, no new kan reads/writes (pure projection). Out-of-scope (tree-sitter, TUI authoring, full-thread-in-column, resizable split) absent. Minor non-blocking: wrap_text(_,0) latent-unreachable in the wide branch. Closed the reviewer's AC-2 note by asserting a wrapped body word. 81 tests, clippy -D warnings, fmt clean. Verdict: SHIP.
***8<***
---
{
  "v": 3,
  "cid": "bafyreigpab5ewgkzfhrmt4bpodhohhiponfn4dni72a46iwukmmcrvukbe",
  "sig": "810a71c0b9d2d7abdde855037128973b7e63a2f19c9861bc9d618fd902c7c4aa6d9decccefe16e8a3a1e7aca9b2518f8e4293b29be5d19f440c8afad0a386520",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-column-reflow"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mtjggkes2q",
  "seq": 8,
  "of": 9,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx1Y29tbWVudC1jb2x1bW4tcmVmbG93aWFydGlmYWN0c4GhZkNvbW1pdHgoZTFkYmFjZmRjZDI4ZWRhMDk5MTlkY2ViZjBlMjc3NjAwZGFmNjJiNGl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABll7GQVfrw=="
}
---
