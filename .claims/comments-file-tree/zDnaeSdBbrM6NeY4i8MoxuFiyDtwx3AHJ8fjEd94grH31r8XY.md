---
{
  "v": 3,
  "cid": "bafyreiaucefpa2yu2cp44lrqahrtkwkgmmg6wtg7v6brmux7ylqgep66rm",
  "sig": "432620cf2a27987555a60bb1b7186a9fe0f82cd7f18ff9b378e96c8e4116d03e0a4bf915f4504c78ed29f5bd95fe6f68e1c1006132d9280138e24b4092ee153c",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comments-file-tree"
  },
  "kind": "observation",
  "cites": [],
  "rev": "223muqnvdjvau",
  "seq": 0,
  "of": 14,
  "text_len": 192,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2FscmNvbW1lbnRzLWZpbGUtdHJlZWlhcnRpZmFjdHOBoWZDb21taXR4KDkxM2I3YTMyZGMxOWUwMGJjNzcxMGVmNjUzOWViMmM2NjU5M2E2ODJpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZatPaX6Mw="
}
---

design doc .design/comments-file-tree.md checked against the live design-doc schema: validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s) [doc 7729:28c2cef6b1ac1490]
***8<***
---
{
  "v": 3,
  "cid": "bafyreia2itqtfsekx6scb4nz5tj73hcfifsxtc6a5tlbwa2qr2rnqkbnl4",
  "sig": "1c8a36f2cd3c2a16c9262fb7db6d177e3ef78964601f670a762f3acd90733b964cdd3c72619f83d0b21f376cf4e05738bb727090e286fe571e861d7f76a1b16d",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comments-file-tree"
  },
  "kind": "plan",
  "cites": [
    "bafyreiaucefpa2yu2cp44lrqahrtkwkgmmg6wtg7v6brmux7ylqgep66rm"
  ],
  "rev": "223muqnve55de",
  "seq": 1,
  "of": 14,
  "text_len": 569,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4HYKlglAAFxEiAUEQrwaxTQn84uMAHjNVlGYw3rTN+vgxZS/8LgYj/ei2ZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZVNkQmJyTTZOZVk0aThNb3h1Rml5RHR3eDNBSEo4ZmpFZDk0Z3JIMzFyOFhZZWFnZW509mdzdWJqZWN0oWVMb2NhbHJjb21tZW50cy1maWxlLXRyZWVpYXJ0aWZhY3RzgaFmQ29tbWl0eCg5MTNiN2EzMmRjMTllMDBiYzc3MTBlZjY1MzllYjJjNjY1OTNhNjgyaXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWrT2oYy1"
}
---

comments-file-tree design (.design/comments-file-tree.md): Turn the Comments tab's "All files" list from a flat ~157-row list into a collapsible directory tree that starts folded, excludes kan's `.claims/` published tree, and badges (rather than dead-lists) symlinks. Also folds in the file-viewer review follow-ups: F2 (symlinks were dead rows) and F3 (a line that already has a comment could not receive a second). All page-side except a small additive `symlink` field on `GET /files`. [validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s)]
***8<***
---
{
  "v": 3,
  "cid": "bafyreidx5pwegkhpsqcxtsnflx7flyxc23e57ktzrl6ggi3gku25nheb7y",
  "sig": "d4b81da14811835cca5b04dcfd423969d6b3f8b10ee6a0ee3dd981bdbefb81c621afd93f04e7b8a68879af9d7ef2f588ea7f97da8aa0d8e8aac1359ecdcd874f",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comments-file-tree"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223muqnvee6lh",
  "seq": 2,
  "of": 14,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXhMRmVhdHVyZTogQ29tbWVudHMgZmlsZSBicm93c2VyIOKAlCBmb2xkZWQgZGlyZWN0b3J5IHRyZWUgKyByZXZpZXcgZm9sbG93LXVwc2xzdWJqZWN0X2tpbmRkSWRlYWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxyY29tbWVudHMtZmlsZS10cmVlaWFydGlmYWN0c4GhZkNvbW1pdHgoOTEzYjdhMzJkYzE5ZTAwYmM3NzEwZWY2NTM5ZWIyYzY2NTkzYTY4Mml3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABlq09qURvA=="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreifox5euv4nbh3bggfbdctx2etibaq2c4voog747yic63qlw77cmwq",
  "sig": "f9338584cbff9af6f9f5c3ba8efde64b07d5cea18e52add3bf8b4aad25aa3955608cab161d8c6b1fb476042bf2cb6015a42660005c40de947d5f525efb60a884",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comments-file-tree"
  },
  "kind": "decision",
  "cites": [
    "bafyreia2itqtfsekx6scb4nz5tj73hcfifsxtc6a5tlbwa2qr2rnqkbnl4"
  ],
  "rev": "223muqnvf3b64",
  "seq": 3,
  "of": 14,
  "text_len": 306,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgGkThMsiKv6Qg8bns0/2cRUFleYvA7NYbA1COotgoLV9mYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxyY29tbWVudHMtZmlsZS10cmVlaWFydGlmYWN0c4GhZkNvbW1pdHgoOTEzYjdhMzJkYzE5ZTAwYmM3NzEwZWY2NTM5ZWIyYzY2NTkzYTY4Mml3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABlq09rCcDA=="
}
---

RQ-1: Exclude `.claims/` (and `.cospan/`) from the browser, rather than folding them in as ordinary directories — they are kan's own published state and cospan's owned sidecar tree, not source files to browse or comment on. `GET /files` still returns them (honest endpoint); the browser filters them out.
***8<***
---
{
  "v": 3,
  "cid": "bafyreiasp27vx5k3zea3be6uilkg7a7uwpz7p7ibzmr3lzrolqkfwr6dcq",
  "sig": "e26099daa72de722561d01c465e38ef5067cfb89ae95fc1475351be26ad695635b28c21f434e521a638a47ae69232744b35dac44c6ae1d83326191e513b10fc8",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comments-file-tree"
  },
  "kind": "decision",
  "cites": [
    "bafyreia2itqtfsekx6scb4nz5tj73hcfifsxtc6a5tlbwa2qr2rnqkbnl4"
  ],
  "rev": "223muqnvfnvvh",
  "seq": 4,
  "of": 14,
  "text_len": 275,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgGkThMsiKv6Qg8bns0/2cRUFleYvA7NYbA1COotgoLV9mYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxyY29tbWVudHMtZmlsZS10cmVlaWFydGlmYWN0c4GhZkNvbW1pdHgoOTEzYjdhMzJkYzE5ZTAwYmM3NzEwZWY2NTM5ZWIyYzY2NTkzYTY4Mml3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABlq09rnu9w=="
}
---

RQ-2: Keep symlinks and badge them `[link]`, rather than skipping them server-side — the entry stays visible but is clearly marked and, on tap, says "symlinks aren't viewable" instead of surfacing the raw guard error. This needs an additive `symlink` field on `GET /files`.
***8<***
---
{
  "v": 3,
  "cid": "bafyreic73fwilad7rebt4dg5d55fpr2hoi4pgrzdno3z7j4lhsgqb2oyaq",
  "sig": "47d029486ef113e27e5a703a7f7abacceeba0a6a2763de70bf6be3450ddf67fa074899cbfab4fe85e66925bc4879473e07a77ca20711cfe935f23e9de1fe75e6",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comments-file-tree"
  },
  "kind": "decision",
  "cites": [
    "bafyreia2itqtfsekx6scb4nz5tj73hcfifsxtc6a5tlbwa2qr2rnqkbnl4"
  ],
  "rev": "223muqnvgc3it",
  "seq": 5,
  "of": 14,
  "text_len": 250,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgGkThMsiKv6Qg8bns0/2cRUFleYvA7NYbA1COotgoLV9mYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxyY29tbWVudHMtZmlsZS10cmVlaWFydGlmYWN0c4GhZkNvbW1pdHgoOTEzYjdhMzJkYzE5ZTAwYmM3NzEwZWY2NTM5ZWIyYzY2NTkzYTY4Mml3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABlq09sQFYA=="
}
---

RQ-3: The line number is the second-comment affordance — tapping `.clnum` always starts an add for that line (F3), while tapping the gutter/code of a commented line still opens its thread. One always-available target, no chooser or per-line button.
***8<***
---
{
  "v": 3,
  "cid": "bafyreia5vxvytrjjl2qfpkvrr66dou3ivxf2576yot44vqt4346nxkors4",
  "sig": "3adf74ce979e1c2d4f074b4e355e00f36f0f20216d0c8587c58ccdcfdb356c6015bd0cc2e8f77c0042795af72e7c1250354b30c9507a36e7a513a1b064933f4d",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comments-file-tree"
  },
  "kind": "result",
  "cites": [
    "bafyreia2itqtfsekx6scb4nz5tj73hcfifsxtc6a5tlbwa2qr2rnqkbnl4"
  ],
  "rev": "223muqo65c7bq",
  "seq": 6,
  "of": 14,
  "text_len": 1906,
  "content": "p2Rib2R5oWZSZXN1bHShZHRleHRgZWNpdGVzgdgqWCUAAXESIBpE4TLIir+kIPG57NP9nEVBZXmLwOzWGwNQjqLYKC1fZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2FscmNvbW1lbnRzLWZpbGUtdHJlZWlhcnRpZmFjdHOBoWZDb21taXR4KDkxM2I3YTMyZGMxOWUwMGJjNzcxMGVmNjUzOWViMmM2NjU5M2E2ODJpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZatQg0Emc="
}
---

generative-build complete for the Comments file-tree slice. Mostly page-side (src/web/index.html) plus one additive server field (src/server.rs get_files) and tests.

Code-change: (1) FOLDED TREE — renderFilesBrowser now builds a nested directory tree (buildFileTree) from the flat GET /files paths; directories render as collapsible .fdir rows starting folded, a session expandedDirs Set tracks open dirs, tap a dir toggles + re-draws, tap a file opens openFileViewer; file rows keep the commented-✎ mark and git-status marker. (2) FILTER — a non-empty filter shows a flat list of matching files; empty shows the folded tree. (3) .claims DE-NOISE — the browser excludes paths under .claims/ and .cospan/ (client-side filter; GET /files still returns them). (4) F2 — get_files adds an additive `symlink` bool per entry (symlink_metadata on repo.join(path); filetree::list untouched); the browser badges a symlink [link] and on tap shows "symlinks aren't viewable" in the detail pane instead of opening the guard error. (5) F3 — the viewer's line number (.clnum) is a distinct add target under caps.writes: tapping it stopPropagation + startAddAt(file,lineNo), so a line that already has a comment can take a new one; the gutter/code tap still opens the thread.

Evidence: cargo test 215 unit + 15 integration green (new index_html_wires_the_file_tree asserts buildFileTree/fdir/expandedDirs, the .claims+.cospan exclusion, the symlink flag read + badge + guarded tap, and the clnum add target; new files_endpoint_flags_symlinks integration test asserts GET /files carries symlink=false for a normal file and true for a committed symlink; updated the file-viewer test's brittle 60-char guard to a precise caps.writes-gated-row-tap assertion). clippy -D warnings clean; fmt clean. UNVERIFIED LAYER: the visual tree render (folding, indentation, badges, line-number add) needs an operator eyeball.
***8<***
---
{
  "v": 3,
  "cid": "bafyreie6lxwcukhrrelads7w4v3aahqbunrqy7v6v45lma2mi4p5o3leti",
  "sig": "b626eba94428a2487f939d29120e4a78a6d720f0a89d3caa7e8ae6f1d06563222bc93a776f7064e91917d1bda2d8272c37a13b4731e556a61f8aaca00be9d221",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comments-file-tree"
  },
  "kind": "decision",
  "cites": [
    "bafyreia2itqtfsekx6scb4nz5tj73hcfifsxtc6a5tlbwa2qr2rnqkbnl4"
  ],
  "rev": "223muqofwwto5",
  "seq": 7,
  "of": 14,
  "text_len": 506,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgGkThMsiKv6Qg8bns0/2cRUFleYvA7NYbA1COotgoLV9mYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxyY29tbWVudHMtZmlsZS10cmVlaWFydGlmYWN0c4GhZkNvbW1pdHgoOTEzYjdhMzJkYzE5ZTAwYmM3NzEwZWY2NTM5ZWIyYzY2NTkzYTY4Mml3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABlq1F85k2w=="
}
---

adversarial review of comments-file-tree: APPROVE — Cold Opus review: clean APPROVE. Folded tree + filter + .claims exclusion + symlink badge/guard + F3 line-number add all trace to real behavior; the two highest-risk claims (stream tick collapsing the tree; F3 double-add) disproven by tracing; endpoint honest (.claims still returned, filtered client-side, curl-confirmed); one surface, zero deps, filetree::list untouched; 215 unit + 15 integration + clippy + fmt green. Nits F-1/F-2/F-3 non-blocking.
***8<***
---
{
  "v": 3,
  "cid": "bafyreih55jvlgkpfaxqkahdrtqmv2htknj7r5rziqy7dtiyb5ivqkagakm",
  "sig": "844cb719820d14a5b2f1db68ef107475cee054c02e0273674e7584b9831c76b656bf236093e6f064bcbf973d8e488d88a1864a949199b884c8bc64f73a197a4b",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comments-file-tree"
  },
  "kind": "observation",
  "cites": [
    "bafyreie6lxwcukhrrelads7w4v3aahqbunrqy7v6v45lma2mi4p5o3leti"
  ],
  "rev": "223muqofxnlbq",
  "seq": 8,
  "of": 14,
  "text_len": 436,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgnl3sKijxiRYBy/bldgAeAaNjDH6+rzq2A0xHH9dtZJpmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxyY29tbWVudHMtZmlsZS10cmVlaWFydGlmYWN0c4GhZkNvbW1pdHgoOTEzYjdhMzJkYzE5ZTAwYmM3NzEwZWY2NTM5ZWIyYzY2NTkzYTY4Mml3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABlq1F9nEfA=="
}
---

Review finding F-1 (non-blocking, cost): get_files runs symlink_metadata on all ~169 files including the ~92 .claims/ rows the browser then discards client-side (~54% wasted stats per /files request). Bounded, localhost read API; the .claims exclusion is deliberately client-side to keep the endpoint honest, so moving the stat past the exclusion would couple the endpoint to the browser. Left as-is; revisit if /files ever grows large.
***8<***
---
{
  "v": 3,
  "cid": "bafyreifx4mgkenmzepblhdynhxa7dil6wah43h2guo2fb6jmmnn3swmocy",
  "sig": "87d4acb53212caec2b1b08e2be32cb644bbce343973f054e336ea8faf3f1f60d72879e408b7f1a71a9bd969601fb73e1cd92d7203964bedf834a75e922850430",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comments-file-tree"
  },
  "kind": "observation",
  "cites": [
    "bafyreie6lxwcukhrrelads7w4v3aahqbunrqy7v6v45lma2mi4p5o3leti"
  ],
  "rev": "223muqofyanz6",
  "seq": 9,
  "of": 14,
  "text_len": 319,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgnl3sKijxiRYBy/bldgAeAaNjDH6+rzq2A0xHH9dtZJpmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxyY29tbWVudHMtZmlsZS10cmVlaWFydGlmYWN0c4GhZkNvbW1pdHgoOTEzYjdhMzJkYzE5ZTAwYmM3NzEwZWY2NTM5ZWIyYzY2NTkzYTY4Mml3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABlq1F+NPcA=="
}
---

Review finding F-2 (non-blocking, nit): files within a tree node are not explicitly sorted; correctness rides on filetree::list returning paths 'sorted by path' (filetree.rs). True today; if that upstream contract changed, tree file order would silently follow. A one-line comment or a client-side sort would harden it.
***8<***
---
{
  "v": 3,
  "cid": "bafyreih7p2j5sdngwkg6nnpmthr6mpkj53dnpwauncnskpwsjwbydja474",
  "sig": "114d3836c83359cb89e8d6a753907852898930d107cca106f69e524e197cb9454fb8454970254cb036b821895820ac86b907fa9394bc3e5910db1eced1b2e64e",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comments-file-tree"
  },
  "kind": "observation",
  "cites": [
    "bafyreie6lxwcukhrrelads7w4v3aahqbunrqy7v6v45lma2mi4p5o3leti"
  ],
  "rev": "223muqofywnd3",
  "seq": 10,
  "of": 14,
  "text_len": 351,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgnl3sKijxiRYBy/bldgAeAaNjDH6+rzq2A0xHH9dtZJpmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxyY29tbWVudHMtZmlsZS10cmVlaWFydGlmYWN0c4GhZkNvbW1pdHgoOTEzYjdhMzJkYzE5ZTAwYmM3NzEwZWY2NTM5ZWIyYzY2NTkzYTY4Mml3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABlq1F+5MrA=="
}
---

Review finding F-3 (non-blocking, intended): expandedDirs (the file-tree open set) never resets for the page lifetime — a deep expansion persists across tab switches with no collapse-all. Consistent with the 'session Set' design; single-child-chain collapsing and an ignore-config were scoped out. Noted for a possible later collapse-all affordance.
***8<***
---
{
  "v": 3,
  "cid": "bafyreifiatobvgjgwululgxm7vqeyrhl6fbgh72ibarjvllfrttnrdapdu",
  "sig": "5ea51bf1c9373126760925982b96a5e7bb16d3a6ca5e5752c5db3bcb308d474e4acd70e7e98e74306e382470363e036d7307314bc4c420bc060b13761c192ad4",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comments-file-tree"
  },
  "kind": "observation",
  "cites": [
    "bafyreie6lxwcukhrrelads7w4v3aahqbunrqy7v6v45lma2mi4p5o3leti"
  ],
  "rev": "223muqy3qcq42",
  "seq": 11,
  "of": 14,
  "text_len": 662,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgnl3sKijxiRYBy/bldgAeAaNjDH6+rzq2A0xHH9dtZJpmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxyY29tbWVudHMtZmlsZS10cmVlaWFydGlmYWN0c4GhZkNvbW1pdHgoOTEzYjdhMzJkYzE5ZTAwYmM3NzEwZWY2NTM5ZWIyYzY2NTkzYTY4Mml3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABlq3g2RXwA=="
}
---

Eyeball fix (file-tree idiom): operator found the folder/file distinction and hierarchy weak in the flat paddingLeft tree. Reworked to the chosen idiom: an inline tree with icons + guide lines — folder rows render 📁 (closed) / 📂 (open) + bold shaded name + count; file rows render 📄 + plain name; and nesting now uses nested .ftree-children containers whose left border draws a per-level depth guide, replacing the paddingLeft indentation. .ffile lost its card look (plain rows) so folders (shaded) read distinct from files. Tree behavior (buildFileTree, expandedDirs, filter flat-list, symlink badge, click-to-open) unchanged. Tests green, fmt clean.
***8<***
---
{
  "v": 3,
  "cid": "bafyreihlopqzwarsg3eveojei5josiim3cik6sdntbpsosktubgdaw7nce",
  "sig": "61757c45abb16b636c0037b3388f0ee40d568b5c598882acfa1ea3e9f7929487681344156dbbcc73d3e834d40bab3e0733844834fbc477122b6de4ef9a688717",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comments-file-tree"
  },
  "kind": "observation",
  "cites": [
    "bafyreie6lxwcukhrrelads7w4v3aahqbunrqy7v6v45lma2mi4p5o3leti"
  ],
  "rev": "223muqysholac",
  "seq": 12,
  "of": 14,
  "text_len": 396,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgnl3sKijxiRYBy/bldgAeAaNjDH6+rzq2A0xHH9dtZJpmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxyY29tbWVudHMtZmlsZS10cmVlaWFydGlmYWN0c4GhZkNvbW1pdHgoOTEzYjdhMzJkYzE5ZTAwYmM3NzEwZWY2NTM5ZWIyYzY2NTkzYTY4Mml3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABlq3sNpCTw=="
}
---

Eyeball fix (file-tree icons): operator found color emoji (📁/📂/📄) garish. Replaced with monochrome inline SVG icons (a svgIcon() helper drawing a folder and a file outline via currentColor, so they follow the theme) plus a faint unicode chevron (▸/▾) for open/closed; folder icon tinted var(--accent), file icon var(--faint). No dependency added (inline SVG). Tests green, fmt clean.
***8<***
---
{
  "v": 3,
  "cid": "bafyreia6aylyz3sypwt5r5yvzoryacpfo56figasdsmk2uzhk55jvndhm4",
  "sig": "99cd02781ddd2d75da715203e2fa73aeb6c9bc73d0e6c6b5581923a27ca682991b27271d095fd8fcca7dea02903c54a9c4368867783f04289dc63c6974ec78cc",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comments-file-tree"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223murdfohiu4",
  "seq": 13,
  "of": 14,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxyY29tbWVudHMtZmlsZS10cmVlaWFydGlmYWN0c4GhZkNvbW1pdHgoOGM1NTA5ZDlhNDkyNWU3YWVjOTkyOGU5NzM4YzI5YWFlMWZiN2ViZWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABlq6V0a6WQ=="
}
---
