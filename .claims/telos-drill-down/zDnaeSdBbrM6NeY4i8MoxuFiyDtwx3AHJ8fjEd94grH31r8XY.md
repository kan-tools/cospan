---
{
  "v": 3,
  "cid": "bafyreigovbxeajv4yqdx5ubnfbwsfv4udt2j2fdqtmr7wmxythe7gnapvq",
  "sig": "a347e761c5019af3af79e7730595993a6618a2a05b55663ed61ecf0be0ac3b8528d1cf3814f3a339c9d2bf3e3a2ef4960a40910d4cd22246391c19ca5502f612",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "telos-drill-down"
  },
  "kind": "observation",
  "cites": [],
  "rev": "223mtmzly6cq6",
  "seq": 0,
  "of": 8,
  "text_len": 190,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2FscHRlbG9zLWRyaWxsLWRvd25pYXJ0aWZhY3RzgaFmQ29tbWl0eCg5MDNmMDBjYTJhODg5ZDRhNzlhZGJhZjQ4NTNiOTIyNjNmZGU4MzI2aXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWZfj4h66"
}
---

design doc .design/telos-drill-down.md checked against the live design-doc schema: validation: 9 check(s), 0 failed, 1 warning(s), 0 unchecked, 0 open question(s) [doc 4402:d7c3fb7328ea9e6a]
***8<***
---
{
  "v": 3,
  "cid": "bafyreigkzyygffek5wezgnmeep5lyxvanj5zatkaxevbc45rmimp4m37di",
  "sig": "fb76e68411d24e8891c3a0315cd3567fa8fc48a2245adcdf637e574f0a8a92c24d5fbbca30711748aea98eebd2493a663ecb0f46f18d710676447088958f326a",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "telos-drill-down"
  },
  "kind": "plan",
  "cites": [
    "bafyreigovbxeajv4yqdx5ubnfbwsfv4udt2j2fdqtmr7wmxythe7gnapvq"
  ],
  "rev": "223mtmzlyhxen",
  "seq": 1,
  "of": 8,
  "text_len": 414,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4HYKlglAAFxEiDOqG5AJrzEB37QLShtIteUHPSdFHCbI/sy+JnJ8zQPrGZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZVNkQmJyTTZOZVk0aThNb3h1Rml5RHR3eDNBSEo4ZmpFZDk0Z3JIMzFyOFhZZWFnZW509mdzdWJqZWN0oWVMb2NhbHB0ZWxvcy1kcmlsbC1kb3duaWFydGlmYWN0c4GhZkNvbW1pdHgoOTAzZjAwY2EyYTg4OWQ0YTc5YWRiYWY0ODUzYjkyMjYzZmRlODMyNml3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABlmX4+b04A=="
}
---

telos-drill-down design (.design/telos-drill-down.md): Turn the Process tab's Telos sub-pane from a flat scrolling list into a selectable list that drills into one telos's full detail — statement, witnesses, and the tensions naming it — mirroring the atom pane's existing Enter/Esc drill-down (`process_detail` / `atom_detail`). [validation: 9 check(s), 0 failed, 1 warning(s), 0 unchecked, 0 open question(s)]
***8<***
---
{
  "v": 3,
  "cid": "bafyreidbn5dyvdl6sgwkhfls57tfydoxa3z72kjj75zam6vo56dut7cqju",
  "sig": "ffa6dd88137b3e706eb2b94b367bcfd90b46300b2cf49ac81f33b511bc9f68812f305f385142c798f824076dce1d48e350b00cc02cdca6e5cb3008e17de0e39d",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "telos-drill-down"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223mtmzlyl7cv",
  "seq": 2,
  "of": 8,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXgpRmVhdHVyZTogUHJvY2VzcyB0YWIg4oCUIHRlbG9zIGRyaWxsLWRvd25sc3ViamVjdF9raW5kZElkZWFlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2FscHRlbG9zLWRyaWxsLWRvd25pYXJ0aWZhY3RzgaFmQ29tbWl0eCg5MDNmMDBjYTJhODg5ZDRhNzlhZGJhZjQ4NTNiOTIyNjNmZGU4MzI2aXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWZfj6JS4"
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreicidd7kbszc3y3pjut3y2ncfux7gsy65enwlallfmuwwyt3w76msa",
  "sig": "2b9cd806aa48ea9ee172a43ab2eca96c6fd686f8de1438eb782d4587c1d6edd17b6fb09ec502b69daa10ee4ba7e0855c1b53dfb903818884106f7cae2a2b7f0b",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "telos-drill-down"
  },
  "kind": "decision",
  "cites": [
    "bafyreigkzyygffek5wezgnmeep5lyxvanj5zatkaxevbc45rmimp4m37di"
  ],
  "rev": "223mtmzlyuomp",
  "seq": 3,
  "of": 8,
  "text_len": 231,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgys4wYpSK7YmTNYQj+rxeoGp7kE1AuSoRc7FiGP4zfxpmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxwdGVsb3MtZHJpbGwtZG93bmlhcnRpZmFjdHOBoWZDb21taXR4KDkwM2YwMGNhMmE4ODlkNGE3OWFkYmFmNDg1M2I5MjI2M2ZkZTgzMjZpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZZl+PtUes="
}
---

RQ-1: Tensions are global (`ProcessSnapshot.tensions`), not per-telos, so the drill-down shows the tensions whose text contains the telos slug — the same slugs `day telos tension <a> <b>` records — rather than inventing a link.
***8<***
---
{
  "v": 3,
  "cid": "bafyreig3p2j32qywowohabvupcfmsyllgvu5zsmxrygjefd5cp7wymtze4",
  "sig": "4add8e9891b90b439eebe9b3440fa2c46c3b8d34119442096439a2a023d26db92a772f18b889a2b0b7c191b7cf6f11bd8cbd76ce7160f727395b08959481836e",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "telos-drill-down"
  },
  "kind": "decision",
  "cites": [
    "bafyreigkzyygffek5wezgnmeep5lyxvanj5zatkaxevbc45rmimp4m37di"
  ],
  "rev": "223mtmzlz6bcw",
  "seq": 4,
  "of": 8,
  "text_len": 201,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgys4wYpSK7YmTNYQj+rxeoGp7kE1AuSoRc7FiGP4zfxpmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxwdGVsb3MtZHJpbGwtZG93bmlhcnRpZmFjdHOBoWZDb21taXR4KDkwM2YwMGNhMmE4ODlkNGE3OWFkYmFmNDg1M2I5MjI2M2ZkZTgzMjZpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZZl+PyHK4="
}
---

RQ-2: `telos_scroll` is reused for the detail scroll (Telos has no list scroll beyond selection); the list is a `List` widget whose `ListState` follows `telos_selected`, matching the Ledger/rail idiom.
***8<***
---
{
  "v": 3,
  "cid": "bafyreibx6mceu6szjxivu33j44fufg5bktnqd67kj25dvgyo2d4goqxpve",
  "sig": "ef99cdb979a09973caf7d0e9f12fe4e62bf3addfe89e5c9ec2768272d7afc0882c663b98ce491b2151c1f8cd475113bb925f9be843ee0f58125ea705a54e9ef6",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "telos-drill-down"
  },
  "kind": "observation",
  "cites": [],
  "rev": "223mtnr3zlbdm",
  "seq": 5,
  "of": 8,
  "text_len": 751,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2FscHRlbG9zLWRyaWxsLWRvd25pYXJ0aWZhY3RzgaFmQ29tbWl0eCgzMjkyZjgzZDNjNjY2MzUyODdhZTk3ZTFhMWIyMzUzNmQ5NWQxNDRhaXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWZ3D+Jck"
}
---

Refined telos drill-down per review (PR #12): (1) two-column layout — telos list (38%) beside the selected telos detail (62%), Enter/Esc move focus between list and detail with pane_block focus borders; narrow terminals show only the focused pane. (2) Witness descriptions — ProcessSnapshot gains a witnesses map (witness type -> probe description) parsed from the schema/witness day-witness block via parse_witness_probes/describe_probe; telos_detail renders each witness as "name — <probe>" (code-change — material+record, design-doc — path: .design/*.md), bare name when no probe. render_scrolled wraps. Verified on cospan: 8 probes known, p0-spine shows code-change/verdict with descriptions. cargo test 127/127, clippy clean, fmt clean.
***8<***
---
{
  "v": 3,
  "cid": "bafyreiem2fxp5drrbochbqp5u3e7fahyw65554cicwyjttcssjaifsui6y",
  "sig": "08f6ab150749998248523b87bde9775f29671cf9f42ff5a36d27a7a413c5144c43f79636cb674e41594d8624819ef7353a766ce1947c1918befa77b4e5bc06fb",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "telos-drill-down"
  },
  "kind": "observation",
  "cites": [],
  "rev": "223mtnrzoz2ye",
  "seq": 6,
  "of": 8,
  "text_len": 824,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2FscHRlbG9zLWRyaWxsLWRvd25pYXJ0aWZhY3RzgaFmQ29tbWl0eCg5YmM2ZTU3OTA2ZWNkNzljYTQ4M2YwYjkyMmRkMmMzYmU4MDg2ZGUxaXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWZ3/T4LQ"
}
---

Richer + formatted telos detail (PR #12): (1) tensions now carry their rationale — parse_tension dropped all but the between pair; ProcessSnapshot.tensions is a Tension {between, why} struct capturing the claim prose; the detail shows each tension pair AND its recorded why, filtered by between (not string-match). (2) Richer witness probes via describe_probe_rich (unfolds one nesting level: "material path src/*.rs, record claim"; "command scripts/no-tracked-junk.sh"). (3) telos_detail returns styled ratatui Lines like the chat view: magenta slug, cyan section headers, markdown statement + tension why, green witness names with dim probes, yellow tension pairs. Verified on cospan: disposable telos shows its witness probe and the full disposable<->kan-is-truth rationale. cargo test 127/127, clippy clean, fmt clean.
***8<***
---
{
  "v": 3,
  "cid": "bafyreigcku4vfgmluwmn54mz7yuma4hi7ldmwpwgfg5rg7jiqvfaezrwna",
  "sig": "b69411b8c6e2bf594ca9e68d0a471c7293ff9a7fb3a8aefdc4e9cd5baf563f261b97e3643f19543741cfbd5161c76d2567c34eb0527bb7a3c6b71f7908036086",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "telos-drill-down"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mtov6jpdq2",
  "seq": 7,
  "of": 8,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxwdGVsb3MtZHJpbGwtZG93bmlhcnRpZmFjdHOBoWZDb21taXR4KDAwY2ViOTU2N2ZiMDU3MTg1MDhiOTNiMjE2MTY3YjdjOTc5NDY1ZWFpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZZpsj6oOo="
}
---
