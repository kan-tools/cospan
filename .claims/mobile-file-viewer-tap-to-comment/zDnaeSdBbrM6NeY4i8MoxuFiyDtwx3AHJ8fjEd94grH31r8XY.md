---
{
  "v": 3,
  "cid": "bafyreibdkavlfxcxybnn4fvj5cksjqzdobnv4bcrdmwu7yzstlwoog4cvm",
  "sig": "a808c5bf2d02f5c61eac561e61a52f73a4ce0e31f94ec1612669bdaac18b179a74a34f8fda09dfb519961a34c7624c46fb4a561f2648ab79f89e639ecd830dec",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "mobile-file-viewer-tap-to-comment"
  },
  "kind": "observation",
  "cites": [],
  "rev": "223mugvtmkwan",
  "seq": 0,
  "of": 14,
  "text_len": 208,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2FseCFtb2JpbGUtZmlsZS12aWV3ZXItdGFwLXRvLWNvbW1lbnRpYXJ0aWZhY3RzgaFmQ29tbWl0eCg2ZTliYzg4ZTRjMmJjZDk3YmRlNTBkOTJjNDQ3MWM4NDFhMzA4ZTRhaXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWmbzKHBg"
}
---

design doc .design/mobile-file-viewer-tap-to-comment.md checked against the live design-doc schema: validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s) [doc 11604:520a568095bc58c7]
***8<***
---
{
  "v": 3,
  "cid": "bafyreid4veqv4pwhlox2ob77kpjd5bd33mh3gtvztoa6arytanicjoyf4e",
  "sig": "136fee5ea8bca64d7c4bd27ebd96b49302453e4c3d065f63aaa465b239215f365df895e1f4ce74ceabce3d99afc11edb1b75eba939be7c5036062936564d5a8f",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "mobile-file-viewer-tap-to-comment"
  },
  "kind": "plan",
  "cites": [
    "bafyreibdkavlfxcxybnn4fvj5cksjqzdobnv4bcrdmwu7yzstlwoog4cvm"
  ],
  "rev": "223mugvtn37gy",
  "seq": 1,
  "of": 14,
  "text_len": 736,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4HYKlglAAFxEiAjUCqy3FfAWt4WqeiVJMMjcFteBFEbLU/jMprs5xuCq2ZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZVNkQmJyTTZOZVk0aThNb3h1Rml5RHR3eDNBSEo4ZmpFZDk0Z3JIMzFyOFhZZWFnZW509mdzdWJqZWN0oWVMb2NhbHghbW9iaWxlLWZpbGUtdmlld2VyLXRhcC10by1jb21tZW50aWFydGlmYWN0c4GhZkNvbW1pdHgoNmU5YmM4OGU0YzJiY2Q5N2JkZTUwZDkyYzQ0NzFjODQxYTMwOGU0YWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABlpm8zCVKw=="
}
---

mobile-file-viewer-tap-to-comment design (.design/mobile-file-viewer-tap-to-comment.md): Close the on-device comment-creation dead-end by giving the phone a way to reach *any* file, not only files that already have a comment. Add a read-only file browser and a syntax-highlighted file viewer over `cospan serve`, and let a tap on a line open the add-comment box pre-anchored to that line. This makes the doc-comment round trip start from the phone: browse a file with highlighting → tap a line → write the first comment (`telos/comment-roundtrip`), while reads stay observation and writes stay opt-in sidecar-only (`telos/observe-now-control-later`). [validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s)]
***8<***
---
{
  "v": 3,
  "cid": "bafyreibqlsvi4omuebigd3vccacmtqijlmfgtuoj3tmmkugnd3fqtidiw4",
  "sig": "85df0953facf22e0fdaf6e1c21f75c3f2a9945a3c5c38a2b5d364de39edc90002d66c8717907138d0ae7a745dec143580eb3c44c7f944f27835ff3fda64c1519",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "mobile-file-viewer-tap-to-comment"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223mugvtnakqv",
  "seq": 2,
  "of": 14,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXg5RmVhdHVyZTogTW9iaWxlIGZpbGUgYnJvd3NlciArIHZpZXdlciB3aXRoIHRhcC10by1jb21tZW50bHN1YmplY3Rfa2luZGRJZGVhZWNpdGVzgGZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZVNkQmJyTTZOZVk0aThNb3h1Rml5RHR3eDNBSEo4ZmpFZDk0Z3JIMzFyOFhZZWFnZW509mdzdWJqZWN0oWVMb2NhbHghbW9iaWxlLWZpbGUtdmlld2VyLXRhcC10by1jb21tZW50aWFydGlmYWN0c4GhZkNvbW1pdHgoNmU5YmM4OGU0YzJiY2Q5N2JkZTUwZDkyYzQ0NzFjODQxYTMwOGU0YWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABlpm8zNCSg=="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreiet7lhyj77pc46poii3btgexekdmkcl3soyxplyhlrlimer6e45u4",
  "sig": "c484ccdc9be37725735daa7706d5a878c269780ffdd3cff4966b152df7cbe08f7c8c838aabf037465355c4d27eb4afe3189ce6a015071e8a9068c99b3b0d0d4b",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "mobile-file-viewer-tap-to-comment"
  },
  "kind": "decision",
  "cites": [
    "bafyreid4veqv4pwhlox2ob77kpjd5bd33mh3gtvztoa6arytanicjoyf4e"
  ],
  "rev": "223mugvtnqdzv",
  "seq": 3,
  "of": 14,
  "text_len": 354,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgfKkhXj7HW6+nB/9T0j6Ee9sPs065m4HgRxMDUCS7BeFmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx4IW1vYmlsZS1maWxlLXZpZXdlci10YXAtdG8tY29tbWVudGlhcnRpZmFjdHOBoWZDb21taXR4KDZlOWJjODhlNGMyYmNkOTdiZGU1MGQ5MmM0NDcxYzg0MWEzMDhlNGFpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZaZvM7J4s="
}
---

RQ-1: Show file content and let the user tap a line, rather than a file-level comment defaulting to line 1 — the first comment anchors to the line the user chooses, which is what `telos/comment-roundtrip` ("browses any file with syntax highlighting and adds … comments") describes. This is the larger of the two scopes considered, taken deliberately.
***8<***
---
{
  "v": 3,
  "cid": "bafyreig3zogpzrfjbnxbczxbzhy7zjqev53uemulv2wbmolzby4nww3kpm",
  "sig": "4036f3eed6ca4ba177c8b32318950cdd3fcd1570f811af55a53fb008565178895d081f195d8d3c7bc0f743bcd080e5bf6e348cc9e81fa80b69d5c422c715d691",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "mobile-file-viewer-tap-to-comment"
  },
  "kind": "decision",
  "cites": [
    "bafyreid4veqv4pwhlox2ob77kpjd5bd33mh3gtvztoa6arytanicjoyf4e"
  ],
  "rev": "223mugvtoafbv",
  "seq": 4,
  "of": 14,
  "text_len": 195,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgfKkhXj7HW6+nB/9T0j6Ee9sPs065m4HgRxMDUCS7BeFmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx4IW1vYmlsZS1maWxlLXZpZXdlci10YXAtdG8tY29tbWVudGlhcnRpZmFjdHOBoWZDb21taXR4KDZlOWJjODhlNGMyYmNkOTdiZGU1MGQ5MmM0NDcxYzg0MWEzMDhlNGFpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZaZvNDLIY="
}
---

RQ-2: The file browser is always visible (a read affordance), not gated on `--allow-writes`; only the add-a-comment action is gated. Browsing files is observation, so it belongs to the read tier.
***8<***
---
{
  "v": 3,
  "cid": "bafyreid772stxi6asojyb5kslmctfp2amd2if44guvas753762yr2buzx4",
  "sig": "0491fa600aa7a4fdd27113a0ecbd63ef714970f89cd0558d34eb94f29c1a9ee2533708c4721bdb97c158ca262aa72d8ccf942e384b20d7028149b6dbb20fe400",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "mobile-file-viewer-tap-to-comment"
  },
  "kind": "decision",
  "cites": [
    "bafyreid4veqv4pwhlox2ob77kpjd5bd33mh3gtvztoa6arytanicjoyf4e"
  ],
  "rev": "223mugvtoqmks",
  "seq": 5,
  "of": 14,
  "text_len": 247,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgfKkhXj7HW6+nB/9T0j6Ee9sPs065m4HgRxMDUCS7BeFmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx4IW1vYmlsZS1maWxlLXZpZXdlci10YXAtdG8tY29tbWVudGlhcnRpZmFjdHOBoWZDb21taXR4KDZlOWJjODhlNGMyYmNkOTdiZGU1MGQ5MmM0NDcxYzg0MWEzMDhlNGFpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZaZvNLSac="
}
---

RQ-3: Highlight server-side and ship a dependency-free page. Reuse the existing `syntect` highlighter via a hex projection rather than adding a client-side JS highlighter, keeping the page one embedded `include_str!` document (`telos/disposable`).
***8<***
---
{
  "v": 3,
  "cid": "bafyreig7575c427wc4rmg4gv52xifpoq4dkxka4eagyzwscmzslk6pusja",
  "sig": "5ead0d976e9a7b39c068d993a95d923d0dfc47976d3320d9bf09279fbbebb7ed0d979a7d4c72629ba63e873261112254bb24db04b9eb9a65aecd1666b354ac44",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "mobile-file-viewer-tap-to-comment"
  },
  "kind": "decision",
  "cites": [
    "bafyreid4veqv4pwhlox2ob77kpjd5bd33mh3gtvztoa6arytanicjoyf4e"
  ],
  "rev": "223mugvtpbd2r",
  "seq": 6,
  "of": 14,
  "text_len": 190,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgfKkhXj7HW6+nB/9T0j6Ee9sPs065m4HgRxMDUCS7BeFmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx4IW1vYmlsZS1maWxlLXZpZXdlci10YXAtdG8tY29tbWVudGlhcnRpZmFjdHOBoWZDb21taXR4KDZlOWJjODhlNGMyYmNkOTdiZGU1MGQ5MmM0NDcxYzg0MWEzMDhlNGFpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZaZvNTo6Q="
}
---

RQ-4: Client-side filter over the whole `GET /files` list, not a server-side `?q=` search — the browsable list is cheap to send whole, and a substring box avoids per-keystroke round-trips.
***8<***
---
{
  "v": 3,
  "cid": "bafyreicotcvvtrvmfpubfmdnrytw36jenblcidpjpxebiy2jqtcrwrokz4",
  "sig": "a8831e2f885b60753fdbe5d15eb49a1a0a8ac090a04bc6857992a233c5c90c117b9d70e416656a939415902cb9c82358d9229cbd5c472281f3e630eeb12bf620",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "mobile-file-viewer-tap-to-comment"
  },
  "kind": "decision",
  "cites": [
    "bafyreid4veqv4pwhlox2ob77kpjd5bd33mh3gtvztoa6arytanicjoyf4e"
  ],
  "rev": "223mugvtprmhf",
  "seq": 7,
  "of": 14,
  "text_len": 334,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgfKkhXj7HW6+nB/9T0j6Ee9sPs065m4HgRxMDUCS7BeFmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx4IW1vYmlsZS1maWxlLXZpZXdlci10YXAtdG8tY29tbWVudGlhcnRpZmFjdHOBoWZDb21taXR4KDZlOWJjODhlNGMyYmNkOTdiZGU1MGQ5MmM0NDcxYzg0MWEzMDhlNGFpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZaZvNbyTk="
}
---

RQ-5: Fold the browser into the Comments tab (an `all files | commented` toggle) rather than adding a sixth nav tab or burying it behind a write-mode entry. The phone keeps five tabs and gains one file surface where the two lists (all files, files-with-comments) live together; the `commented` default keeps today's landing unchanged.
***8<***
---
{
  "v": 3,
  "cid": "bafyreifcju56ukyzgaxkbe6orwflxgy4wk7haau3t36aq5l7vgdugn6pbq",
  "sig": "2b03614c123370f1eba5861c8d45b1801a117cdbcb055361f4b65e15d0efe5c65cf4a5c445a4698e6a07d60b8c6acd12c8cd9c42b677fbb01bcecacb1daff85b",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "mobile-file-viewer-tap-to-comment"
  },
  "kind": "result",
  "cites": [],
  "rev": "223mugwk7udoz",
  "seq": 8,
  "of": 14,
  "text_len": 1228,
  "content": "p2Rib2R5oWZSZXN1bHShZHRleHRgZWNpdGVzgGZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZVNkQmJyTTZOZVk0aThNb3h1Rml5RHR3eDNBSEo4ZmpFZDk0Z3JIMzFyOFhZZWFnZW509mdzdWJqZWN0oWVMb2NhbHghbW9iaWxlLWZpbGUtdmlld2VyLXRhcC10by1jb21tZW50aWFydGlmYWN0c4GhZkNvbW1pdHgoNmU5YmM4OGU0YzJiY2Q5N2JkZTUwZDkyYzQ0NzFjODQxYTMwOGU0YWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABlpnIF0mHA=="
}
---

generative-build complete for the mobile file-browser + viewer with tap-to-comment slice. Reads-only surface plus the existing opt-in write path; closes the on-device first-comment dead-end.

Code-change: highlight::styled_web (hex projection over the existing syntect pipeline; page stays dependency-free); mcp::file_view + FILE_VIEW_MAX_LINES (guarded, capped, {path,lines,truncated,total}); server GET /files (filetree::list projection) and GET /file?path= (both read-only, behind auth); web/index.html Comments tab folded into an all-files|commented toggle with a client-side filter, a highlighted file viewer (line numbers + comment gutter + exhaustive reachable comment list), and tap-a-line add gated on caps.writes.

Evidence: cargo test 210 unit + 14 integration green (new: styled_web hex AC-3, file_view endpoint AC-2, /files auth AC-1, first-comment-on-uncommented-file dead-end regression AC-4, index_html_wires_the_file_viewer AC-5); cargo fmt --check clean; cargo clippy --all-targets -D warnings clean; real serve smoke confirmed /files, /file (hex + truncation), guard, capabilities. UNVERIFIED layer: the in-browser render of the new viewer/toggle (needs a real phone/browser eyeball), as with prior UI slices.
***8<***
---
{
  "v": 3,
  "cid": "bafyreid4ol7cwpe7hnq7y5m3mvnatkuivqlst224lqwbbjplr2e4xhoz54",
  "sig": "176ec951518f4e3cfd5d4ad13fac369700476cd565fec2051f537e1f00d4d7af7389481cb4bc8a8d921cfe4343b80cbd773da0d0204da328033be4cf44864593",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "mobile-file-viewer-tap-to-comment"
  },
  "kind": "decision",
  "cites": [
    "bafyreid4veqv4pwhlox2ob77kpjd5bd33mh3gtvztoa6arytanicjoyf4e"
  ],
  "rev": "223mugx3xgm7u",
  "seq": 9,
  "of": 14,
  "text_len": 285,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgfKkhXj7HW6+nB/9T0j6Ee9sPs065m4HgRxMDUCS7BeFmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx4IW1vYmlsZS1maWxlLXZpZXdlci10YXAtdG8tY29tbWVudGlhcnRpZmFjdHOBoWZDb21taXR4KDZlOWJjODhlNGMyYmNkOTdiZGU1MGQ5MmM0NDcxYzg0MWEzMDhlNGFpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZaZ0PWRA8="
}
---

adversarial review of mobile-file-viewer-tap-to-comment: APPROVE-WITH-FOLLOW-UPS — Cold Opus review: primary telos met, guards/auth/XSS/binary/symlink all held against the running binary, build+test+clippy+fmt green; F1 (byte-cap truncation) fixed in-round, F2/F3 cosmetic follow-ups
***8<***
---
{
  "v": 3,
  "cid": "bafyreiblyv6jh23fwk5tttdia5y753jinhvedkjy7iqe6eqp6jpnccw4r4",
  "sig": "09cc651d62d6d2278df7f38119d57ccc99ff903e86ce9e5e773ffcaaf54e6aab65efe7c67d316e06f96c0c413093cbab6cd687b581790280fdd2e4079dd2b580",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "mobile-file-viewer-tap-to-comment"
  },
  "kind": "observation",
  "cites": [
    "bafyreid4ol7cwpe7hnq7y5m3mvnatkuivqlst224lqwbbjplr2e4xhoz54"
  ],
  "rev": "223mugx4fsmiu",
  "seq": 10,
  "of": 14,
  "text_len": 696,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgfHL+KzyfO2H8dZtlWgmqiKwXKetcXCwQpeuOicud2e9mYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx4IW1vYmlsZS1maWxlLXZpZXdlci10YXAtdG8tY29tbWVudGlhcnRpZmFjdHOBoWZDb21taXR4KDZlOWJjODhlNGMyYmNkOTdiZGU1MGQ5MmM0NDcxYzg0MWEzMDhlNGFpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZaZ0S8STE="
}
---

Review finding F1 (FIXED in-round): file_view capped lines but not bytes — a low-newline/minified file was read whole into memory and highlighted, defeating the design's "truncated head" contract (reviewer repro: a 50MB single-line file returned whole). Fix: added FILE_VIEW_MAX_BYTES=512KiB; file_view now bounds the read via File::take before decode/highlight, marks truncated on either byte- or line-overflow, accepts a multibyte char sliced at the byte boundary (valid-prefix), still rejects binary. Covered by mcp::file_view_serves_guards_and_byte_caps and the /file integration byte-cap assertion; client notice now says "truncated head — file too large" when the true total is unknown.
***8<***
---
{
  "v": 3,
  "cid": "bafyreidosblyqzzxbnawwoqshtobzekuzpg6arc2ib23n7qysb3jiyr5ee",
  "sig": "93c48acded95f2c8549e81ccb737d5c794684c36f65623e85fefbc6376c6159606784496b2cfbbd031b1a3a4fbfbc81128565e3c10496565101c1683dc1b6f66",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "mobile-file-viewer-tap-to-comment"
  },
  "kind": "observation",
  "cites": [
    "bafyreid4ol7cwpe7hnq7y5m3mvnatkuivqlst224lqwbbjplr2e4xhoz54"
  ],
  "rev": "223mugx4gkt2y",
  "seq": 11,
  "of": 14,
  "text_len": 355,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgfHL+KzyfO2H8dZtlWgmqiKwXKetcXCwQpeuOicud2e9mYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx4IW1vYmlsZS1maWxlLXZpZXdlci10YXAtdG8tY29tbWVudGlhcnRpZmFjdHOBoWZDb21taXR4KDZlOWJjODhlNGMyYmNkOTdiZGU1MGQ5MmM0NDcxYzg0MWEzMDhlNGFpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZaZ0TIY3I="
}
---

Review finding F2 (follow-up, cosmetic): GET /files lists in-repo tracked symlinks (via filetree::list) that the viewer then refuses with the guard error — dead rows. Not a leak (only the link name, never target/content, is exposed). Fix later by skipping/badging symlinks in the /files projection (avoid touching shared filetree::list used by the TUI).
***8<***
---
{
  "v": 3,
  "cid": "bafyreih4mrezj3txudpsm5we2zzxzitxehj2isxrosm2ve7xmf7nyy2t6u",
  "sig": "69ef7a06d8881c65ebfa4f4ab7e15e66b74149507005e4f578c59e9f6537c9156151f41969e81b740f2b1cc4d52af56e8afc5cb4a99f6f66eff60321d5e36a85",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "mobile-file-viewer-tap-to-comment"
  },
  "kind": "observation",
  "cites": [
    "bafyreid4ol7cwpe7hnq7y5m3mvnatkuivqlst224lqwbbjplr2e4xhoz54"
  ],
  "rev": "223mugx4hfgts",
  "seq": 12,
  "of": 14,
  "text_len": 406,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgfHL+KzyfO2H8dZtlWgmqiKwXKetcXCwQpeuOicud2e9mYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx4IW1vYmlsZS1maWxlLXZpZXdlci10YXAtdG8tY29tbWVudGlhcnRpZmFjdHOBoWZDb21taXR4KDZlOWJjODhlNGMyYmNkOTdiZGU1MGQ5MmM0NDcxYzg0MWEzMDhlNGFpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZaZ0TVsnk="
}
---

Review finding F3 (follow-up, low): in the file viewer, tapping a line that already has a comment opens that comment's thread and offers no way to add a SECOND comment anchored to the same line. Nothing is unreachable (every comment, incl. multi-per-line and unanchored line:null, is in the exhaustive vcomments list) and the first-comment dead-end is closed. Add a per-line 'add another' affordance later.
***8<***
---
{
  "v": 3,
  "cid": "bafyreie5pauz5l3rzad3rtux3mesodf5qlxhisjpmbqckktbpakgzja4pi",
  "sig": "98706376d0f02cb77817c152a3aff552fa5c522dedaa45c1cccb2f3a951b41911326643167f4a9a6b7902e79ea117c663193c10dde55cd0183be3b13f808076e",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "mobile-file-viewer-tap-to-comment"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223muh3ltrkev",
  "seq": 13,
  "of": 14,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx4IW1vYmlsZS1maWxlLXZpZXdlci10YXAtdG8tY29tbWVudGlhcnRpZmFjdHOBoWZDb21taXR4KDQyZjgzZjk0MWI3NGZjYWU4OWFlNWJmOGZmNDkyMGU3OGJiMWJlY2Zpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZaaGObwOo="
}
---
