---
{
  "v": 3,
  "cid": "bafyreihkwyugs3fqhm7fbsvbb3hsqq5mz6hqq5l6xcjovqj34s5pfagbau",
  "sig": "240bcbe3bc2de4c5bcf62046f908a351bce51c35ba3d8e3e81a3c7ce6d0f530e3ef4f33b73609d739492bc95fb666895a22f38f50e7ad3cfd0003368942016ff",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "teloi-grid-drilldown"
  },
  "kind": "observation",
  "cites": [],
  "rev": "223munbvtt4ow",
  "seq": 0,
  "of": 10,
  "text_len": 194,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2FsdHRlbG9pLWdyaWQtZHJpbGxkb3duaWFydGlmYWN0c4GhZkNvbW1pdHgoODIxNzI0OGQ5MjIwYjdiOWI1MTE1YjA4NTY3YTY3Yzk4ODg4ODBkZWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABlqZ95yKJg=="
}
---

design doc .design/teloi-grid-drilldown.md checked against the live design-doc schema: validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s) [doc 8529:1f00b84c757e8e6f]
***8<***
---
{
  "v": 3,
  "cid": "bafyreib2q76vssx5hv3gzvpdeordfny4mosvkxxse5e75f5hb32vip3qaq",
  "sig": "08bf95ea88db24dccadfc8204450ed01ec4afb72ede145c07646863eaea3a4fb2d87db0aeb01c020ff07c92df3e4def5b205e25313d7191e04163e16bbb61c51",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "teloi-grid-drilldown"
  },
  "kind": "plan",
  "cites": [
    "bafyreihkwyugs3fqhm7fbsvbb3hsqq5mz6hqq5l6xcjovqj34s5pfagbau"
  ],
  "rev": "223munbvuelze",
  "seq": 1,
  "of": 10,
  "text_len": 623,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4HYKlglAAFxEiDqtihpbLA7PlDKoQ7PKEOsz48IdX64kurBO+S68oDBBWZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZVNkQmJyTTZOZVk0aThNb3h1Rml5RHR3eDNBSEo4ZmpFZDk0Z3JIMzFyOFhZZWFnZW509mdzdWJqZWN0oWVMb2NhbHR0ZWxvaS1ncmlkLWRyaWxsZG93bmlhcnRpZmFjdHOBoWZDb21taXR4KDgyMTcyNDhkOTIyMGI3YjliNTExNWIwODU2N2E2N2M5ODg4ODgwZGVpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZamfelR3A="
}
---

teloi-grid-drilldown design (.design/teloi-grid-drilldown.md): Turn the serve page's Teloi tab from a flat list into a master-detail view: a grid of telos cards in the list pane, and — on tapping one — that telos's full detail in the detail pane (its statement, each witness with the human probe description from the `schema/witness` map, the tensions that name it, and its recorded claims). Reuses the responsive-layout slice's pane machinery and the Browse tab's claim renderer; all data is already in the fold, so no endpoint changes. [validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s)]
***8<***
---
{
  "v": 3,
  "cid": "bafyreibstv5c6syqvnpi5crtouwuqiru4tccksybqlmc5om53gumr4xdgu",
  "sig": "f3f2bb0f953ffa966573e749d6eb78ccfe3872a7f19658e663a73b2808d5277908e64c3d37a3ce50729017d204acb74583b15ab04f7f71d891d2af4559815604",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "teloi-grid-drilldown"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223munbvul5ge",
  "seq": 2,
  "of": 10,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXggRmVhdHVyZTogVGVsb2kgZ3JpZCArIGRyaWxsLWRvd25sc3ViamVjdF9raW5kZElkZWFlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2FsdHRlbG9pLWdyaWQtZHJpbGxkb3duaWFydGlmYWN0c4GhZkNvbW1pdHgoODIxNzI0OGQ5MjIwYjdiOWI1MTE1YjA4NTY3YTY3Yzk4ODg4ODBkZWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABlqZ96iNEg=="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreiepywmldpb7m3i2bgljdo2pealu23e7uxfywjhhjh7rbzcbl75uk4",
  "sig": "8cdf3f9d2e3e2c81887ad19c117a8a6b4a8754f22f1392b9d4593b9cd97189163af051d76a9b12beef42547c7572722f8951f03cb7d50b9c452f2570f9573c0b",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "teloi-grid-drilldown"
  },
  "kind": "decision",
  "cites": [
    "bafyreib2q76vssx5hv3gzvpdeordfny4mosvkxxse5e75f5hb32vip3qaq"
  ],
  "rev": "223munbvv57nj",
  "seq": 3,
  "of": 10,
  "text_len": 312,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgOof9WUr9PXZs1eMjojK3HGOlVV7yJ0n+l6cO9VQ/cARmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx0dGVsb2ktZ3JpZC1kcmlsbGRvd25pYXJ0aWZhY3RzgaFmQ29tbWl0eCg4MjE3MjQ4ZDkyMjBiN2I5YjUxMTViMDg1NjdhNjdjOTg4ODg4MGRlaXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWpn3sZX4"
}
---

RQ-1: Master-detail, not a full-width grid-that-replaces — Teloi reuses the `.view.md` list/detail panes like Comments and Chat, for one consistent drill-in model across the page. The card grid lives in the (narrower) list pane and is a responsive 1–2 column layout; the operator chose this over a wide grid.
***8<***
---
{
  "v": 3,
  "cid": "bafyreibfdn3lltwkhgoyozu4336vwvmn363rbpsjy5vat6km26p4thynuq",
  "sig": "62effd1d259bd1a73ec5e95819822ed244e2d9f691cace6eebf6ce3730feeb4b7bc6f4c96d169dfa60bbd61fd9332aef7ca519a2bfa00fb3a4efe294a30c0bda",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "teloi-grid-drilldown"
  },
  "kind": "decision",
  "cites": [
    "bafyreib2q76vssx5hv3gzvpdeordfny4mosvkxxse5e75f5hb32vip3qaq"
  ],
  "rev": "223munbvvp3r4",
  "seq": 4,
  "of": 10,
  "text_len": 251,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgOof9WUr9PXZs1eMjojK3HGOlVV7yJ0n+l6cO9VQ/cARmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx0dGVsb2ktZ3JpZC1kcmlsbGRvd25pYXJ0aWZhY3RzgaFmQ29tbWl0eCg4MjE3MjQ4ZDkyMjBiN2I5YjUxMTViMDg1NjdhNjdjOTg4ODg4MGRlaXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWpn3uoZu"
}
---

RQ-2: The telos detail includes its recorded claims, rendered with the shared Browse `claimEl`, rather than only linking out to Browse — a telos opens to everything about it (`telos/readable-claim-browser`), and reuse avoids a second claim renderer.
***8<***
---
{
  "v": 3,
  "cid": "bafyreie5exsyybcr4l5f7obuy65lndndstlrb3ljdjumrwlwanm7e7cg74",
  "sig": "b48fb116de0d990fc6bda56ba1fffc795f68db4b7f37bef1a8f60721e2aec4a46cf1f4575943ca77d6a196db7a0586e1d1e6b7196566d52a5e627973c6dcf5ec",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "teloi-grid-drilldown"
  },
  "kind": "decision",
  "cites": [
    "bafyreib2q76vssx5hv3gzvpdeordfny4mosvkxxse5e75f5hb32vip3qaq"
  ],
  "rev": "223munbvwbxvu",
  "seq": 5,
  "of": 10,
  "text_len": 223,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgOof9WUr9PXZs1eMjojK3HGOlVV7yJ0n+l6cO9VQ/cARmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx0dGVsb2ktZ3JpZC1kcmlsbGRvd25pYXJ0aWZhY3RzgaFmQ29tbWl0eCg4MjE3MjQ4ZDkyMjBiN2I5YjUxMTViMDg1NjdhNjdjOTg4ODg4MGRlaXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWpn3w/cD"
}
---

RQ-3: Keep the standalone "Tensions held" overview on the list page in addition to per-telos tensions in the detail — the global map is worth an at-a-glance view, and a tension naturally appears under both teloi it names.
***8<***
---
{
  "v": 3,
  "cid": "bafyreibr7kfr43mlps7uyeow4vgx2vk6sa424spcgpo4rzh6itnq7n4rau",
  "sig": "4945e82b54a0c251a38808dec54cd677443588e3759c67aceb2a5917835a600f0f9e8488da889bab939049dae03e888eeaf68522178e51f731a9268f162de46c",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "teloi-grid-drilldown"
  },
  "kind": "result",
  "cites": [
    "bafyreib2q76vssx5hv3gzvpdeordfny4mosvkxxse5e75f5hb32vip3qaq"
  ],
  "rev": "223munc5ihp5q",
  "seq": 6,
  "of": 10,
  "text_len": 1622,
  "content": "p2Rib2R5oWZSZXN1bHShZHRleHRgZWNpdGVzgdgqWCUAAXESIDqH/VlK/T12bNXjI6IytxxjpVVe8idJ/penDvVUP3AEZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2FsdHRlbG9pLWdyaWQtZHJpbGxkb3duaWFydGlmYWN0c4GhZkNvbW1pdHgoODIxNzI0OGQ5MjIwYjdiOWI1MTE1YjA4NTY3YTY3Yzk4ODg4ODBkZWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABlqaBubUAA=="
}
---

generative-build complete for the teloi grid + drill-down (page-only slice). All changes in src/web/index.html plus test updates in server.rs; no Rust behavior/API change.

Code-change: #view-telos became a master-detail .view.md (pane-list #telos-list + pane-detail #telos-detail), reusing the slice-1 pane machinery. renderTeloi now fills the list pane with a responsive telos-grid of tappable cards (openTelos(slug)) plus the retained "Tensions held" overview, and runs on every fold tick without ever touching the detail pane (an open telos detail survives a /stream re-render; a placeholder shows when nothing is selected). New openTelos(slug) fills the detail pane: title+statement, each witness with its probe description from fold.process.witnesses (falling back to a bare type when absent, honest-ambiguity), the tensions filtered to those naming the slug, and the telos's claims from fold.claims["telos/"+slug] rendered with the SHARED Browse claimEl (no second renderer). selectedTelos marks the active card. All data from the already-served fold — no endpoint change; page stays one include_str! document, no new dependency.

Evidence: cargo test 213 unit + 14 integration green (new index_html_wires_the_teloi_grid asserts the panes, grid, openTelos, list/detail pane targeting, witness-desc map read, telos-claims read, claimEl reuse, retained tensions overview, and no new dependency; the responsive test's md/pane counts updated 2->3 as Teloi joined the master-detail views); clippy -D warnings clean; fmt clean. UNVERIFIED LAYER: the visual grid/drill-in render at both widths needs an operator eyeball.
***8<***
---
{
  "v": 3,
  "cid": "bafyreieatveq4m5dwflykuuyixqrqikeu6ozivr73sdux2besm6xuisvqq",
  "sig": "877ba6894254ea48e7c343eed3606582aaf8140b7643bbe4a2188858700168b9535ab0f6d3c8b7de72aacef2b1a78b51c3d4789ac9dddbc86aa783dd44a61d2e",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "teloi-grid-drilldown"
  },
  "kind": "decision",
  "cites": [
    "bafyreib2q76vssx5hv3gzvpdeordfny4mosvkxxse5e75f5hb32vip3qaq"
  ],
  "rev": "223muncei6pzm",
  "seq": 7,
  "of": 10,
  "text_len": 480,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgOof9WUr9PXZs1eMjojK3HGOlVV7yJ0n+l6cO9VQ/cARmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx0dGVsb2ktZ3JpZC1kcmlsbGRvd25pYXJ0aWZhY3RzgaFmQ29tbWl0eCg4MjE3MjQ4ZDkyMjBiN2I5YjUxMTViMDg1NjdhNjdjOTg4ODg4MGRlaXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWpoU4lIT"
}
---

adversarial review of teloi-grid-drilldown: APPROVE — Cold Opus review: clean APPROVE. Master-detail markup, list-pane-only live re-render that provably never disturbs an open detail, honest witness fallback, genuine claimEl reuse on correctly-shaped telos-subject claims, correct slug-matched tension filter, zero new dependencies, no XSS, honest test-count bump; build+test+clippy+fmt green. Only unverified piece is the visual render (delegated to operator eyeball per AC-5).
***8<***
---
{
  "v": 3,
  "cid": "bafyreigh7jattg4bczcbcfpznaxbfvoaicgwe4vfxe5wb4ialtrzad4ghq",
  "sig": "3fc888f2c6e9446cc64e62b98ca20e0ccb8315d7b29246b73a81363a0920e4a02c35c2bf87126b8acf73e7e73b0992439c32bd12919658616bb9b3ef9c7b8878",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "teloi-grid-drilldown"
  },
  "kind": "decision",
  "cites": [
    "bafyreib2q76vssx5hv3gzvpdeordfny4mosvkxxse5e75f5hb32vip3qaq"
  ],
  "rev": "223mup7tfueqq",
  "seq": 8,
  "of": 10,
  "text_len": 327,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgOof9WUr9PXZs1eMjojK3HGOlVV7yJ0n+l6cO9VQ/cARmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx0dGVsb2ktZ3JpZC1kcmlsbGRvd25pYXJ0aWZhY3RzgaFmQ29tbWl0eCg4MjE3MjQ4ZDkyMjBiN2I5YjUxMTViMDg1NjdhNjdjOTg4ODg4MGRlaXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWqlyvSbx"
}
---

Post-review eyeball refinement: the telos list pane renders the teloi as a single-column list of full-width rows (.telos-rows, a flex column), not a multi-column card grid. The operator liked the drill-down but asked for rows over a grid. Drill-in detail unchanged. CSS/markup/test renamed telos-grid -> telos-rows accordingly.
***8<***
---
{
  "v": 3,
  "cid": "bafyreieyp6d6zterdgqhdu64zxnny23yetj75vwaxgeofn43pptwywdw3i",
  "sig": "8251e07e884a4577456b94022c338193071e17dc1fdf24ad3c95061da2d0d804510ae17e441b07f262c0621bb04ef385f278ee81a54033dfc07d165c6d11eac7",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "teloi-grid-drilldown"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mupa64hcei",
  "seq": 9,
  "of": 10,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx0dGVsb2ktZ3JpZC1kcmlsbGRvd25pYXJ0aWZhY3RzgaFmQ29tbWl0eChmOTA5NjVhZGVkNDIwMDkwODM2MThjNjE3OTY3YWVjNmVjZTI5OTQ2aXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWqmIJqDf"
}
---
