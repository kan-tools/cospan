---
{
  "v": 3,
  "cid": "bafyreiawuozzzqx4s4556ax7ras5bjjxiq6pcnphlrnggjmt7dpqlzlaty",
  "sig": "ed6cc625e1e25294e867e9c88f3cd40dea1bce4ad0195c7d8fc0109ae92265ef2c00e5edbbf0a9cd537ebbcbe337bbd0d7f18827358b1a2e8e636c8cc310e576",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "tab-restructure"
  },
  "kind": "observation",
  "cites": [],
  "rev": "223mtjsgel2lb",
  "seq": 0,
  "of": 8,
  "text_len": 189,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2Fsb3RhYi1yZXN0cnVjdHVyZWlhcnRpZmFjdHOBoWZDb21taXR4KGFjNzk0ZDk0NGJiMzQ5M2Q5NmViMTM0NDgxNzk2ZWRjYjlhYWRkMWJpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZZfhiofO0="
}
---

design doc .design/tab-restructure.md checked against the live design-doc schema: validation: 9 check(s), 0 failed, 1 warning(s), 0 unchecked, 0 open question(s) [doc 4793:cfcb9f0041abd2d4]
***8<***
---
{
  "v": 3,
  "cid": "bafyreid5s3z65rni6hwqqg6bzd5bsbuygdvwsbq7dg2bsrmhdjabtiht24",
  "sig": "89ffc21f1475c6fda23694b56d0bc2c0df41ad9737d1b4a57e7a3fa64ca6e387763532f6254357868869e6bc113b6ff71b7a8326dd235051e49b9826ac8cf169",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "tab-restructure"
  },
  "kind": "plan",
  "cites": [
    "bafyreiawuozzzqx4s4556ax7ras5bjjxiq6pcnphlrnggjmt7dpqlzlaty"
  ],
  "rev": "223mtjsgeuqut",
  "seq": 1,
  "of": 8,
  "text_len": 596,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4HYKlglAAFxEiAWo7OcwvyXO98C/4gl0KU3RDzxNedcWmMlk/jfBeVgnmZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZVNkQmJyTTZOZVk0aThNb3h1Rml5RHR3eDNBSEo4ZmpFZDk0Z3JIMzFyOFhZZWFnZW509mdzdWJqZWN0oWVMb2NhbG90YWItcmVzdHJ1Y3R1cmVpYXJ0aWZhY3RzgaFmQ29tbWl0eChhYzc5NGQ5NDRiYjM0OTNkOTZlYjEzNDQ4MTc5NmVkY2I5YWFkZDFiaXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWX4YrVre"
}
---

tab-restructure design (.design/tab-restructure.md): Restructure the top-level tabs toward the recorded UI vision: rename Browser to **Ledger**, and fold today's separate Atoms and Telos views into a single **Process** tab with an atoms/telos sub-pane. The tab bar becomes `1 Comments · 2 Ledger · 3 Process` (Chat deferred until its architecture is designed). This is the scaffold the later Process content reshape (atoms-as-flowchart, telos drill-down) hangs off; the pane contents are unchanged in this cycle. [validation: 9 check(s), 0 failed, 1 warning(s), 0 unchecked, 0 open question(s)]
***8<***
---
{
  "v": 3,
  "cid": "bafyreiblpeezguaysumyvsmhgyiu6gf5mjmucbx2cpshaqqkakjmumd3vy",
  "sig": "78f0b5445c66b01090dec1b95b8789269a1650a825b9f90b9d020aed310481d07d79d081cd77d20ea6cbd2bd30098a80cc320f2a29911a2eca2932e9e78989ca",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "tab-restructure"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223mtjsgexysj",
  "seq": 2,
  "of": 8,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXg6RmVhdHVyZTogdGFiIHJlc3RydWN0dXJlIOKAlCBDb21tZW50cyDCtyBMZWRnZXIgwrcgUHJvY2Vzc2xzdWJqZWN0X2tpbmRkSWRlYWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxvdGFiLXJlc3RydWN0dXJlaWFydGlmYWN0c4GhZkNvbW1pdHgoYWM3OTRkOTQ0YmIzNDkzZDk2ZWIxMzQ0ODE3OTZlZGNiOWFhZGQxYml3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABll+GK76nQ=="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreihcv7oda3b7rrexnstnsuf3buverf4hcnpfxdiprdqut7632i3mkm",
  "sig": "ad2c8b9f5750cf9478f5758f89a9dfa0009251e9fc9db870e430b070b8645c555d06199a3b6654c85fe66f039a9c25289199bc2651f076ac3947d657d0dc37ff",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "tab-restructure"
  },
  "kind": "decision",
  "cites": [
    "bafyreid5s3z65rni6hwqqg6bzd5bsbuygdvwsbq7dg2bsrmhdjabtiht24"
  ],
  "rev": "223mtjsgfbnsx",
  "seq": 3,
  "of": 8,
  "text_len": 243,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgfZbz7sWo8e0IG8HI+hkGmDDraQYfGbQZRYcaQBmg89dmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxvdGFiLXJlc3RydWN0dXJlaWFydGlmYWN0c4GhZkNvbW1pdHgoYWM3OTRkOTQ0YmIzNDkzZDk2ZWIxMzQ0ODE3OTZlZGNiOWFhZGQxYml3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABll+GLPOpw=="
}
---

RQ-1: Atoms and Telos become sub-panes of one Process tab (toggled `←`/`→`) rather than two tabs, matching the vision's single Process tab; their content is carried over unchanged now and reshaped (flowchart / drill-down) in a later cycle.
***8<***
---
{
  "v": 3,
  "cid": "bafyreigfi5hpyifmbazwrl5ifnaf5mdl2dsh3rg4gad4bhbqrftuwdvvh4",
  "sig": "cfe54fa082c1d0066b2d9b5ca0a43c1396aebc37c4672f0cba08a065884157cf66d81fcd1c73168cd4011aacfe2fe811e33e3763fc5c38395bc1187e98f9c3d1",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "tab-restructure"
  },
  "kind": "decision",
  "cites": [
    "bafyreid5s3z65rni6hwqqg6bzd5bsbuygdvwsbq7dg2bsrmhdjabtiht24"
  ],
  "rev": "223mtjsgflfbo",
  "seq": 4,
  "of": 8,
  "text_len": 147,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgfZbz7sWo8e0IG8HI+hkGmDDraQYfGbQZRYcaQBmg89dmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxvdGFiLXJlc3RydWN0dXJlaWFydGlmYWN0c4GhZkNvbW1pdHgoYWM3OTRkOTQ0YmIzNDkzZDk2ZWIxMzQ0ODE3OTZlZGNiOWFhZGQxYml3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABll+GLisfA=="
}
---

RQ-2: Chat is left off the tab bar entirely until its source is designed, rather than shown as an empty stub — an honest tab bar over a fake one.
***8<***
---
{
  "v": 3,
  "cid": "bafyreigud4gpkl6hpuzlzblgvazejbcqdrhqfajrqr55g43rdrk3e2jqpm",
  "sig": "330145b2309c07e09383ac02a15eeaf7724a02a65051d996411cfa156c3eb3767223828263fd303fc076880b0887b3e7667df0f05cecab6f33fed2ebbc411519",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "tab-restructure"
  },
  "kind": "decision",
  "cites": [
    "bafyreid5s3z65rni6hwqqg6bzd5bsbuygdvwsbq7dg2bsrmhdjabtiht24"
  ],
  "rev": "223mtjsgfuzve",
  "seq": 5,
  "of": 8,
  "text_len": 142,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgfZbz7sWo8e0IG8HI+hkGmDDraQYfGbQZRYcaQBmg89dmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxvdGFiLXJlc3RydWN0dXJlaWFydGlmYWN0c4GhZkNvbW1pdHgoYWM3OTRkOTQ0YmIzNDkzZDk2ZWIxMzQ0ODE3OTZlZGNiOWFhZGQxYml3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABll+GL1+9g=="
}
---

RQ-3: Tab order is `Comments · Ledger · Process` (Comments first) for now; Chat takes the first slot when it lands, per the recorded vision.
***8<***
---
{
  "v": 3,
  "cid": "bafyreia7u6f5pi2rygpqyw3fhhl7yk45i4wetlztdyj27bapvf66m6z4uy",
  "sig": "9f5376dc48a7483a777d6f3680554ef55c3a7a507c191ff9bea501f5c069b1e54ba5c29d529d138d3df210ed61eeb22c47e6a768f8aa6e5770a98bf0bb01602a",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "tab-restructure"
  },
  "kind": "decision",
  "cites": [
    "bafyreihcv7oda3b7rrexnstnsuf3buverf4hcnpfxdiprdqut7632i3mkm"
  ],
  "rev": "223mtjsx6re5n",
  "seq": 6,
  "of": 8,
  "text_len": 1116,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIg4q/cMGw/jEl2ym2VC7DSpIl4cTXluND4jhSf/b0jbFNmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxvdGFiLXJlc3RydWN0dXJlaWFydGlmYWN0c4GhZkNvbW1pdHgoYWM3OTRkOTQ0YmIzNDkzZDk2ZWIxMzQ0ODE3OTZlZGNiOWFhZGQxYml3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABll+OkuixQ=="
}
---

adversarial review of tab-restructure: SHIP. Independent Opus review, all angles pass. Dispatch exhaustive (View = Comments/Ledger/Process, 3 arms each, all reachable); zero leftover Browser/Atoms/Telos in source or tests; digit 4 inert (from_digit->None, no panic). Process sub-pane: the two guarded Left/Right arms order correctly (Comments-guarded first fails its guard in Process and falls through to the Process arm); process_scroll computes max from the active pane and selects atom_scroll/telos_scroll by the same pane with no cross-contamination, clamp correct. Default-Comments startup clean (commented_files populated in new(); refresh_comments runs before first draw; empty-state renders). Ledger = old Browser byte-for-byte (rename only); footer/plain_frame/--once/subject untouched; Atoms/Telos content identical (process_view_lines body unchanged, only re-keyed View->ProcessPane + block titles/legends per REQ-3). Out-of-scope (flowchart/drill-down reshape, Chat) absent. Closed the reviewer's AC-3 note by adding a ProcessPane::Telos assertion. 87 tests, clippy -D warnings, fmt clean. Verdict: SHIP.
***8<***
---
{
  "v": 3,
  "cid": "bafyreigsmjipvosi6ys6qcklwkdxzh3n4t63e73nr3vnlpkyzl72yiaffy",
  "sig": "2e110adbebf7bad850aff6f7cecf3b3b63cd15a939240ad0f399e61a3512ffac61c1a7cb5a63e5136498d3dd8b18d898ca6eceacd585763a1926b0b3867acd28",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "tab-restructure"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mtjsx7325l",
  "seq": 7,
  "of": 8,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxvdGFiLXJlc3RydWN0dXJlaWFydGlmYWN0c4GhZkNvbW1pdHgoYWM3OTRkOTQ0YmIzNDkzZDk2ZWIxMzQ0ODE3OTZlZGNiOWFhZGQxYml3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABll+OlCAAA=="
}
---
