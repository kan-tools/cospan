---
{
  "v": 3,
  "cid": "bafyreibdhciuuyvvaldepfasx6r7rfv366nbspknwmxwkuhb4j72gtjcvm",
  "sig": "8b966a66b7d24499c5c53addde51e7cc863712891922eff119140abfcd949852574e1b78f13d749de6d1468e0294ea17be0138b3f27de7edd3a0033cddce2cc7",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "subject-claim-drill-in"
  },
  "kind": "observation",
  "cites": [],
  "rev": "223mtfc7n5z5z",
  "seq": 0,
  "of": 12,
  "text_len": 196,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2FsdnN1YmplY3QtY2xhaW0tZHJpbGwtaW5pYXJ0aWZhY3RzgaFmQ29tbWl0eChmMTMzNzY0YzExMjU5ODBiNjBmYmVlZjM1MWQ4M2MxZmNmMDc3MTgzaXdvcmtzcGFjZaFpV29ya3NwYWNleEA4MzVmODdhZGU0MGJkYzhkZjIyYzNiYmQyNzhkNTdlMDQwZmFhMDhjYzZlNDE1ZDZiZDAzODRlYjlhN2U1M2Nla3JlY29yZGVkX2F0GwAGWVoLMfwV"
}
---

design doc .design/subject-claim-drill-in.md checked against the live design-doc schema: validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s) [doc 6703:804cad4226c38f7d]
***8<***
---
{
  "v": 3,
  "cid": "bafyreifurtcpijxpsmlnphyrtbuw36s4vlesinsxr35ucarx7fiyo3o6iu",
  "sig": "eec3e8dec5d96ae6c76287f3029e79cd4d0f104f26301b95debe5848c0e702f111608a07452d468870ba316594c0e5863afda49d7f3aa504381c7b6f060b18a7",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "subject-claim-drill-in"
  },
  "kind": "plan",
  "cites": [
    "bafyreibdhciuuyvvaldepfasx6r7rfv366nbspknwmxwkuhb4j72gtjcvm"
  ],
  "rev": "223mtfc7navyj",
  "seq": 1,
  "of": 12,
  "text_len": 527,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4HYKlglAAFxEiAjOJFKYrUCxkeUEr+j+Ja795oZPU2zL2VQ4eJ/o00iq2ZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZVNkQmJyTTZOZVk0aThNb3h1Rml5RHR3eDNBSEo4ZmpFZDk0Z3JIMzFyOFhZZWFnZW509mdzdWJqZWN0oWVMb2NhbHZzdWJqZWN0LWNsYWltLWRyaWxsLWluaWFydGlmYWN0c4GhZkNvbW1pdHgoZjEzMzc2NGMxMTI1OTgwYjYwZmJlZWYzNTFkODNjMWZjZjA3NzE4M2l3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAODM1Zjg3YWRlNDBiZGM4ZGYyMmMzYmJkMjc4ZDU3ZTA0MGZhYTA4Y2M2ZTQxNWQ2YmQwMzg0ZWI5YTdlNTNjZWtyZWNvcmRlZF9hdBsABllaCzNvZg=="
}
---

subject-claim-drill-in design (.design/subject-claim-drill-in.md): Add a read-only `cospan subject <repo> <subject>` command that lists one kan subject's live claims — the natural drill-in from the P0 dashboard's subjects-by-namespace summary into the actual claims a subject holds. It extends the proven shell-and-fold spine with zero new dependencies and is the core fold the claims browser will reuse in the TUI. It serves `telos/p0-spine`. [validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s)]
***8<***
---
{
  "v": 3,
  "cid": "bafyreigrspzcp7jbiax2sxxbmlrak2cgwvvrqvpbatiywvqplhrkjctz4i",
  "sig": "386f1fc09eb699d5bb16571b30e3d00f2ed4a14d6bde20673bdfc3763120c4c32ef38a7e22c53c9c833d1a8eb3f762fada5009a5f1a275cc90995be4e0aeb8f0",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "subject-claim-drill-in"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223mtfc7nbvaf",
  "seq": 2,
  "of": 12,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXgjRmVhdHVyZTogUGVyLXN1YmplY3QgY2xhaW0gZHJpbGwtaW5sc3ViamVjdF9raW5kZElkZWFlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2FsdnN1YmplY3QtY2xhaW0tZHJpbGwtaW5pYXJ0aWZhY3RzgaFmQ29tbWl0eChmMTMzNzY0YzExMjU5ODBiNjBmYmVlZjM1MWQ4M2MxZmNmMDc3MTgzaXdvcmtzcGFjZaFpV29ya3NwYWNleEA4MzVmODdhZGU0MGJkYzhkZjIyYzNiYmQyNzhkNTdlMDQwZmFhMDhjYzZlNDE1ZDZiZDAzODRlYjlhN2U1M2Nla3JlY29yZGVkX2F0GwAGWVoLM+xj"
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreigmlir66hyn5afaahdhbbenkumr4mlyesoolvggdnvgmxljy2y7na",
  "sig": "897089629818fa7c8f0eac97986743b986b4e9e2a25ec766f06db332c8e26d4c0156c988c68921b90d0b08be36477d754c85a62ab86edda32049c5de64a32f89",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "subject-claim-drill-in"
  },
  "kind": "decision",
  "cites": [
    "bafyreifurtcpijxpsmlnphyrtbuw36s4vlesinsxr35ucarx7fiyo3o6iu"
  ],
  "rev": "223mtfc7ngbmx",
  "seq": 3,
  "of": 12,
  "text_len": 218,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgtIzE9CbvkxbXnxGYaW36XKrJJDZXjvtBAjf5UYdt3kVmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx2c3ViamVjdC1jbGFpbS1kcmlsbC1pbmlhcnRpZmFjdHOBoWZDb21taXR4KGYxMzM3NjRjMTEyNTk4MGI2MGZiZWVmMzUxZDgzYzFmY2YwNzcxODNpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDgzNWY4N2FkZTQwYmRjOGRmMjJjM2JiZDI3OGQ1N2UwNDBmYWEwOGNjNmU0MTVkNmJkMDM4NGViOWE3ZTUzY2VrcmVjb3JkZWRfYXQbAAZZWgs2He4="
}
---

RQ-1: Ordering is newest-first by `recorded_at`, tie-broken by `cid` — no `rev` field exists in kan 0.13.0-beta.1, so chronological-descending is the deterministic choice and matches "what does this subject say now".
***8<***
---
{
  "v": 3,
  "cid": "bafyreidqvo5b23p7ea7kszyskpfbxf5gwoq5prfsdqawdigxcefookd5ja",
  "sig": "72b19163c509864753c2a0129d6e3448e3d8cbeca69d4c1545cb3d5dc53bf77a59b150a9d2bd9e83d691d730a3b85233b4f44db074cf0a4f9e7834d7dfc0e093",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "subject-claim-drill-in"
  },
  "kind": "decision",
  "cites": [
    "bafyreifurtcpijxpsmlnphyrtbuw36s4vlesinsxr35ucarx7fiyo3o6iu"
  ],
  "rev": "223mtfc7nj6mb",
  "seq": 4,
  "of": 12,
  "text_len": 210,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgtIzE9CbvkxbXnxGYaW36XKrJJDZXjvtBAjf5UYdt3kVmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx2c3ViamVjdC1jbGFpbS1kcmlsbC1pbmlhcnRpZmFjdHOBoWZDb21taXR4KGYxMzM3NjRjMTEyNTk4MGI2MGZiZWVmMzUxZDgzYzFmY2YwNzcxODNpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDgzNWY4N2FkZTQwYmRjOGRmMjJjM2JiZDI3OGQ1N2UwNDBmYWEwOGNjNmU0MTVkNmJkMDM4NGViOWE3ZTUzY2VrcmVjb3JkZWRfYXQbAAZZWgs3kd8="
}
---

RQ-2: Every live claim is shown with a kind-aware summary (text, else title, else a kind label), rather than filtering to narrative kinds — the drill-in should not hide that a subject was titled or published.
***8<***
---
{
  "v": 3,
  "cid": "bafyreibaibbwo54edixjnlm3c522yv5etl2bj2u6q7xkg5uhs6qoti7gsi",
  "sig": "509c1dbba202c569fb609b57f665e1ad2a09882d29677cc30312a6edfab3b718370701539fdb33ae464d7357576280e957b3c392c31de17fd08e88f949441976",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "subject-claim-drill-in"
  },
  "kind": "decision",
  "cites": [
    "bafyreifurtcpijxpsmlnphyrtbuw36s4vlesinsxr35ucarx7fiyo3o6iu"
  ],
  "rev": "223mtfc7nlzt3",
  "seq": 5,
  "of": 12,
  "text_len": 247,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgtIzE9CbvkxbXnxGYaW36XKrJJDZXjvtBAjf5UYdt3kVmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx2c3ViamVjdC1jbGFpbS1kcmlsbC1pbmlhcnRpZmFjdHOBoWZDb21taXR4KGYxMzM3NjRjMTEyNTk4MGI2MGZiZWVmMzUxZDgzYzFmY2YwNzcxODNpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDgzNWY4N2FkZTQwYmRjOGRmMjJjM2JiZDI3OGQ1N2UwNDBmYWEwOGNjNmU0MTVkNmJkMDM4NGViOWE3ZTUzY2VrcmVjb3JkZWRfYXQbAAZZWgs4/rM="
}
---

RQ-3: The query is per-subject `kan show <subject> --json`, matching Step 1's spec and the existing per-command shell-out; the `kan#181` O(n^2) cost is accepted for a single small P0 subject and revisited when the TUI folds many subjects per tick.
***8<***
---
{
  "v": 3,
  "cid": "bafyreihz5vuosjayp7i4opxgc7wy73mxo436vxgoxfqmhumlluvn2ursd4",
  "sig": "dcaff2eac2a8f41f702697f0d62cb29a2d8e252ced748669c169a0d697e1316351e4f2cf26628f459f37ca9572b6da77f59035525fd9b2060b7edd7f7a044197",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "subject-claim-drill-in"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mtfcad5dey",
  "seq": 6,
  "of": 12,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx2c3ViamVjdC1jbGFpbS1kcmlsbC1pbmlhcnRpZmFjdHOBoWZDb21taXR4KGYxMzM3NjRjMTEyNTk4MGI2MGZiZWVmMzUxZDgzYzFmY2YwNzcxODNpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDgzNWY4N2FkZTQwYmRjOGRmMjJjM2JiZDI3OGQ1N2UwNDBmYWEwOGNjNmU0MTVkNmJkMDM4NGViOWE3ZTUzY2VrcmVjb3JkZWRfYXQbAAZZWgyRpPs="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreiflsvmy6c3jdbgld6xacr52ge5tun3wzmqe47sytpewklrgyovabi",
  "sig": "8a86d63948abca1ae9f8f3f1ffbc8451ad49c23ab19ed68f4d085c7e4210ff4a6bd2b356a7e9b6eefbcf588c55f83d1a6580d219f91980ed5509156fe9dd92cc",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "subject-claim-drill-in"
  },
  "kind": "decision",
  "cites": [
    "bafyreifurtcpijxpsmlnphyrtbuw36s4vlesinsxr35ucarx7fiyo3o6iu"
  ],
  "rev": "223mtfe73un2j",
  "seq": 7,
  "of": 12,
  "text_len": 487,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgtIzE9CbvkxbXnxGYaW36XKrJJDZXjvtBAjf5UYdt3kVmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx2c3ViamVjdC1jbGFpbS1kcmlsbC1pbmlhcnRpZmFjdHOBoWZDb21taXR4KGYxMzM3NjRjMTEyNTk4MGI2MGZiZWVmMzUxZDgzYzFmY2YwNzcxODNpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDgzNWY4N2FkZTQwYmRjOGRmMjJjM2JiZDI3OGQ1N2UwNDBmYWEwOGNjNmU0MTVkNmJkMDM4NGViOWE3ZTUzY2VrcmVjb3JkZWRfYXQbAAZZWoodS6w="
}
---

adversarial review of subject-claim-drill-in: APPROVE-WITH-FOLLOW-UPS — adversarial review of subject-claim-drill-in Step 1: correct and shippable — fmt_utc verified against epoch/negative/leap/century cases, sort deterministic, kan-is-truth honored, empty-fold message honest; follow-ups: cid tie-break and calendar math are under-tested, missing recorded_at silently renders 1970, short_author truncation can collide, and the design file diverged from its recorded validation hash.
***8<***
---
{
  "v": 3,
  "cid": "bafyreifvg4co5zs7226p2zw4h3ktd2kdkpzk6kzi522y4lqfqyj3h3r3ry",
  "sig": "111b11b459ef0aa46db62fdb2f403fa367aaeec517ad0862f1db1e069a60fa83069cfc2ac8e8b1db6208020cbf15139c01814fa33dd10398e6bcec93c054a272",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "subject-claim-drill-in"
  },
  "kind": "observation",
  "cites": [
    "bafyreiflsvmy6c3jdbgld6xacr52ge5tun3wzmqe47sytpewklrgyovabi"
  ],
  "rev": "223mtfe7j7uzz",
  "seq": 8,
  "of": 12,
  "text_len": 251,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgq5VZjwtpGEyx+uAUe6MTs6N3bLIE5+WJvJZS4mw6oApmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx2c3ViamVjdC1jbGFpbS1kcmlsbC1pbmlhcnRpZmFjdHOBoWZDb21taXR4KGYxMzM3NjRjMTEyNTk4MGI2MGZiZWVmMzUxZDgzYzFmY2YwNzcxODNpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDgzNWY4N2FkZTQwYmRjOGRmMjJjM2JiZDI3OGQ1N2UwNDBmYWEwOGNjNmU0MTVkNmJkMDM4NGViOWE3ZTUzY2VrcmVjb3JkZWRfYXQbAAZZWory65U="
}
---

Coverage gap: the cid tie-break in claims_from_json (substrate.rs sort_by .then_with(|| a.cid.cmp(&b.cid))) is never exercised — the test fixture's recorded_at values are all distinct (100/200/250/300), so RQ-1's deterministic tie-break has no test.
***8<***
---
{
  "v": 3,
  "cid": "bafyreic2cuelm2y3xc5lpm4o5mcfmjjjwgspdud6wvqaodspz2vtjadpoq",
  "sig": "798e44bfaeaa0a356c3f83724b82fbe0918a58150ef22c95ec2e3795c6b3beae4d3d1ce5276625665aeb4ddcd3727a8cba961c8757120be8821929626d3ab638",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "subject-claim-drill-in"
  },
  "kind": "observation",
  "cites": [
    "bafyreiflsvmy6c3jdbgld6xacr52ge5tun3wzmqe47sytpewklrgyovabi"
  ],
  "rev": "223mtfe7jcsvt",
  "seq": 9,
  "of": 12,
  "text_len": 292,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgq5VZjwtpGEyx+uAUe6MTs6N3bLIE5+WJvJZS4mw6oApmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx2c3ViamVjdC1jbGFpbS1kcmlsbC1pbmlhcnRpZmFjdHOBoWZDb21taXR4KGYxMzM3NjRjMTEyNTk4MGI2MGZiZWVmMzUxZDgzYzFmY2YwNzcxODNpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDgzNWY4N2FkZTQwYmRjOGRmMjJjM2JiZDI3OGQ1N2UwNDBmYWEwOGNjNmU0MTVkNmJkMDM4NGViOWE3ZTUzY2VrcmVjb3JkZWRfYXQbAAZZWor0Yww="
}
---

Latent honest-ambiguity gap: claim_from_value defaults a missing/non-integer recorded_at to 0 (substrate.rs), which fmt_utc renders as a real-looking '1970-01-01 00:00' and sorts as oldest — fabricating certainty rather than surfacing 'unknown'. Latent today (kan always emits recorded_at).
***8<***
---
{
  "v": 3,
  "cid": "bafyreihqlsl5mpm6nw7vz2jzacpt76agmzw5kinkpvkqqtzkc2ddp5uo7q",
  "sig": "169e7b79509f8fcdd17b69435d291d70d23e7e399adaf4445fd3b67cf8dc80597e76045f9a0da971f33afc5af0c8bb6f7b495ea4654fda2865bc5a672da13d8a",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "subject-claim-drill-in"
  },
  "kind": "observation",
  "cites": [
    "bafyreiflsvmy6c3jdbgld6xacr52ge5tun3wzmqe47sytpewklrgyovabi"
  ],
  "rev": "223mtfe7jfmqi",
  "seq": 10,
  "of": 12,
  "text_len": 265,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgq5VZjwtpGEyx+uAUe6MTs6N3bLIE5+WJvJZS4mw6oApmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx2c3ViamVjdC1jbGFpbS1kcmlsbC1pbmlhcnRpZmFjdHOBoWZDb21taXR4KGYxMzM3NjRjMTEyNTk4MGI2MGZiZWVmMzUxZDgzYzFmY2YwNzcxODNpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDgzNWY4N2FkZTQwYmRjOGRmMjJjM2JiZDI3OGQ1N2UwNDBmYWEwOGNjNmU0MTVkNmJkMDM4NGViOWE3ZTUzY2VrcmVjb3JkZWRfYXQbAAZZWor1yms="
}
---

Test under-coverage of hand-rolled calendar math: fmt_utc has a single test point (2026-08-18). Independently verified correct against epoch 0, negative/pre-1970, 2024-02-29 leap, and 1900/2000/2100 century rules — but the committed suite exercises none of these.
***8<***
---
{
  "v": 3,
  "cid": "bafyreifmi7emr53id2vn2iygpjjc4baat7477v34ik2bzf3f7clrlucu5e",
  "sig": "cf110bf192ad3be324598830d02669b8d42e8fd01f9b10db4394dd1382de59e35f2c1c0e97f786aec095ce876bd3ac0504b1e3ec5ff4c925a4c1f5699440285b",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "subject-claim-drill-in"
  },
  "kind": "observation",
  "cites": [
    "bafyreiflsvmy6c3jdbgld6xacr52ge5tun3wzmqe47sytpewklrgyovabi"
  ],
  "rev": "223mtfe7jifjp",
  "seq": 11,
  "of": 12,
  "text_len": 273,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgq5VZjwtpGEyx+uAUe6MTs6N3bLIE5+WJvJZS4mw6oApmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx2c3ViamVjdC1jbGFpbS1kcmlsbC1pbmlhcnRpZmFjdHOBoWZDb21taXR4KGYxMzM3NjRjMTEyNTk4MGI2MGZiZWVmMzUxZDgzYzFmY2YwNzcxODNpd29ya3NwYWNloWlXb3Jrc3BhY2V4QDgzNWY4N2FkZTQwYmRjOGRmMjJjM2JiZDI3OGQ1N2UwNDBmYWEwOGNjNmU0MTVkNmJkMDM4NGViOWE3ZTUzY2VrcmVjb3JkZWRfYXQbAAZZWor3LZM="
}
---

short_author truncates the did:key to 8 chars from the front, where did:key encodes a shared multicodec prefix — different signers can share the shown prefix and read as the same author. Moot in this single-author repo; a real collision risk once multiple signers appear.
