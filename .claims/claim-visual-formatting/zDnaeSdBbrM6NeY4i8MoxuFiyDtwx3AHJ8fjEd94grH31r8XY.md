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
  "of": 9,
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
  "of": 9,
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
  "of": 9,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx3Y2xhaW0tdmlzdWFsLWZvcm1hdHRpbmdpYXJ0aWZhY3RzgaFmQ29tbWl0eCg3MmE5ODdiMjQ5ZWUyYjZmZmQxODNkNzAyODE3NjkxZmIyMmY2ODgxaXdvcmtzcGFjZaFpV29ya3NwYWNleEA4MzVmODdhZGU0MGJkYzhkZjIyYzNiYmQyNzhkNTdlMDQwZmFhMDhjYzZlNDE1ZDZiZDAzODRlYjlhN2U1M2Nla3JlY29yZGVkX2F0GwAGWWbVeVgE"
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreifdn4igwkt6vmhjziq6rsgaove4h4tdhdqyku2tstxtr7u4aufrzy",
  "sig": "b37f7d85154969c2ab8144670812ac588b71f9a2e405a3fa3297dfdc4f8aa4e8438467fa0e4eb0aa3e5a5b0e6a52448e1d5e5797e6d403969f2f6415a3e673ab",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "claim-visual-formatting"
  },
  "kind": "observation",
  "cites": [],
  "rev": "223mtgxjchfy7",
  "seq": 3,
  "of": 9,
  "text_len": 197,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2Fsd2NsYWltLXZpc3VhbC1mb3JtYXR0aW5naWFydGlmYWN0c4GhZkNvbW1pdHgoMGExMjlhYmM4NzM3ZmFmNGU2MmM0MThlMGFhMzkzYTJmOTE5YTRjZWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAODM1Zjg3YWRlNDBiZGM4ZGYyMmMzYmJkMjc4ZDU3ZTA0MGZhYTA4Y2M2ZTQxNWQ2YmQwMzg0ZWI5YTdlNTNjZWtyZWNvcmRlZF9hdBsABllnXoavWA=="
}
---

design doc .design/claim-visual-formatting.md checked against the live design-doc schema: validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s) [doc 5206:f2c26d418c7fcf5f]
***8<***
---
{
  "v": 3,
  "cid": "bafyreifzyeplwshvlvlooqapzuetxgbii2su5hvsycpahwvltdk3st43ja",
  "sig": "61e55beb510a59f3f8010e9288ed1f8cb1d7bd5565f0638448b4e96e8899bc2e7daf29392704ecad9bf35977efdf496a5e6ad26b84e8848c1d7536228a0f7a61",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "claim-visual-formatting"
  },
  "kind": "plan",
  "cites": [
    "bafyreifdn4igwkt6vmhjziq6rsgaove4h4tdhdqyku2tstxtr7u4aufrzy"
  ],
  "rev": "223mtgxjcn2cv",
  "seq": 4,
  "of": 9,
  "text_len": 528,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4HYKlglAAFxEiCjbxBrKn6rDpyiHoyMB1ScPyYzjhhVNTlO84/pwFCxzmZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZVNkQmJyTTZOZVk0aThNb3h1Rml5RHR3eDNBSEo4ZmpFZDk0Z3JIMzFyOFhZZWFnZW509mdzdWJqZWN0oWVMb2NhbHdjbGFpbS12aXN1YWwtZm9ybWF0dGluZ2lhcnRpZmFjdHOBoWZDb21taXR4KDBhMTI5YWJjODczN2ZhZjRlNjJjNDE4ZTBhYTM5M2EyZjkxOWE0Y2Vpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDgzNWY4N2FkZTQwYmRjOGRmMjJjM2JiZDI3OGQ1N2UwNDBmYWEwOGNjNmU0MTVkNmJkMDM4NGViOWE3ZTUzY2VrcmVjb3JkZWRfYXQbAAZZZ16JgKo="
}
---

claim-visual-formatting design (.design/claim-visual-formatting.md): Give the claim browser richer visual structure: color each claim by kind so the eye parses a subject's log fast, and render Retraction claims distinctly — showing what they retract and when. The retracted claim's *content* is not available from kan's live fold, so this is the visible-retraction annotation, with full retracted-content trees left blocked on a kan capability. [validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s)]
***8<***
---
{
  "v": 3,
  "cid": "bafyreics4sikzbazutq5zwjxf5aey36elhvhus5dmctcmy2wge3c3buj4q",
  "sig": "9adcda396de5423cdd5a08b26263d37384f7c399dda4e77305b6bd8f12e342fd36c05ad389c4b5f1ccbc5768ea6ac42b50be6eda4bfa6e99a9721f8851164e0f",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "claim-visual-formatting"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223mtgxjcoukf",
  "seq": 5,
  "of": 9,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXhFRmVhdHVyZTogQ2xhaW0gdmlzdWFsIGZvcm1hdHRpbmcg4oCUIGtpbmQgY29sb3JzICsgcmV0cmFjdGlvbiBkaXNwbGF5bHN1YmplY3Rfa2luZGRJZGVhZWNpdGVzgGZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZVNkQmJyTTZOZVk0aThNb3h1Rml5RHR3eDNBSEo4ZmpFZDk0Z3JIMzFyOFhZZWFnZW509mdzdWJqZWN0oWVMb2NhbHdjbGFpbS12aXN1YWwtZm9ybWF0dGluZ2lhcnRpZmFjdHOBoWZDb21taXR4KDBhMTI5YWJjODczN2ZhZjRlNjJjNDE4ZTBhYTM5M2EyZjkxOWE0Y2Vpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDgzNWY4N2FkZTQwYmRjOGRmMjJjM2JiZDI3OGQ1N2UwNDBmYWEwOGNjNmU0MTVkNmJkMDM4NGViOWE3ZTUzY2VrcmVjb3JkZWRfYXQbAAZZZ16Kaag="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreigd6jwaldgfcdfgcxlg5emiof6m5s5wsnukvv4crztvnphf2alxse",
  "sig": "04620960f7b2a3f1ee485acf98453f8b9dc898bf56e4049c4bfe676c3e61cc254b12314dffc6e59666f37d65fea344b0105107806d80ff8ddf7ba1bee357b18d",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "claim-visual-formatting"
  },
  "kind": "decision",
  "cites": [
    "bafyreifzyeplwshvlvlooqapzuetxgbii2su5hvsycpahwvltdk3st43ja"
  ],
  "rev": "223mtgxjcudsa",
  "seq": 6,
  "of": 9,
  "text_len": 184,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgucEeu0j1XVbnQA/NCTuYKEalTp6ywJ4D2quY1blPm0hmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx3Y2xhaW0tdmlzdWFsLWZvcm1hdHRpbmdpYXJ0aWZhY3RzgaFmQ29tbWl0eCgwYTEyOWFiYzg3MzdmYWY0ZTYyYzQxOGUwYWEzOTNhMmY5MTlhNGNlaXdvcmtzcGFjZaFpV29ya3NwYWNleEA4MzVmODdhZGU0MGJkYzhkZjIyYzNiYmQyNzhkNTdlMDQwZmFhMDhjYzZlNDE1ZDZiZDAzODRlYjlhN2U1M2Nla3JlY29yZGVkX2F0GwAGWWdejSaP"
}
---

RQ-1: Colors use the ANSI 16-color palette keyed by claim kind, so they render on both light and dark terminals without a theme system; the selection highlight still overrides the row.
***8<***
---
{
  "v": 3,
  "cid": "bafyreihocfzdsyoyk6tqeckew4bnnrnujwz5qmhugubf532w774uxn7seq",
  "sig": "8e47f117d689c3d07ebd0bffa6a33dc4142049e743cda369e18de07b0cf012ec4ed19359fe8ced1815ccfd614300d294b73ce33465834656b884dc308857d3a7",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "claim-visual-formatting"
  },
  "kind": "decision",
  "cites": [
    "bafyreifzyeplwshvlvlooqapzuetxgbii2su5hvsycpahwvltdk3st43ja"
  ],
  "rev": "223mtgxjczuin",
  "seq": 7,
  "of": 9,
  "text_len": 283,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgucEeu0j1XVbnQA/NCTuYKEalTp6ywJ4D2quY1blPm0hmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx3Y2xhaW0tdmlzdWFsLWZvcm1hdHRpbmdpYXJ0aWZhY3RzgaFmQ29tbWl0eCgwYTEyOWFiYzg3MzdmYWY0ZTYyYzQxOGUwYWEzOTNhMmY5MTlhNGNlaXdvcmtzcGFjZaFpV29ya3NwYWNleEA4MzVmODdhZGU0MGJkYzhkZjIyYzNiYmQyNzhkNTdlMDQwZmFhMDhjYzZlNDE1ZDZiZDAzODRlYjlhN2U1M2Nla3JlY29yZGVkX2F0GwAGWWdej+lq"
}
---

RQ-2: Retraction is shown as an annotation — `retracts @shortcid` plus the retraction's own timestamp — since kan's live fold does not expose the retracted claim's content; the tree'd retracted-content view is blocked on a kan capability and recorded as such, not attempted here.
***8<***
---
{
  "v": 3,
  "cid": "bafyreidyjifz2jbqac4zp5ej3koc3e3ae336ixjqx7bi2i6o3btfcsikqu",
  "sig": "4649ce111d293ae68884a2488f6b558b070cbce01411114ce2d64a0b8a27395327b0cead153626b5bf0c50cb16c2a9039625cb3c6babfaa42e033f35c7b94429",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "claim-visual-formatting"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mtgxz2xk63",
  "seq": 8,
  "of": 9,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx3Y2xhaW0tdmlzdWFsLWZvcm1hdHRpbmdpYXJ0aWZhY3RzgaFmQ29tbWl0eCgwYTEyOWFiYzg3MzdmYWY0ZTYyYzQxOGUwYWEzOTNhMmY5MTlhNGNlaXdvcmtzcGFjZaFpV29ya3NwYWNleEA4MzVmODdhZGU0MGJkYzhkZjIyYzNiYmQyNzhkNTdlMDQwZmFhMDhjYzZlNDE1ZDZiZDAzODRlYjlhN2U1M2Nla3JlY29yZGVkX2F0GwAGWWd+DsAR"
}
---
