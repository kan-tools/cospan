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
  "of": 3,
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
  "of": 3,
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
  "of": 3,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx4GHByb2Nlc3MtYXRvbS10ZWxvcy12aWV3c2lhcnRpZmFjdHOBoWZDb21taXR4KDcyYTk4N2IyNDllZTJiNmZmZDE4M2Q3MDI4MTc2OTFmYjIyZjY4ODFpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDgzNWY4N2FkZTQwYmRjOGRmMjJjM2JiZDI3OGQ1N2UwNDBmYWEwOGNjNmU0MTVkNmJkMDM4NGViOWE3ZTUzY2VrcmVjb3JkZWRfYXQbAAZZZtV8Zss="
}
---
