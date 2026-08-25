---
{
  "v": 3,
  "cid": "bafyreidjhdluamn7hjsgzge6qc2aoy4x64feg6oez2fmgaci5sr6oqkq2y",
  "sig": "7dfdbe552cb1160c546968dc5962f9f5490ea219184264a9814870a510725ee87a850a10e553cf854c7deed702e9cd0b32b959d404d420758c2bfeb39f003c47",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-editor-diffs"
  },
  "kind": "observation",
  "cites": [],
  "rev": "223mtuqrtcvxw",
  "seq": 0,
  "of": 10,
  "text_len": 194,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2FsdGNvbW1lbnQtZWRpdG9yLWRpZmZzaWFydGlmYWN0c4GhZkNvbW1pdHgoZWQ1NjQ5YTU3Yjc0MTEyOGVkZTI2ZDZiNjI2ZWU2ODU4MTQwNmYyZWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABlnVr5RvSA=="
}
---

design doc .design/comment-editor-diffs.md checked against the live design-doc schema: validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s) [doc 8368:d00a4d5d4a753459]
***8<***
---
{
  "v": 3,
  "cid": "bafyreidsn3ul75z5vm5ufouz5mtqh4dienaj2fmaef344m5svsgcwpmitq",
  "sig": "a4c4a183779b0eb703e7cb06d30a458d97a446d899622de0597fb00cb543ab3866a6b35f6e55b33fec845f257057487e45c7983f2f60a4d69f676370282243c2",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-editor-diffs"
  },
  "kind": "plan",
  "cites": [
    "bafyreidjhdluamn7hjsgzge6qc2aoy4x64feg6oez2fmgaci5sr6oqkq2y"
  ],
  "rev": "223mtuqrtoxrw",
  "seq": 1,
  "of": 10,
  "text_len": 850,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4HYKlglAAFxEiBpONdAMb86ZGyYnoC0B2OX9wpDecTOisMASOyj50FQ1mZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZVNkQmJyTTZOZVk0aThNb3h1Rml5RHR3eDNBSEo4ZmpFZDk0Z3JIMzFyOFhZZWFnZW509mdzdWJqZWN0oWVMb2NhbHRjb21tZW50LWVkaXRvci1kaWZmc2lhcnRpZmFjdHOBoWZDb21taXR4KGVkNTY0OWE1N2I3NDExMjhlZGUyNmQ2YjYyNmVlNjg1ODE0MDZmMmVpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZZ1a+adoc="
}
---

comment-editor-diffs design (.design/comment-editor-diffs.md): Show the working-tree diff (vs `HEAD`) inline in the Comments-tab code pane as agents rewrite files: added and changed lines carry a diff **sign** (`+`/`~`) and a subtle tint, and deleted content is marked at its boundary. Slice B of the Comments-tab redesign, split out of `comment-editor-layout` (Slice A). It keeps the current-file-line-indexed render model Slice A established — no interleaved removed lines — so the comment gutter, band, note reflow, sticky scroll, and the pinned unresolvable band all keep working. Serves `telos/comment-roundtrip` (the review surface shows what changed) and `telos/poll-dont-subscribe` (the diff is computed on the fold loop, mtime-gated, never per keystroke). [validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s)]
***8<***
---
{
  "v": 3,
  "cid": "bafyreieg4em7325ukvd7uussgi67t53nvbvd5zzwajgi5mvnsfkdxdzusm",
  "sig": "0eeb4b7c058d8689d221d43d07d1893e17907a5ce57c134cc361f814eb64a8870668b6616f21bc2e71de63916d0da502d21d69133887a8d1a6de98e7918854ba",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-editor-diffs"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223mtuqrtsvvm",
  "seq": 2,
  "of": 10,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXhBRmVhdHVyZTogTGl2ZSBnaXQgd29ya2luZy10cmVlIGRpZmZzIGluIHRoZSBlZGl0b3IgcGFuZSAoU2xpY2UgQilsc3ViamVjdF9raW5kZElkZWFlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2FsdGNvbW1lbnQtZWRpdG9yLWRpZmZzaWFydGlmYWN0c4GhZkNvbW1pdHgoZWQ1NjQ5YTU3Yjc0MTEyOGVkZTI2ZDZiNjI2ZWU2ODU4MTQwNmYyZWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABlnVr5xvAA=="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreifisgnlewvsb4hhwwl5ntbackvp5z6vbg7y3kmbysptcdjtq5lb74",
  "sig": "a9ab3b785f213e22fc62c2d0946c0440018f6abb4b7de5940e95b30d477b05b858e2d7dbd7ee1f4df60402bd1eb7dab087fb9aa84be0da912e0a7ac8e6a7605d",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-editor-diffs"
  },
  "kind": "decision",
  "cites": [
    "bafyreidsn3ul75z5vm5ufouz5mtqh4dienaj2fmaef344m5svsgcwpmitq"
  ],
  "rev": "223mtuqru6q3h",
  "seq": 3,
  "of": 10,
  "text_len": 426,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgcm7ov/c9qztCupnrJwPwaCNAnRWAIXfOM7KsjCs9iJxmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx0Y29tbWVudC1lZGl0b3ItZGlmZnNpYXJ0aWZhY3RzgaFmQ29tbWl0eChlZDU2NDlhNTdiNzQxMTI4ZWRlMjZkNmI2MjZlZTY4NTgxNDA2ZjJlaXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWdWvole1"
}
---

RQ-1: Deletions are shown by marking changed/added current-file lines (sign + tint) and a boundary marker where content was removed — no interleaved removed-line content and no inserted rows, because every downstream index (comment anchoring, reflow, sticky scroll, the pinned band) is keyed on the current-file line and an inserted row would shift them all. A full interleaved diff was rejected as fighting Slice A's model.
***8<***
---
{
  "v": 3,
  "cid": "bafyreifebaab5wrffx3bp4n6adqs2yz7xpd6g2r5sumejpr3xeuvplwf44",
  "sig": "da3ffc51159dfc8a90338d1d340d04fdeef7d799aa618036a2aa091d95e18d205a60f2b20b506d42799064929c13ff4d35349f84b692fa10b16ca20e309e3855",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-editor-diffs"
  },
  "kind": "decision",
  "cites": [
    "bafyreidsn3ul75z5vm5ufouz5mtqh4dienaj2fmaef344m5svsgcwpmitq"
  ],
  "rev": "223mtuqrukoii",
  "seq": 4,
  "of": 10,
  "text_len": 164,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgcm7ov/c9qztCupnrJwPwaCNAnRWAIXfOM7KsjCs9iJxmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx0Y29tbWVudC1lZGl0b3ItZGlmZnNpYXJ0aWZhY3RzgaFmQ29tbWl0eChlZDU2NDlhNTdiNzQxMTI4ZWRlMjZkNmI2MjZlZTY4NTgxNDA2ZjJlaXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWdWvqFFY"
}
---

RQ-2: The diff is toggleable with `D`, default on — shown live as agents rewrite (the section-(2) vision), but hideable for a clean read of a heavily-edited file.
***8<***
---
{
  "v": 3,
  "cid": "bafyreiectpyivcfa2cqxz45dcjjvjykfbrxzchiqjoou3wbvezbw6nl55a",
  "sig": "74b61d420635fb9ec839edfc84b2e209245f997c23fc94b2cc8d70e8dda974df201721830a869e4839dcd2b1053ffe8196d493bab1d05ce0995da87c6e2871f1",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-editor-diffs"
  },
  "kind": "decision",
  "cites": [
    "bafyreidsn3ul75z5vm5ufouz5mtqh4dienaj2fmaef344m5svsgcwpmitq"
  ],
  "rev": "223mtuqruwoko",
  "seq": 5,
  "of": 10,
  "text_len": 207,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgcm7ov/c9qztCupnrJwPwaCNAnRWAIXfOM7KsjCs9iJxmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx0Y29tbWVudC1lZGl0b3ItZGlmZnNpYXJ0aWZhY3RzgaFmQ29tbWl0eChlZDU2NDlhNTdiNzQxMTI4ZWRlMjZkNmI2MjZlZTY4NTgxNDA2ZjJlaXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWdWvrlGg"
}
---

RQ-3: When a line is both changed and comment-covered, the comment band wins the row background and the diff shows via the sign column (plus a tint that yields to the band), so neither layer hides the other.
***8<***
---
{
  "v": 3,
  "cid": "bafyreifntunlt2gebq7x4hnh5bzw7sgvww6pnoqofksr2xcxzhgaa4zn3e",
  "sig": "d0ad5387f91d3f1bd008e0d4d722b2839ec12437b7bc4c107f6f1d25e07db49d43b3a501e5755084f49acd8c5730c889f337d8b9cfc718ca8bff2255013a3852",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-editor-diffs"
  },
  "kind": "result",
  "cites": [],
  "rev": "223mtursoekrm",
  "seq": 6,
  "of": 10,
  "text_len": 1273,
  "content": "p2Rib2R5oWZSZXN1bHShZHRleHRgZWNpdGVzgGZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZVNkQmJyTTZOZVk0aThNb3h1Rml5RHR3eDNBSEo4ZmpFZDk0Z3JIMzFyOFhZZWFnZW509mdzdWJqZWN0oWVMb2NhbHRjb21tZW50LWVkaXRvci1kaWZmc2lhcnRpZmFjdHOBoWZDb21taXR4KDIzMDM2YWU5MDc3Y2VmOWUzY2E0NjM2ZGM0Yzg5YTk1ZTBmNTA0NTRpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZZ1fFFPyE="
}
---

Adversarial review of comment-editor-diffs (Slice B): BLOCK then fixed. An independent hostile Opus reviewer found one confirmed correctness blocker: the diff parser guarded +++/--- as file headers on EVERY line, so a hunk-body line whose CONTENT starts with ++ or -- (a SQL -- comment, a YAML ---, ++i, a patch file) was mis-parsed, dropping the line and cascading an off-by-one across the rest of the hunk. Fixed in 23036ae: dispatch on the leading byte only INSIDE a hunk (in_hunk tracked after the first @@), with a diff --git reset for multi-file diffs. The reviewer also flagged the git->compute->render path as untested (AC-3 only faked with a hand-set FileDiff); added a real-git end-to-end test of compute() plus a regression test for ++/-- content lines. Everything else was attacked and held: line-index alignment with styled_upto, the sign span not disturbing fill_line_bg width or reflow/sticky/pinned-band indices, tint yielding to the comment band, the D toggle and clean compose view, and parser robustness (interleaved -/+, no-count headers, CRLF, No-newline). 190 tests, clippy -D warnings clean, fmt clean. Pending: an operator live-TTY eyeball of the visual coherence (sign column + tint + band; the deletion boundary glyph), and a re-review of the fix.
***8<***
---
{
  "v": 3,
  "cid": "bafyreiauafzcvmsa2l3efu3zs4zr3ff6q6cy2qg5m4a4kmc2tuzo4bgqqe",
  "sig": "3d319314bfd68f6ce3f6e10dac2366b46dd7272509733e5cd09259c9d838af445d3276f9e64a695f514a7d1d21f23f2f25fc805de0f01c79a26e86a621c8a33b",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-editor-diffs"
  },
  "kind": "result",
  "cites": [],
  "rev": "223mtvztmoild",
  "seq": 7,
  "of": 10,
  "text_len": 625,
  "content": "p2Rib2R5oWZSZXN1bHShZHRleHRgZWNpdGVzgGZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZVNkQmJyTTZOZVk0aThNb3h1Rml5RHR3eDNBSEo4ZmpFZDk0Z3JIMzFyOFhZZWFnZW509mdzdWJqZWN0oWVMb2NhbHRjb21tZW50LWVkaXRvci1kaWZmc2lhcnRpZmFjdHOBoWZDb21taXR4KDk5ZTVkOGI2NTM1MTcyODdkZGU0OGY3ZjIyNjE1ZWFlMzU2MzkyNTFpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZZ3/MqObw="
}
---

Re-review of the post-BLOCK delta (parser fix + visual refinements + hole fix): the parser blocker and all deletion-gutter/all-views refinements were CONFIRMED correct, but the reviewer found a NEW regression from the hole fix (3408a52): the note pane is g rows shorter than the code pane when an unresolvable comment exists, yet the sticky-scroll view_h was measured from the full-height code pane, so scrolling down to an off-screen comment clipped its note into the band. Fixed: measure the note pane and keep the whole selected note block visible (start up / end down). Regression test added. 192 tests, clippy/fmt clean.
***8<***
---
{
  "v": 3,
  "cid": "bafyreidws42p2qbjp2bcugen6v4mqx32msw2qbln64o6ttlibnmo2jyekm",
  "sig": "d1e3fe3b2995ead3f979cd5f6e1abde48e6d36d2321086ea2737599275a7424d5c93af064dbce109fa75c62ac9e0cc70e0c0c7f508eb9b045e073d811ccf224a",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-editor-diffs"
  },
  "kind": "result",
  "cites": [],
  "rev": "223mtvzzdlqva",
  "seq": 8,
  "of": 10,
  "text_len": 874,
  "content": "p2Rib2R5oWZSZXN1bHShZHRleHRgZWNpdGVzgGZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZVNkQmJyTTZOZVk0aThNb3h1Rml5RHR3eDNBSEo4ZmpFZDk0Z3JIMzFyOFhZZWFnZW509mdzdWJqZWN0oWVMb2NhbHRjb21tZW50LWVkaXRvci1kaWZmc2lhcnRpZmFjdHOBoWZDb21taXR4KDk5ZTVkOGI2NTM1MTcyODdkZGU0OGY3ZjIyNjE1ZWFlMzU2MzkyNTFpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZZ3/6Y2vs="
}
---

Final re-review of the sticky-scroll regression fix (99e5d8b): SHIP. An independent hostile Opus reviewer traced every boundary of the new scroll logic (note fully visible / start above / end below / note taller than view_h / view_h==0 / block_h 0-1 / unresolvable selected) and found no underflow or panic; confirmed view_h now measures the shorter note pane, the whole selected note block stays visible (start up / end down), the common no-band case is unchanged (no snap), sticky_top is still used by the pick-line picker (not dead), and the regression test provably FAILS on the pre-fix source and passes after. 192 tests, clippy -D warnings clean, fmt clean. Slice B is fully reviewed end to end: original parser BLOCK fixed, then a scroll regression from the hole fix found and fixed, all deletion-gutter/all-views visual refinements confirmed, operator eyeball clean.
***8<***
---
{
  "v": 3,
  "cid": "bafyreifnol4xyamco3cbqi2p6fq77tpwkdu5nh5ertsgwwusxq4yofalpi",
  "sig": "71cb6a6a4dcedbcb7458c17714d737e6986a901c53a2e11e1e7c266f73d9bd0d2fe5cb094d4c2e670a620796eaee2938af5125238eeb07a1670d6d0b20aa6bec",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-editor-diffs"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mtwooohfg4",
  "seq": 9,
  "of": 10,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx0Y29tbWVudC1lZGl0b3ItZGlmZnNpYXJ0aWZhY3RzgaFmQ29tbWl0eCgwYjQzOTU2NmViNjk4ZTY5ZTVkNjNlNTY3YWZmNjg5MmQwNGIzZGQ1aXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWeUpRq0W"
}
---
