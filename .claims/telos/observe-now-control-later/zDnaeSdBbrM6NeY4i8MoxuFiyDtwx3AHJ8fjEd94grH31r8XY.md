---
{
  "v": 3,
  "cid": "bafyreibmlrvb6zljkt3ixsxymutfkgigatruobxivd62bxbb2tucphhzdy",
  "sig": "796f3b3f6f4fe9c9cbff471daec66652263b13118f2e3597b41f26b56fd30a057dbd321e6f4cbbd5bb21dbc5ff2e1b67604f927bb8b31d4114a7e55160301a23",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "telos/observe-now-control-later"
  },
  "kind": "decision",
  "cites": [],
  "rev": "223mtfaehxxhl",
  "seq": 0,
  "of": 5,
  "text_len": 201,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2FseB90ZWxvcy9vYnNlcnZlLW5vdy1jb250cm9sLWxhdGVyaWFydGlmYWN0c4GhZkNvbW1pdHgoYzMwNWQ5MTlkNzU3ODBhZjI5NDNkN2U4YzEwMDQzMDZhNDExMjIwYWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAODM1Zjg3YWRlNDBiZGM4ZGYyMmMzYmJkMjc4ZDU3ZTA0MGZhYTA4Y2M2ZTQxNWQ2YmQwMzg0ZWI5YTdlNTNjZWtyZWNvcmRlZF9hdBsABllZlN71Qw=="
}
---

cospan ships read-only observation first, leaving a clean command-bus seam for spawn/kill/redirect and claim-writes to arrive later.

```day-telos
{"witnesses":["published-artifact","design-doc"]}
```

***8<***
---
{
  "v": 3,
  "cid": "bafyreiajefjnvry2tdew5fl2wp6k3lvpxl5y3klctu6xs4f3a74fsqmegm",
  "sig": "a68b5245cf8db5e263751bf797e16762839263f7b56eaedd75124de7d97e068526be6e0c4030600b6138e57c98db8ebaa79af90342d19ecd44292e9026272504",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "telos/observe-now-control-later"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223mtfaehyoso",
  "seq": 1,
  "of": 5,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXgaT2JzZXJ2ZSBub3csIGNvbnRyb2wgbGF0ZXJsc3ViamVjdF9raW5kZElkZWFlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2FseB90ZWxvcy9vYnNlcnZlLW5vdy1jb250cm9sLWxhdGVyaWFydGlmYWN0c4GhZkNvbW1pdHgoYzMwNWQ5MTlkNzU3ODBhZjI5NDNkN2U4YzEwMDQzMDZhNDExMjIwYWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAODM1Zjg3YWRlNDBiZGM4ZGYyMmMzYmJkMjc4ZDU3ZTA0MGZhYTA4Y2M2ZTQxNWQ2YmQwMzg0ZWI5YTdlNTNjZWtyZWNvcmRlZF9hdBsABllZlN9Ssw=="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreigpzvwrm55ysdq6rewa2d4y6rnjks6ghq625vo6ntbmizmn5p2kre",
  "sig": "0a5a6a971a9450c89af0b02ad37d4057ec6bddcaebb110d488307ba3732ad7df6025e3a2f40f020a7bd1f079e508a2375784a7b48a121b34954f45d158b087f2",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "telos/observe-now-control-later"
  },
  "kind": "decision",
  "cites": [
    "bafyreiajefjnvry2tdew5fl2wp6k3lvpxl5y3klctu6xs4f3a74fsqmegm"
  ],
  "rev": "223mtfafr3voq",
  "seq": 2,
  "of": 5,
  "text_len": 188,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgCSFS2scamMlulXqz/K2ur7r7jalinT15cLsH+FlBhDNmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx4H3RlbG9zL29ic2VydmUtbm93LWNvbnRyb2wtbGF0ZXJpYXJ0aWZhY3RzgaFmQ29tbWl0eChjMzA1ZDkxOWQ3NTc4MGFmMjk0M2Q3ZThjMTAwNDMwNmE0MTEyMjBhaXdvcmtzcGFjZaFpV29ya3NwYWNleEA4MzVmODdhZGU0MGJkYzhkZjIyYzNiYmQyNzhkNTdlMDQwZmFhMDhjYzZlNDE1ZDZiZDAzODRlYjlhN2U1M2Nla3JlY29yZGVkX2F0GwAGWVmXcO4g"
}
---

cospan ships read-only observation first, leaving a clean command-bus seam for spawn/kill/redirect and claim-writes to arrive later.

```day-telos
{"witnesses":["published-artifact"]}
```

***8<***
---
{
  "v": 3,
  "cid": "bafyreifn7nvyjfh3hpueddvzu7fkysm5tfu3whhwgisv6t7yk7wkr377xm",
  "sig": "44d0461a826b18cc53b9d8d316be1da472e3eb1337753278984f79511cd32f713777b32088ae161bcb944683cae4de5f94878fbd235c61250be0d960d448525a",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "telos/observe-now-control-later"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223mtfafr4oov",
  "seq": 3,
  "of": 5,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXgaT2JzZXJ2ZSBub3csIGNvbnRyb2wgbGF0ZXJsc3ViamVjdF9raW5kZElkZWFlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2FseB90ZWxvcy9vYnNlcnZlLW5vdy1jb250cm9sLWxhdGVyaWFydGlmYWN0c4GhZkNvbW1pdHgoYzMwNWQ5MTlkNzU3ODBhZjI5NDNkN2U4YzEwMDQzMDZhNDExMjIwYWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAODM1Zjg3YWRlNDBiZGM4ZGYyMmMzYmJkMjc4ZDU3ZTA0MGZhYTA4Y2M2ZTQxNWQ2YmQwMzg0ZWI5YTdlNTNjZWtyZWNvcmRlZF9hdBsABllZl3FSLg=="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreihrilwe7qkdmxebor3ydwwj3thl2vtzisgtebdawr54dqsqfkiriy",
  "sig": "619aeb1d49dca18a0c399146d974f582a198154643d42d94a91595decbcb1ac50f093816a3fb1dc6ae095c104a091bb8b33d75ae2d26ebc03785d72a413fce65",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "telos/observe-now-control-later"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mtfaruekd5",
  "seq": 4,
  "of": 5,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx4H3RlbG9zL29ic2VydmUtbm93LWNvbnRyb2wtbGF0ZXJpYXJ0aWZhY3RzgaFmQ29tbWl0eChmMTMzNzY0YzExMjU5ODBiNjBmYmVlZjM1MWQ4M2MxZmNmMDc3MTgzaXdvcmtzcGFjZaFpV29ya3NwYWNleEA4MzVmODdhZGU0MGJkYzhkZjIyYzNiYmQyNzhkNTdlMDQwZmFhMDhjYzZlNDE1ZDZiZDAzODRlYjlhN2U1M2Nla3JlY29yZGVkX2F0GwAGWVmvpUC9"
}
---
