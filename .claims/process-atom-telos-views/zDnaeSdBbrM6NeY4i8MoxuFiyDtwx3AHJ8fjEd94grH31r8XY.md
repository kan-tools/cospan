---
{
  "v": 3,
  "cid": "bafyreihas5ofqn6m7iyj74qgixsxnj2me6u37lvtt5ghy2oca5oqa2clye",
  "sig": "97cc208bd1ab7ae49b87ca6d5f20730549e55938695cbcb17d447986ec3125e638c5cb07a83fcf8ed65a6326916e680944cc02d4a0bc4a1d69d00d8860fcfd0f",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "process-atom-telos-views"
  },
  "kind": "decision",
  "cites": [],
  "rev": "223mtgverb47h",
  "seq": 0,
  "of": 9,
  "text_len": 717,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2FseBhwcm9jZXNzLWF0b20tdGVsb3Mtdmlld3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCg3MmE5ODdiMjQ5ZWUyYjZmZmQxODNkNzAyODE3NjkxZmIyMmY2ODgxaXdvcmtzcGFjZaFpV29ya3NwYWNleEA4MzVmODdhZGU0MGJkYzhkZjIyYzNiYmQyNzhkNTdlMDQwZmFhMDhjYzZlNDE1ZDZiZDAzODRlYjlhN2U1M2Nla3JlY29yZGVkX2F0GwAGWWbVc4hG"
}
---

Two day-process views in the TUI. (1) Atom-layout view: render the day atom DAG (atoms with typed in/out, next edges) plus the current inferred appraisal of where the work sits — the candidate atom(s), satisfied inputs, met/unmet done criteria, and any off-sequence finding. Day reports position as a candidate list, not a cursor — mirror that (honest-ambiguity). (2) Telos view: list the teloi, each telos's witnesses, and each witness's current state (met / unmet / vacuous / not-run), plus recorded tensions between teloi. Related to day-summary-in-cospan (also blocked on machine-readable day status; day status / day assess telos have no --json yet) and the harness-view direction in .dropbox/05-views-ux.md.
***8<***
---
{
  "v": 3,
  "cid": "bafyreifeaqners6ziiyaplotz457hhylxlsfokfmfa2tukjmjceq5f7yze",
  "sig": "0c5555e25dd3b91acf9bfcb9668fd6b324b085afb2abb7189157faafa993a44e527c026ece99374c45eae6f07ebe7b123a5697dd5155f53bb0dea2aa0b8ffb63",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "process-atom-telos-views"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223mtgverclpw",
  "seq": 1,
  "of": 9,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXg2ZGF5IGF0b20tREFHICsgcG9zaXRpb24gdmlldywgYW5kIGEgdGVsb3Mvd2l0bmVzcyB2aWV3bHN1YmplY3Rfa2luZGRJZGVhZWNpdGVzgGZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZVNkQmJyTTZOZVk0aThNb3h1Rml5RHR3eDNBSEo4ZmpFZDk0Z3JIMzFyOFhZZWFnZW509mdzdWJqZWN0oWVMb2NhbHgYcHJvY2Vzcy1hdG9tLXRlbG9zLXZpZXdzaWFydGlmYWN0c4GhZkNvbW1pdHgoNzJhOTg3YjI0OWVlMmI2ZmZkMTgzZDcwMjgxNzY5MWZiMjJmNjg4MWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAODM1Zjg3YWRlNDBiZGM4ZGYyMmMzYmJkMjc4ZDU3ZTA0MGZhYTA4Y2M2ZTQxNWQ2YmQwMzg0ZWI5YTdlNTNjZWtyZWNvcmRlZF9hdBsABllm1XRGWg=="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreiaz33xwlzrdumyrlyr34js6k6m34whsbdkeickbmxmnpxut5uykja",
  "sig": "bfac2bb4d4639cc8949f54142dc34751b66a21380a67b0b8c6d9fd59a4bdf52e5b4b00d1beb79c35a7389d24a47f4f5e7b46117f55f5d2663155bd618bef5de3",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "process-atom-telos-views"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mtgverstti",
  "seq": 2,
  "of": 9,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GHByb2Nlc3MtYXRvbS10ZWxvcy12aWV3c2lhcnRpZmFjdHOBoWZDb21taXR4KDcyYTk4N2IyNDllZTJiNmZmZDE4M2Q3MDI4MTc2OTFmYjIyZjY4ODFpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDgzNWY4N2FkZTQwYmRjOGRmMjJjM2JiZDI3OGQ1N2UwNDBmYWEwOGNjNmU0MTVkNmJkMDM4NGViOWE3ZTUzY2VrcmVjb3JkZWRfYXQbAAZZZtV8Zss="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreidzryveex3yzctmp7ket6gm3vhh2qw5a52pzxsg5jlmci6ja7pqai",
  "sig": "b550532f043b20554d5847a2b46b8b2e6317430909ffea96075783c8d0065c3b7a480bcf40d18170904a9747b1e3ec3cc8e77fe202a2801631780d7fd1967f03",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "process-atom-telos-views"
  },
  "kind": "observation",
  "cites": [],
  "rev": "223mtgxps4r5t",
  "seq": 3,
  "of": 9,
  "text_len": 198,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2FseBhwcm9jZXNzLWF0b20tdGVsb3Mtdmlld3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCgwYTEyOWFiYzg3MzdmYWY0ZTYyYzQxOGUwYWEzOTNhMmY5MTlhNGNlaXdvcmtzcGFjZaFpV29ya3NwYWNleEA4MzVmODdhZGU0MGJkYzhkZjIyYzNiYmQyNzhkNTdlMDQwZmFhMDhjYzZlNDE1ZDZiZDAzODRlYjlhN2U1M2Nla3JlY29yZGVkX2F0GwAGWWdrgVwP"
}
---

design doc .design/process-atom-telos-views.md checked against the live design-doc schema: validation: 9 check(s), 0 failed, 1 warning(s), 0 unchecked, 0 open question(s) [doc 5580:f54709a20ef5ec38]
***8<***
---
{
  "v": 3,
  "cid": "bafyreihxdh2tmn3o76vomn742xmuieiankrl65iigocqsy623ks3jsqyh4",
  "sig": "95778a7e6a2f38077e295a5f322d185c9fad61b7159611d1c50c78fd76ad0d2c6edb9c66c91c128ef68c14a2f0a03fdb4ed5fdac26bd605b6778ad8259e838d7",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "process-atom-telos-views"
  },
  "kind": "plan",
  "cites": [
    "bafyreidzryveex3yzctmp7ket6gm3vhh2qw5a52pzxsg5jlmci6ja7pqai"
  ],
  "rev": "223mtgxpsc36u",
  "seq": 4,
  "of": 9,
  "text_len": 609,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4HYKlglAAFxEiB5jipCX3jIpsf9RJ+MzdTn1C3Qd0/N5G6lbBI8kH3wAmZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZVNkQmJyTTZOZVk0aThNb3h1Rml5RHR3eDNBSEo4ZmpFZDk0Z3JIMzFyOFhZZWFnZW509mdzdWJqZWN0oWVMb2NhbHgYcHJvY2Vzcy1hdG9tLXRlbG9zLXZpZXdzaWFydGlmYWN0c4GhZkNvbW1pdHgoMGExMjlhYmM4NzM3ZmFmNGU2MmM0MThlMGFhMzkzYTJmOTE5YTRjZWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAODM1Zjg3YWRlNDBiZGM4ZGYyMmMzYmJkMjc4ZDU3ZTA0MGZhYTA4Y2M2ZTQxNWQ2YmQwMzg0ZWI5YTdlNTNjZWtyZWNvcmRlZF9hdBsABllna4QEJg=="
}
---

process-atom-telos-views design (.design/process-atom-telos-views.md): Add two top-level views beside the claim browser: an **Atoms** view showing the day atom graph (each atom's in/out types and next edges) and a **Telos** view showing the teloi with their declared witnesses and recorded tensions. Both are parsed from kan claims (the fenced `day-atom`/`day-telos` blocks) — the declared structure. Live position and witness state are deferred, since they need machine-readable day, so cospan does not fake day's inference. [validation: 9 check(s), 0 failed, 1 warning(s), 0 unchecked, 0 open question(s)]
***8<***
---
{
  "v": 3,
  "cid": "bafyreiaqg5dqi5ahchdf2hsbygzckdf3yh4i5xauggjdmbjmzaahthrwte",
  "sig": "6db77287ae192ee981092e4e432c031f2582fa22077c1ed5cd03e0461fcdef37165addd10a370e9351a4ace69feb57f51dd7afb5ae8a32b00edcba9bef9110cc",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "process-atom-telos-views"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223mtgxpsdtvm",
  "seq": 5,
  "of": 9,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXgpRmVhdHVyZTogQXRvbS1EQUcgYW5kIHRlbG9zL3dpdG5lc3Mgdmlld3Nsc3ViamVjdF9raW5kZElkZWFlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2FseBhwcm9jZXNzLWF0b20tdGVsb3Mtdmlld3NpYXJ0aWZhY3RzgaFmQ29tbWl0eCgwYTEyOWFiYzg3MzdmYWY0ZTYyYzQxOGUwYWEzOTNhMmY5MTlhNGNlaXdvcmtzcGFjZaFpV29ya3NwYWNleEA4MzVmODdhZGU0MGJkYzhkZjIyYzNiYmQyNzhkNTdlMDQwZmFhMDhjYzZlNDE1ZDZiZDAzODRlYjlhN2U1M2Nla3JlY29yZGVkX2F0GwAGWWdrhOcE"
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreibnxo3eidjvpzw4bkpgwwy3ihjkkmpvritlm3lb7ugnhy3gurrifi",
  "sig": "8f4751389ca0f1c3b0284fbc43f88fac76925ed08178d7560d97832e03e247ce66a1bc9943f9412b024f76c88610a5f70ccce1f2d05f02e0d161567bc6bd75c3",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "process-atom-telos-views"
  },
  "kind": "decision",
  "cites": [
    "bafyreihxdh2tmn3o76vomn742xmuieiankrl65iigocqsy623ks3jsqyh4"
  ],
  "rev": "223mtgxpsj3hz",
  "seq": 6,
  "of": 9,
  "text_len": 144,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIg9xn1Njdu/6rmN/zV2UQRAGqiv3UIM4UJY9rapbTKGD9mYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GHByb2Nlc3MtYXRvbS10ZWxvcy12aWV3c2lhcnRpZmFjdHOBoWZDb21taXR4KDBhMTI5YWJjODczN2ZhZjRlNjJjNDE4ZTBhYTM5M2EyZjkxOWE0Y2Vpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDgzNWY4N2FkZTQwYmRjOGRmMjJjM2JiZDI3OGQ1N2UwNDBmYWEwOGNjNmU0MTVkNmJkMDM4NGViOWE3ZTUzY2VrcmVjb3JkZWRfYXQbAAZZZ2uHhVM="
}
---

RQ-1: Views switch with `1`/`2`/`3` (direct) and `Tab` (cycle); the browser keeps its subjects/claims/detail navigation within the Browser view.
***8<***
---
{
  "v": 3,
  "cid": "bafyreie67ayou5g6sm7ndyd2bnmh3xbr6nowvplnvgvlyctqi66gx3uwwu",
  "sig": "279223a96fe457aa36b589196935dd22120426056fd6b4e62d6521d7f49906fd4d881b110fe38a401c992cb17e8a20c129e7047c7e0ee94ee4afebce73cf95f5",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "process-atom-telos-views"
  },
  "kind": "decision",
  "cites": [
    "bafyreihxdh2tmn3o76vomn742xmuieiankrl65iigocqsy623ks3jsqyh4"
  ],
  "rev": "223mtgxpsofia",
  "seq": 7,
  "of": 9,
  "text_len": 230,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIg9xn1Njdu/6rmN/zV2UQRAGqiv3UIM4UJY9rapbTKGD9mYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GHByb2Nlc3MtYXRvbS10ZWxvcy12aWV3c2lhcnRpZmFjdHOBoWZDb21taXR4KDBhMTI5YWJjODczN2ZhZjRlNjJjNDE4ZTBhYTM5M2EyZjkxOWE0Y2Vpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDgzNWY4N2FkZTQwYmRjOGRmMjJjM2JiZDI3OGQ1N2UwNDBmYWEwOGNjNmU0MTVkNmJkMDM4NGViOWE3ZTUzY2VrcmVjb3JkZWRfYXQbAAZZZ2uKLVY="
}
---

RQ-2: The atom and telos views render the declared structure parsed from the `day-atom`/`day-telos` blocks in kan; live position and witness state are deferred and labeled as needing machine-readable day, not re-derived in cospan.
***8<***
---
{
  "v": 3,
  "cid": "bafyreifsy6r36uwvf6yrgte4ccsgdqp3cmhyfggnjuwu4elpck3j5g7uqm",
  "sig": "d04e65b6e0dcbee967f6ef854b0dfa026d7c3be8fc21385e6a4e5d10e99d866047424dbe788df3a1ac564f90f64dba8a2575602e838961de16aa27bb551c8085",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "process-atom-telos-views"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mtgxz373x4",
  "seq": 8,
  "of": 9,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GHByb2Nlc3MtYXRvbS10ZWxvcy12aWV3c2lhcnRpZmFjdHOBoWZDb21taXR4KDBhMTI5YWJjODczN2ZhZjRlNjJjNDE4ZTBhYTM5M2EyZjkxOWE0Y2Vpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDgzNWY4N2FkZTQwYmRjOGRmMjJjM2JiZDI3OGQ1N2UwNDBmYWEwOGNjNmU0MTVkNmJkMDM4NGViOWE3ZTUzY2VrcmVjb3JkZWRfYXQbAAZZZ34Shz8="
}
---
