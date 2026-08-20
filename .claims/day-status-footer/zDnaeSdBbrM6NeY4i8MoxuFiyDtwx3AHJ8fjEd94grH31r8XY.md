---
{
  "v": 3,
  "cid": "bafyreie2u2osut7t5bcosutertivyqv3nhfvlbzvkb3lm6zxgsfdtaibo4",
  "sig": "4430ac1d7d0decdf0883372ca96dccd7bf5b25fbafe2d0c5d474347d5cc32ce50fede209485efa5f533d5fde15081ad58beaf27976b2e7178cce35f6f6ef7fe6",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "day-status-footer"
  },
  "kind": "observation",
  "cites": [],
  "rev": "223mtji2clkhu",
  "seq": 0,
  "of": 8,
  "text_len": 192,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2FscWRheS1zdGF0dXMtZm9vdGVyaWFydGlmYWN0c4GhZkNvbW1pdHgoZTM2M2VjZmI3MWVjNzRjZDlkNjFlODAyYWRjODBlZGEzODc1MGU1N2l3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABll7gIi9rg=="
}
---

design doc .design/day-status-footer.md checked against the live design-doc schema: validation: 10 check(s), 0 failed, 2 warning(s), 0 unchecked, 0 open question(s) [doc 5713:51e2f313a16b7503]
***8<***
---
{
  "v": 3,
  "cid": "bafyreieapks3nchsnfa772pbf4mckrl3qzvlawe6oot2b3qalxf3gzikqi",
  "sig": "6b03512b74186d1704e881abfce513568e0f934aa33b1003f795a54730e5c3ec22de86eb111818bd2e9e2dbf75795640d105d1b68bdb9933741f46dcadea7861",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "day-status-footer"
  },
  "kind": "plan",
  "cites": [
    "bafyreie2u2osut7t5bcosutertivyqv3nhfvlbzvkb3lm6zxgsfdtaibo4"
  ],
  "rev": "223mtji2cuqul",
  "seq": 1,
  "of": 8,
  "text_len": 533,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4HYKlglAAFxEiCapp0qT/PoROlSZIzRXEK7actVhzVQdrZ7NzSKOYEBd2ZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZVNkQmJyTTZOZVk0aThNb3h1Rml5RHR3eDNBSEo4ZmpFZDk0Z3JIMzFyOFhZZWFnZW509mdzdWJqZWN0oWVMb2NhbHFkYXktc3RhdHVzLWZvb3RlcmlhcnRpZmFjdHOBoWZDb21taXR4KGUzNjNlY2ZiNzFlYzc0Y2Q5ZDYxZTgwMmFkYzgwZWRhMzg3NTBlNTdpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZZe4CNWts="
}
---

day-status-footer design (.design/day-status-footer.md): Replace the top `day status` process panel with a thin, always-on **footer** at the bottom of the TUI that renders day's own status line — the exact compact text Claude Code shows — sourced from day's status-line cache and width-matched to the footer. This fixes the mis-sourced top panel (it currently scrapes the verbose `day status`) and frees the top of the screen for the active view. [validation: 10 check(s), 0 failed, 2 warning(s), 0 unchecked, 0 open question(s)]
***8<***
---
{
  "v": 3,
  "cid": "bafyreiggh33u6hc7dy7qvibbgjt25ivpq35mc3wxmvfduogbrphig6h4q4",
  "sig": "8fc0264a1eadff2105e503d53953d6041b118954224d4f2cd2f81e1f38e29646773f9b3d8c8d67977a5ad88dd61583268d5efd096589349dd8df8fd41b84187d",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "day-status-footer"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223mtji2cxsfr",
  "seq": 2,
  "of": 8,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXgjRmVhdHVyZTogdGhlIGRheSBzdGF0dXMtbGluZSBmb290ZXJsc3ViamVjdF9raW5kZElkZWFlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2FscWRheS1zdGF0dXMtZm9vdGVyaWFydGlmYWN0c4GhZkNvbW1pdHgoZTM2M2VjZmI3MWVjNzRjZDlkNjFlODAyYWRjODBlZGEzODc1MGU1N2l3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABll7gI7hEA=="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreifmeneahywwovkdcjbs7sofivexwythzxc4sopas4jba6yolu4n5m",
  "sig": "3fc10fd37e092fbbdb1ee64640344f8f1f1adef177f9d9c4fabd8ab81bf6d14c5d20701fdc26be2065450ef0b36e9102680e3c4a912345d83c2101b4a1a06006",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "day-status-footer"
  },
  "kind": "decision",
  "cites": [
    "bafyreieapks3nchsnfa772pbf4mckrl3qzvlawe6oot2b3qalxf3gzikqi"
  ],
  "rev": "223mtji2dap7n",
  "seq": 3,
  "of": 8,
  "text_len": 372,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIggHqltojyaUH/6eEvGCVFe4ZqsFiec6eg7gBdy7NlCoJmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxxZGF5LXN0YXR1cy1mb290ZXJpYXJ0aWZhY3RzgaFmQ29tbWl0eChlMzYzZWNmYjcxZWM3NGNkOWQ2MWU4MDJhZGM4MGVkYTM4NzUwZTU3aXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWXuAk1RC"
}
---

RQ-1: The footer reads day's variants cache file (for width-matching) rather than only shelling `day status-line` (which emits one variant), with `day status-line` as the fallback — so cospan picks the right width without losing a stable CLI backstop. The cache is an artifact day publishes for consumers, like `.kan/log` (`telos/kan-is-truth`'s substrate-read pattern).
***8<***
---
{
  "v": 3,
  "cid": "bafyreihn3ajc7bqr2d34jvbtt7wk4wrvw2aksmo5yru2a75finxihz4w5m",
  "sig": "bb307f12467dea83f95dfa8b042e9d9db0c0f5de00e7a099666dfbadfe6dfcbc3c40708cde2b13f94d9c0a93db0de62f86d698794cd36dc9b5a3433d8331857e",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "day-status-footer"
  },
  "kind": "decision",
  "cites": [
    "bafyreieapks3nchsnfa772pbf4mckrl3qzvlawe6oot2b3qalxf3gzikqi"
  ],
  "rev": "223mtji2djpxk",
  "seq": 4,
  "of": 8,
  "text_len": 150,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIggHqltojyaUH/6eEvGCVFe4ZqsFiec6eg7gBdy7NlCoJmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxxZGF5LXN0YXR1cy1mb290ZXJpYXJ0aWZhY3RzgaFmQ29tbWl0eChlMzYzZWNmYjcxZWM3NGNkOWQ2MWU4MDJhZGM4MGVkYTM4NzUwZTU3aXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWXuAl9c3"
}
---

RQ-2: The status moves to the bottom as a thin footer (not a top panel and not a tab), matching a status bar's conventional place and freeing the top.
***8<***
---
{
  "v": 3,
  "cid": "bafyreicr5aryg67biinl2nm2ukap6mtbu4esibsqp5rao6p5eqijv6nmie",
  "sig": "cd32537c578be0c15861a8f87ad1e83d290102951389e5b4412e40d602c5dbdb52a2a1cb3ec5a35933152a1af47f415d33132a777c2e3a367f28eb6e22d31333",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "day-status-footer"
  },
  "kind": "decision",
  "cites": [
    "bafyreieapks3nchsnfa772pbf4mckrl3qzvlawe6oot2b3qalxf3gzikqi"
  ],
  "rev": "223mtji2dsrmt",
  "seq": 5,
  "of": 8,
  "text_len": 188,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIggHqltojyaUH/6eEvGCVFe4ZqsFiec6eg7gBdy7NlCoJmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxxZGF5LXN0YXR1cy1mb290ZXJpYXJ0aWZhY3RzgaFmQ29tbWl0eChlMzYzZWNmYjcxZWM3NGNkOWQ2MWU4MDJhZGM4MGVkYTM4NzUwZTU3aXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWXuAnF3t"
}
---

RQ-3: Emoji vs plain: cospan requests `emoji` by default (its other views use Unicode already); a future setting can select `plain`. Width comes from the footer's own width at render time.
***8<***
---
{
  "v": 3,
  "cid": "bafyreid2cal2zpdhnvebiw6oepy2qnf65veq35xbhyelzx6n3gmwruwufe",
  "sig": "ff813217b057d972e76f1ab7428f2939ad318d64137da050557d43910f5868d606a49bf731d7bdaabc859630448b420dd8ec5fa7ee01bfc065c18c441cf7a7d5",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "day-status-footer"
  },
  "kind": "decision",
  "cites": [
    "bafyreifmeneahywwovkdcjbs7sofivexwythzxc4sopas4jba6yolu4n5m"
  ],
  "rev": "223mtjikhsjpj",
  "seq": 6,
  "of": 8,
  "text_len": 1143,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgrCNIA+LWdVQxJDL8nFRUl7YmfNxck54JcSEHsOXTjetmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxxZGF5LXN0YXR1cy1mb290ZXJpYXJ0aWZhY3RzgaFmQ29tbWl0eChlMzYzZWNmYjcxZWM3NGNkOWQ2MWU4MDJhZGM4MGVkYTM4NzUwZTU3aXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWXug3D5C"
}
---

adversarial review of day-status-footer: SHIP. Independent Opus review traced pick_variant against the real cache (emoji 43, plain 44, plain 57): emoji@50->emoji43, plain@50->plain44 (widest<=50, 57 excluded), emoji@40->narrowest emoji43; only-plain-when-emoji-requested falls back to any; empty/garbage->None; non-numeric width parses to 0 and stays selectable; content before first header dropped. Fallback chain + poll discipline sound: after first call state.footer is non-empty so the (width+mtime) gate latches — day status-line is shelled only on a width or cache-mtime change, no per-tick spawn; missing cache + failing day -> explicit unavailable line, gate holds. Layout header/body/footer can't panic (constraint solver clips), body keeps Min(3), all four views render, fold errors prepended to the footer (honest-ambiguity), footer_h clamp(1,6) never truncates the realistic 5-line worst case. No regressions: Fold::day_status still used by plain_frame/--once, removed Block import left no clippy warning, out-of-scope (Process reshape, tab rename, Chat, recompute) absent. 84 tests, clippy -D warnings, fmt clean. Verdict: SHIP.
***8<***
---
{
  "v": 3,
  "cid": "bafyreiddn5f56zglybzmps2iricl5vgtwcir2ktlvn3qpoo4wqdgkciqpa",
  "sig": "67c5bfe266426936a56ee95f4167a1442bf37b54cf15fea4c91bd7fe10e256984a68a119a81c605c6ff477c72e46cee8052032388a538dcaae65c48ef06f7777",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "day-status-footer"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mtjiki3e57",
  "seq": 7,
  "of": 8,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxxZGF5LXN0YXR1cy1mb290ZXJpYXJ0aWZhY3RzgaFmQ29tbWl0eChlMzYzZWNmYjcxZWM3NGNkOWQ2MWU4MDJhZGM4MGVkYTM4NzUwZTU3aXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWXug4Kf9"
}
---
