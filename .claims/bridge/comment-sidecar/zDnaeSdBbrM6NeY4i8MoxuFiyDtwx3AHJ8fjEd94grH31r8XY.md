---
{
  "v": 3,
  "cid": "bafyreih5twb4muaedw4qugyxkifpxkc7fu25k4fvdn2pcqrmnz4mdsu6za",
  "sig": "66ce8c8487bc9b9f2a9a0f554d016f852d4e2e706b99e69bbc5db8fa88a1361937bbda0a3bbde319383aaba808e3139bebae74371b5dc2d889f25ad1f3a0f0b3",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "bridge/comment-sidecar"
  },
  "kind": "observation",
  "cites": [],
  "rev": "223mtgucqvh6n",
  "seq": 0,
  "of": 2,
  "text_len": 259,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2FsdmJyaWRnZS9jb21tZW50LXNpZGVjYXJpYXJ0aWZhY3RzgaFmQ29tbWl0eCg3MmE5ODdiMjQ5ZWUyYjZmZmQxODNkNzAyODE3NjkxZmIyMmY2ODgxaXdvcmtzcGFjZaFpV29ya3NwYWNleEA4MzVmODdhZGU0MGJkYzhkZjIyYzNiYmQyNzhkNTdlMDQwZmFhMDhjYzZlNDE1ZDZiZDAzODRlYjlhN2U1M2Nla3JlY29yZGVkX2F0GwAGWWaRbbQi"
}
---

Step 4: build the ephemeral comment sidecar + live re-localization, then review it against the round-trip telos.

```day-bridge
{"telos":"comment-roundtrip","have":["design-doc"],"plan":{"seq":[{"atom":"generative-build"},{"atom":"adversarial-review"}]}}
```

***8<***
---
{
  "v": 3,
  "cid": "bafyreicrspdhnt3ijwrzy2pwf5qdqhvgtlpq3qpffqwhgea6pujmtip76u",
  "sig": "89ad6f7e939dccb154a8326742f6dd5e77130136d1e7f7b3970b4b1cc0b92d27408e270bd77969d40d5b9878a6eebc8b588bcdacdb844439335b2d3e8a4db481",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "bridge/comment-sidecar"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mtgucrjw4b",
  "seq": 1,
  "of": 2,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx2YnJpZGdlL2NvbW1lbnQtc2lkZWNhcmlhcnRpZmFjdHOBoWZDb21taXR4KDcyYTk4N2IyNDllZTJiNmZmZDE4M2Q3MDI4MTc2OTFmYjIyZjY4ODFpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDgzNWY4N2FkZTQwYmRjOGRmMjJjM2JiZDI3OGQ1N2UwNDBmYWEwOGNjNmU0MTVkNmJkMDM4NGViOWE3ZTUzY2VrcmVjb3JkZWRfYXQbAAZZZpF379o="
}
---
