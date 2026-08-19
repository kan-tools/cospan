---
{
  "v": 3,
  "cid": "bafyreid3lec7gg7qgvp5ar5bpdg67y7vkxtv7p4baiviqpfpcwb3jpqqky",
  "sig": "768ef496572f12d2872ba7c9c6f8a5f68c2320c722e5a4a557393aec20915cbd0af0c6f4497c995e5efd44615808b6fd48be29d43dd2886033c4fcd27943bf03",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "two-pane-claim-browser"
  },
  "kind": "observation",
  "cites": [],
  "rev": "223mtfm3vrcuo",
  "seq": 0,
  "of": 9,
  "text_len": 196,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2FsdnR3by1wYW5lLWNsYWltLWJyb3dzZXJpYXJ0aWZhY3RzgaFmQ29tbWl0eCg4OGVkZTBiNjNiOWVkOTQxNTNlYmYwOWE3ZWZkMWFjYzI5NmU3NTczaXdvcmtzcGFjZaFpV29ya3NwYWNleEA4MzVmODdhZGU0MGJkYzhkZjIyYzNiYmQyNzhkNTdlMDQwZmFhMDhjYzZlNDE1ZDZiZDAzODRlYjlhN2U1M2Nla3JlY29yZGVkX2F0GwAGWVyDu6Lo"
}
---

design doc .design/two-pane-claim-browser.md checked against the live design-doc schema: validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s) [doc 7663:758bb7e070a89367]
***8<***
---
{
  "v": 3,
  "cid": "bafyreif2wowkasjcmdwqlpqgjaldr5dzfmcao3dd652ejsikzejzv7jzi4",
  "sig": "5543536e9b0f422d0d30f7978499d6a88dad86e13403f6bef863d692bde55a836c25947c0d778da2db0bf895fdee9c9d838cd6956412288cddb81b4cf874c9c2",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "two-pane-claim-browser"
  },
  "kind": "plan",
  "cites": [
    "bafyreid3lec7gg7qgvp5ar5bpdg67y7vkxtv7p4baiviqpfpcwb3jpqqky"
  ],
  "rev": "223mtfm3vuudo",
  "seq": 1,
  "of": 9,
  "text_len": 647,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4HYKlglAAFxEiB7WQXzG/A1X9BHoXjN7+P1Vedfv4ECKog8rxWDtL4QVmZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZVNkQmJyTTZOZVk0aThNb3h1Rml5RHR3eDNBSEo4ZmpFZDk0Z3JIMzFyOFhZZWFnZW509mdzdWJqZWN0oWVMb2NhbHZ0d28tcGFuZS1jbGFpbS1icm93c2VyaWFydGlmYWN0c4GhZkNvbW1pdHgoODhlZGUwYjYzYjllZDk0MTUzZWJmMDlhN2VmZDFhY2MyOTZlNzU3M2l3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAODM1Zjg3YWRlNDBiZGM4ZGYyMmMzYmJkMjc4ZDU3ZTA0MGZhYTA4Y2M2ZTQxNWQ2YmQwMzg0ZWI5YTdlNTNjZWtyZWNvcmRlZF9hdBsABllcg71ozw=="
}
---

two-pane-claim-browser design (.design/two-pane-claim-browser.md): Split the interactive `watch-repo` TUI into two panes: the grouped subject list on the left (the selection already built in Step 2) and, on the right, the selected subject's live claims folded via `subject_claims` (Step 1), updating as the selection moves. A first responsive breakpoint shows both panes side by side on wide terminals and one at a time on narrow ones. This is the seed of the claims browser — you can arrow through subjects and read their claims in-pane. Serves `telos/p0-spine`. [validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s)]
***8<***
---
{
  "v": 3,
  "cid": "bafyreibiq6bdw34zxo3v5btzt7yib2h5jg7rhvhsqiulgecjrkjp4ppism",
  "sig": "420a60941fa1b2b45e9f5d8e89a95425bfdb21eb710634767accce8cb47078967ac82dac42228a7635eb2fc3103b919f9565a9d02b6236f505dcc02c63ec8678",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "two-pane-claim-browser"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223mtfm3vw3f6",
  "seq": 2,
  "of": 9,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXgtRmVhdHVyZTogVHdvLXBhbmUgc3ViamVjdCBsaXN0ICsgY2xhaW0gZGV0YWlsbHN1YmplY3Rfa2luZGRJZGVhZWNpdGVzgGZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZVNkQmJyTTZOZVk0aThNb3h1Rml5RHR3eDNBSEo4ZmpFZDk0Z3JIMzFyOFhZZWFnZW509mdzdWJqZWN0oWVMb2NhbHZ0d28tcGFuZS1jbGFpbS1icm93c2VyaWFydGlmYWN0c4GhZkNvbW1pdHgoODhlZGUwYjYzYjllZDk0MTUzZWJmMDlhN2VmZDFhY2MyOTZlNzU3M2l3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAODM1Zjg3YWRlNDBiZGM4ZGYyMmMzYmJkMjc4ZDU3ZTA0MGZhYTA4Y2M2ZTQxNWQ2YmQwMzg0ZWI5YTdlNTNjZWtyZWNvcmRlZF9hdBsABllcg74E+w=="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreigxnd25gky3pucqvbjwkioohlj3fbtsgisjp35y34bf5rqedhic5y",
  "sig": "6667fd52299a88b5b1e24c85ea58484f2f3fb55fecc73168ca9d03c2384227d134b5147cf1214935f48f7f68be5dab58664101d1d2ca4b659d496109554159f6",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "two-pane-claim-browser"
  },
  "kind": "decision",
  "cites": [
    "bafyreif2wowkasjcmdwqlpqgjaldr5dzfmcao3dd652ejsikzejzv7jzi4"
  ],
  "rev": "223mtfm3w3m6t",
  "seq": 3,
  "of": 9,
  "text_len": 212,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgurOsoEkiYO0FvgZIFjj0eSsEB2xj93REyQrJE5r9OUdmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx2dHdvLXBhbmUtY2xhaW0tYnJvd3NlcmlhcnRpZmFjdHOBoWZDb21taXR4KDg4ZWRlMGI2M2I5ZWQ5NDE1M2ViZjA5YTdlZmQxYWNjMjk2ZTc1NzNpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDgzNWY4N2FkZTQwYmRjOGRmMjJjM2JiZDI3OGQ1N2UwNDBmYWEwOGNjNmU0MTVkNmJkMDM4NGViOWE3ZTUzY2VrcmVjb3JkZWRfYXQbAAZZXIPAyDA="
}
---

RQ-1: The right pane fetches on selection change and caches per fold generation — at most one `kan show` spawn per subject until the log changes, so scrolling back is instant and a re-fold refreshes everything.
***8<***
---
{
  "v": 3,
  "cid": "bafyreicyukibnjoxn7frjc72r2iznydfs7ebowcqhy6ca5cexp7m5vsqai",
  "sig": "c4125aaa1020844b9e625d090865d32525bcf2a9ea2ee93787cc822323f803e862cdf94cabcb6f3da62b4643a3a8f2dd6406bfbbe44867d42cee3ab026291701",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "two-pane-claim-browser"
  },
  "kind": "decision",
  "cites": [
    "bafyreif2wowkasjcmdwqlpqgjaldr5dzfmcao3dd652ejsikzejzv7jzi4"
  ],
  "rev": "223mtfm3w76ey",
  "seq": 4,
  "of": 9,
  "text_len": 182,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgurOsoEkiYO0FvgZIFjj0eSsEB2xj93REyQrJE5r9OUdmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx2dHdvLXBhbmUtY2xhaW0tYnJvd3NlcmlhcnRpZmFjdHOBoWZDb21taXR4KDg4ZWRlMGI2M2I5ZWQ5NDE1M2ViZjA5YTdlZmQxYWNjMjk2ZTc1NzNpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDgzNWY4N2FkZTQwYmRjOGRmMjJjM2JiZDI3OGQ1N2UwNDBmYWEwOGNjNmU0MTVkNmJkMDM4NGViOWE3ZTUzY2VrcmVjb3JkZWRfYXQbAAZZXIPCkPY="
}
---

RQ-2: The narrow switcher is `Enter` to open the detail and `Esc` to return; the breakpoint to side-by-side is 100 columns. `j`/`k` always move the subject selection in both layouts.
***8<***
---
{
  "v": 3,
  "cid": "bafyreiga2bpsvgh4pma2h46llaf6nk37jsbjvqy3hns7mlr5xurw32be5i",
  "sig": "9294489da6d6ac8fb05ba3529bf658bdc5b08073c67f31dbd55c5f4f46ab930b609fe206b066bf2600c9766dd36b8369dfc1ca5f85cd9dec963e159b8990754d",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "two-pane-claim-browser"
  },
  "kind": "decision",
  "cites": [
    "bafyreif2wowkasjcmdwqlpqgjaldr5dzfmcao3dd652ejsikzejzv7jzi4"
  ],
  "rev": "223mtfm3wcphv",
  "seq": 5,
  "of": 9,
  "text_len": 185,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgurOsoEkiYO0FvgZIFjj0eSsEB2xj93REyQrJE5r9OUdmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx2dHdvLXBhbmUtY2xhaW0tYnJvd3NlcmlhcnRpZmFjdHOBoWZDb21taXR4KDg4ZWRlMGI2M2I5ZWQ5NDE1M2ViZjA5YTdlZmQxYWNjMjk2ZTc1NzNpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDgzNWY4N2FkZTQwYmRjOGRmMjJjM2JiZDI3OGQ1N2UwNDBmYWEwOGNjNmU0MTVkNmJkMDM4NGViOWE3ZTUzY2VrcmVjb3JkZWRfYXQbAAZZXIPEVVU="
}
---

RQ-3: The compact day-summary header is out of scope for this step — blocked on a machine-readable day status (recorded on `day-summary-in-cospan`); Step 3 is the two-pane split only.
***8<***
---
{
  "v": 3,
  "cid": "bafyreigiqzjmct6chaxlgbnf4g3r24ao5vwxeheln6kocerywnegjlpcje",
  "sig": "f11641950eccb45f42c8ac840093383a5dae6ca8173de075e24295ca7c254410256f70bd4e2d4ab6c0da8a9d78cdb6a0c31c93bf23905d4bc82e84ce4a78e01f",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "two-pane-claim-browser"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mtfm3wq4vt",
  "seq": 6,
  "of": 9,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx2dHdvLXBhbmUtY2xhaW0tYnJvd3NlcmlhcnRpZmFjdHOBoWZDb21taXR4KDg4ZWRlMGI2M2I5ZWQ5NDE1M2ViZjA5YTdlZmQxYWNjMjk2ZTc1NzNpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDgzNWY4N2FkZTQwYmRjOGRmMjJjM2JiZDI3OGQ1N2UwNDBmYWEwOGNjNmU0MTVkNmJkMDM4NGViOWE3ZTUzY2VrcmVjb3JkZWRfYXQbAAZZXIPLCw4="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreihv7z7zii7iu7jsk237uhqr2kors7h4uavgaxbecwvtlxronht7um",
  "sig": "05c720d4b63ebfc378fcc715db27a4693bbef25849ff626c1a151b0acf5757d758b090afdc29ebd62b26535ef0df13e5369210624ef5fb246e9fcd90c52d336b",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "two-pane-claim-browser"
  },
  "kind": "decision",
  "cites": [
    "bafyreif2wowkasjcmdwqlpqgjaldr5dzfmcao3dd652ejsikzejzv7jzi4"
  ],
  "rev": "223mtfn3irtqs",
  "seq": 7,
  "of": 9,
  "text_len": 595,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgurOsoEkiYO0FvgZIFjj0eSsEB2xj93REyQrJE5r9OUdmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx2dHdvLXBhbmUtY2xhaW0tYnJvd3NlcmlhcnRpZmFjdHOBoWZDb21taXR4KDg4ZWRlMGI2M2I5ZWQ5NDE1M2ViZjA5YTdlZmQxYWNjMjk2ZTc1NzNpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDgzNWY4N2FkZTQwYmRjOGRmMjJjM2JiZDI3OGQ1N2UwNDBmYWEwOGNjNmU0MTVkNmJkMDM4NGViOWE3ZTUzY2VrcmVjb3JkZWRfYXQbAAZZXMLr5nI="
}
---

adversarial review of two-pane-claim-browser: APPROVE-WITH-FOLLOW-UPS — adversarial review of two-pane-claim-browser (Step 3): cache is once-per-fold and cleared on refold, right pane is a consistent projection of subject_claims keyed on the highlighted subject (no stale/wrong-subject render), the four detail states are distinct, the 100-col breakpoint and shared Claim::display_line are correct, and every test is load-bearing; shippable but for one introduced regression (5 new clippy needless_borrows warnings in tui.rs test code fail -D warnings) plus two cosmetic honest-ambiguity nits.
***8<***
---
{
  "v": 3,
  "cid": "bafyreiapybg6siitih4c5by4sdkc66n7fuwav4dgi33wld6rx2ucflvnoy",
  "sig": "ebd83d0a11a084ee7b4f1cae257bfd530236368eccb302076852c5951267b5dd284b121cbf4cf57d42d276975e1673f0765c6a8ac37873760a48cb602cd247fc",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "two-pane-claim-browser"
  },
  "kind": "observation",
  "cites": [
    "bafyreihv7z7zii7iu7jsk237uhqr2kors7h4uavgaxbecwvtlxronht7um"
  ],
  "rev": "223mtfn3r3rpx",
  "seq": 8,
  "of": 9,
  "text_len": 387,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIg9f5/lCPop9Mla3+h4R0p0ZfPygKmBcJBWrNd4uaef6NmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx2dHdvLXBhbmUtY2xhaW0tYnJvd3NlcmlhcnRpZmFjdHOBoWZDb21taXR4KDg4ZWRlMGI2M2I5ZWQ5NDE1M2ViZjA5YTdlZmQxYWNjMjk2ZTc1NzNpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDgzNWY4N2FkZTQwYmRjOGRmMjJjM2JiZDI3OGQ1N2UwNDBmYWEwOGNjNmU0MTVkNmJkMDM4NGViOWE3ZTUzY2VrcmVjb3JkZWRfYXQbAAZZXMNw3lM="
}
---

Follow-up (from adversarial review): Step 3's new tui.rs test code introduced 5 clippy needless_borrows_for_generic_args warnings (src/tui.rs:586,588,591,594,598 — pass fetch, not &fetch). `cargo clippy --all-targets -- -D warnings` now exits 101; pre-diff the repo had only 3 pre-existing doc-overindent warnings on src/lib.rs. Test-only, mechanical; the new code is not clippy-clean.
