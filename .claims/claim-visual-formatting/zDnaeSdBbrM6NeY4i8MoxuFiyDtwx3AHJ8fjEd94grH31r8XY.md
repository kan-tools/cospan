---
{
  "v": 3,
  "cid": "bafyreiddl3yyp5sqa3dassib7hjgmhx2mn4ybytervcdbqhxvamorpj5em",
  "sig": "40a39fdd14703ed41916aed1a32ee04440b6b5eb2008009aa13a87fc62023a8c78d144db6d5745b12758498ac08ac922cb340416669e94da6cc9ff4526e4fdfb",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "claim-visual-formatting"
  },
  "kind": "decision",
  "cites": [],
  "rev": "223mtgver3jhk",
  "seq": 0,
  "of": 3,
  "text_len": 757,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2Fsd2NsYWltLXZpc3VhbC1mb3JtYXR0aW5naWFydGlmYWN0c4GhZkNvbW1pdHgoNzJhOTg3YjI0OWVlMmI2ZmZkMTgzZDcwMjgxNzY5MWZiMjJmNjg4MWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAODM1Zjg3YWRlNDBiZGM4ZGYyMmMzYmJkMjc4ZDU3ZTA0MGZhYTA4Y2M2ZTQxNWQ2YmQwMzg0ZWI5YTdlNTNjZWtyZWNvcmRlZF9hdBsABllm1XC9Pw=="
}
---

Richer visual formatting for kan claims in the TUI. Two concrete asks from the user: (1) Retracted claims: render them distinctly and GROUP a retracted claim with its retracting claim, the retracted one indented/tree'd underneath, so both the claim and the time of its retraction are visible together rather than the retraction floating separately. (2) Color: tint claim content by schema — e.g. day-telos / day-atom / day-bridge / day-witness blocks and claim kinds get schema-driven colors so the eye parses structure fast. Serves honest-ambiguity (retractions are visible, not silently folded away) and kan-is-truth (the render reflects the log's real structure). Depends on cospan reading retraction edges and the schema/blocks + day block vocabulary.
***8<***
---
{
  "v": 3,
  "cid": "bafyreighpl7iztfxf5ohqz735bacvxnhyhzbtjqzi5c3sp364i4rfluo2u",
  "sig": "1ea21c3dee698e2420efff715391b929df05522180a576228029d980885debfc3c8dc5df34ea8c4a910bd35f9072cd3e60628391a0fff61b0948612c6ba0b9b7",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "claim-visual-formatting"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223mtgver4ygq",
  "seq": 1,
  "of": 3,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXg5UmljaGVyIGNsYWltIGZvcm1hdHRpbmc6IHJldHJhY3Rpb24gdHJlZXMgKyBzY2hlbWEgY29sb3JzbHN1YmplY3Rfa2luZGRJZGVhZWNpdGVzgGZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZVNkQmJyTTZOZVk0aThNb3h1Rml5RHR3eDNBSEo4ZmpFZDk0Z3JIMzFyOFhZZWFnZW509mdzdWJqZWN0oWVMb2NhbHdjbGFpbS12aXN1YWwtZm9ybWF0dGluZ2lhcnRpZmFjdHOBoWZDb21taXR4KDcyYTk4N2IyNDllZTJiNmZmZDE4M2Q3MDI4MTc2OTFmYjIyZjY4ODFpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDgzNWY4N2FkZTQwYmRjOGRmMjJjM2JiZDI3OGQ1N2UwNDBmYWEwOGNjNmU0MTVkNmJkMDM4NGViOWE3ZTUzY2VrcmVjb3JkZWRfYXQbAAZZZtVxeTQ="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreicsbdymqed5ega6nzcyyha7k447adwm5lbvqhsfamwhpeagzo7lyq",
  "sig": "1e16b1843ad66a7b0bcd73be8fc7c839e82e029b05e9911b0df67cf3c1397b575d2e544596cff45ff4b1263fca081000ae55d22c8a7fa8da91747b889d932d00",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "claim-visual-formatting"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mtgvermq5b",
  "seq": 2,
  "of": 3,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx3Y2xhaW0tdmlzdWFsLWZvcm1hdHRpbmdpYXJ0aWZhY3RzgaFmQ29tbWl0eCg3MmE5ODdiMjQ5ZWUyYjZmZmQxODNkNzAyODE3NjkxZmIyMmY2ODgxaXdvcmtzcGFjZaFpV29ya3NwYWNleEA4MzVmODdhZGU0MGJkYzhkZjIyYzNiYmQyNzhkNTdlMDQwZmFhMDhjYzZlNDE1ZDZiZDAzODRlYjlhN2U1M2Nla3JlY29yZGVkX2F0GwAGWWbVeVgE"
}
---
