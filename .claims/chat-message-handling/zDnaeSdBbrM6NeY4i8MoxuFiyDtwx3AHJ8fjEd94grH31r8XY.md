---
{
  "v": 3,
  "cid": "bafyreibmsivehwgbdvlzpfjelyen2nplcq4b56jwlpbsgqbfriu7zramny",
  "sig": "d6032b92133710e8a871dbea9ae75269aaba9327dc761a0f4c2d8d70b489d72b560a1ce9d16bb9422ea5341175de7f835b46d1815382074a912ea9f8380194fa",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "chat-message-handling"
  },
  "kind": "observation",
  "cites": [],
  "rev": "223muuzaihtkd",
  "seq": 0,
  "of": 12,
  "text_len": 196,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2FsdWNoYXQtbWVzc2FnZS1oYW5kbGluZ2lhcnRpZmFjdHOBoWZDb21taXR4KGZiMjhlNmYyNGExNTM3M2I3Y2Y4MTNiNjYzYmVlNDY0NGYwZGU0YThpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZa18zm4FY="
}
---

design doc .design/chat-message-handling.md checked against the live design-doc schema: validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s) [doc 11390:47cc16834c7ef229]
***8<***
---
{
  "v": 3,
  "cid": "bafyreicuej4yy27jencipghru2i7w4k5muolpbeqekjqavk22ye47j5bpi",
  "sig": "fbf099a3cdb1eb18b271dc8045e93cb76a95d497a30f7620398bd9c4036a846f5a6c10e940b478a79a9e846e8cf5d52c2e5059f8086b193b06b83ad44bc1fc37",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "chat-message-handling"
  },
  "kind": "plan",
  "cites": [
    "bafyreibmsivehwgbdvlzpfjelyen2nplcq4b56jwlpbsgqbfriu7zramny"
  ],
  "rev": "223muuzaj4xk3",
  "seq": 1,
  "of": 12,
  "text_len": 756,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4HYKlglAAFxEiAskipD2MEdV5eVJF4I3TXrFDge+TZbwyNAJYop/MQMbmZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZVNkQmJyTTZOZVk0aThNb3h1Rml5RHR3eDNBSEo4ZmpFZDk0Z3JIMzFyOFhZZWFnZW509mdzdWJqZWN0oWVMb2NhbHVjaGF0LW1lc3NhZ2UtaGFuZGxpbmdpYXJ0aWZhY3RzgaFmQ29tbWl0eChmYjI4ZTZmMjRhMTUzNzNiN2NmODEzYjY2M2JlZTQ2NDRmMGRlNGE4aXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWtfM8XVr"
}
---

chat-message-handling design (.design/chat-message-handling.md): The Chat transcript renders message bodies as flat plain text (`turnEl` → `.tbody`/`textContent` at `src/web/index.html:1270`). Render message turns as markdown — headings, paragraphs, bold/italic, inline code, bullet and numbered lists, fenced code blocks, and rules — by **pre-rendering on the server** into a structured block stream that `GET /chat` ships in each event, mirroring how `highlight::styled_web` already ships colored code runs for the file viewer. Fenced code blocks are syntax-highlighted through that same `styled_web`. No client-side markdown library and no CDN (`telos/disposable`). [validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s)]
***8<***
---
{
  "v": 3,
  "cid": "bafyreidpt5em6ibc7cxsrf3lfaa3hvhhplpsbfmsaugavh6zonsumdd7nu",
  "sig": "6c64ae7d6f06769a60bebc356ee33cb6ea66fc9af0e767d85a38d1fbe64cdb950200cbf0216343568939270ae9743f933ffcc7a9f838c0b280370e65d9241fe9",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "chat-message-handling"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223muuzajdzha",
  "seq": 2,
  "of": 12,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXhVRmVhdHVyZTogQ2hhdCBtZXNzYWdlIGhhbmRsaW5nIOKAlCBzZXJ2ZXItcmVuZGVyZWQgbWFya2Rvd24gKyBoaWdobGlnaHRlZCBjb2RlIGJsb2Nrc2xzdWJqZWN0X2tpbmRkSWRlYWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx1Y2hhdC1tZXNzYWdlLWhhbmRsaW5naWFydGlmYWN0c4GhZkNvbW1pdHgoZmIyOGU2ZjI0YTE1MzczYjdjZjgxM2I2NjNiZWU0NjQ0ZjBkZTRhOGl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABlrXzPT9LQ=="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreiaqvwyo53nybxhjmmvyyn5q2j2nxlkafd3or6xw45l62worrxwjru",
  "sig": "d7982cd02b9d3d63d940003d5b32ea4ae1c4de8dffc70cada4f0289796c9bff64c64cc820a40bef5c319c8f8124d2cd89eac996eccfa308b12fd9fbd220603c2",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "chat-message-handling"
  },
  "kind": "decision",
  "cites": [
    "bafyreicuej4yy27jencipghru2i7w4k5muolpbeqekjqavk22ye47j5bpi"
  ],
  "rev": "223muuzak2v7i",
  "seq": 3,
  "of": 12,
  "text_len": 399,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgVCJ5jGvpI0SHmPGmkftxXWUct4SQIpMAVVrWCc+noXpmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx1Y2hhdC1tZXNzYWdlLWhhbmRsaW5naWFydGlmYWN0c4GhZkNvbW1pdHgoZmIyOGU2ZjI0YTE1MzczYjdjZjgxM2I2NjNiZWU0NjQ0ZjBkZTRhOGl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABlrXzQBsNg=="
}
---

RQ-1: Parse on the server, not in inline JS. `highlight::styled_web` + the file viewer already establish server-pre-render/client-dumb-render as the house pattern, and reusing `pulldown-cmark` (already a dep) puts parsing correctness in Rust with tests rather than a hand-rolled JS parser. Cost accepted: a Rust change and a wider `/chat` JSON, departing from the round's otherwise page-only slices.
***8<***
---
{
  "v": 3,
  "cid": "bafyreig7zfll47v7pvprr26qf7ysvuaj7ctntjgajo3xwm226spwatjpmy",
  "sig": "455980ab8b37ab3532ca1e5e5c4dca5bef8116a2056b982c773eb5c8af97d9a776b5f5ce9e48daa89664736d52b4b4ee688199a81287161876b564f74b57d43c",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "chat-message-handling"
  },
  "kind": "decision",
  "cites": [
    "bafyreicuej4yy27jencipghru2i7w4k5muolpbeqekjqavk22ye47j5bpi"
  ],
  "rev": "223muuzakqvxa",
  "seq": 4,
  "of": 12,
  "text_len": 207,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgVCJ5jGvpI0SHmPGmkftxXWUct4SQIpMAVVrWCc+noXpmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx1Y2hhdC1tZXNzYWdlLWhhbmRsaW5naWFydGlmYWN0c4GhZkNvbW1pdHgoZmIyOGU2ZjI0YTE1MzczYjdjZjgxM2I2NjNiZWU0NjQ0ZjBkZTRhOGl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABlrXzQtvEw=="
}
---

RQ-2: Fenced code blocks are highlighted via `styled_web` by their language tag, so chat code reads like the file viewer, rather than a plain monochrome `<pre>`. Essentially free given the server-side parse.
***8<***
---
{
  "v": 3,
  "cid": "bafyreigfldqh6tu2vmulb5hfjnucwfegj65meti64bi4ox6kw5okyfdkna",
  "sig": "3a748721516c3e5057b503e4b89a5798639005e29170adc368296b3652c388916a7e2e0934df3c2d176831be49b709697608d7635c59e89f725775e6ea8ecb5c",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "chat-message-handling"
  },
  "kind": "decision",
  "cites": [
    "bafyreicuej4yy27jencipghru2i7w4k5muolpbeqekjqavk22ye47j5bpi"
  ],
  "rev": "223muuzalg2lg",
  "seq": 5,
  "of": 12,
  "text_len": 227,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgVCJ5jGvpI0SHmPGmkftxXWUct4SQIpMAVVrWCc+noXpmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx1Y2hhdC1tZXNzYWdlLWhhbmRsaW5naWFydGlmYWN0c4GhZkNvbW1pdHgoZmIyOGU2ZjI0YTE1MzczYjdjZjgxM2I2NjNiZWU0NjQ0ZjBkZTRhOGl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABlrXzRYBmQ=="
}
---

RQ-3: Message turns only. Thinking/toolcall/toolresult keep their collapsed `<pre>` rendering — tool results are frequently raw logs/JSON that markdown parsing would mangle, and the handoff scoped the slice to message bodies.
***8<***
---
{
  "v": 3,
  "cid": "bafyreifehdkppao25upgpz4hlnv4brk7trxcaes34itcendgpkxwbyzpge",
  "sig": "07c8ff6fbf31af6a83aee5ff0f008513098851a17949954f39b6d4c61a24d99d755b2f6ccb428a969464d99f56bbeaf7f11b2f0ea8b04f6e4174631c0d3da486",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "chat-message-handling"
  },
  "kind": "decision",
  "cites": [
    "bafyreicuej4yy27jencipghru2i7w4k5muolpbeqekjqavk22ye47j5bpi"
  ],
  "rev": "223muuzam35u4",
  "seq": 6,
  "of": 12,
  "text_len": 429,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgVCJ5jGvpI0SHmPGmkftxXWUct4SQIpMAVVrWCc+noXpmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx1Y2hhdC1tZXNzYWdlLWhhbmRsaW5naWFydGlmYWN0c4GhZkNvbW1pdHgoZmIyOGU2ZjI0YTE1MzczYjdjZjgxM2I2NjNiZWU0NjQ0ZjBkZTRhOGl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABlrXzSCOvQ=="
}
---

RQ-4: Links are rendered as sanitized anchors, not dropped to plain text. A `Tag::Link` becomes an `<a target="_blank" rel="noopener noreferrer">` — but only when its scheme is `http`/`https`, checked on the server (an allowlist, so the client never sees a `javascript:`/`data:` href); any other scheme falls back to plain span text. This keeps chat links useful without opening an href surface on untrusted transcript content.
***8<***
---
{
  "v": 3,
  "cid": "bafyreidzo6ocmmq46m2xz4zpetgky7d7io7fbzti63yzbyx4muprqaumnq",
  "sig": "fc9f7e31e741ce973fb74252b077cdeffdd14d494e577e0d0536f10e35da626a21adb12e2cf2e14f11750294b3182cad342e2c5f09e495f847a59662910519b5",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "chat-message-handling"
  },
  "kind": "result",
  "cites": [
    "bafyreicuej4yy27jencipghru2i7w4k5muolpbeqekjqavk22ye47j5bpi"
  ],
  "rev": "223muy36txs33",
  "seq": 7,
  "of": 12,
  "text_len": 579,
  "content": "p2Rib2R5oWZSZXN1bHShZHRleHRgZWNpdGVzgdgqWCUAAXESIFQieYxr6SNEh5jxppH7cV1lHLeEkCKTAFVa1gnPp6F6ZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2FsdWNoYXQtbWVzc2FnZS1oYW5kbGluZ2lhcnRpZmFjdHOBoWZDb21taXR4KGZiMjhlNmYyNGExNTM3M2I3Y2Y4MTNiNjYzYmVlNDY0NGYwZGU0YThpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZa8Eme27M="
}
---

generative-build complete: server-side markdown block stream (markdown::render_web) shipped in src/markdown.rs; event_json attaches blocks to Message turns only (src/chat.rs); the Chat page renders blocks into DOM nodes with fenced code highlighted via styled_web and sanitized http(s) anchors, no client parser/library (src/web/index.html). 5 new tests; cargo test 220 pass, clippy -D warnings clean, fmt clean. Operator eyeballed the render (AC-5) — looks good; server-side wire shape verified on a real 1752-event session (368 message turns, json/rs/sh/toml/md highlighted).
***8<***
---
{
  "v": 3,
  "cid": "bafyreiezjxa633xbcr35axrfnfao5wygbldpumfhbbtsn2xbhvczmoh52i",
  "sig": "58351606ae402c09d66e10caa19b369a85f3de55901086e5766dd4b4dffa17a257c219207c76a17f28475ccb25a057b88ff9c40e97d5b469b22c3b408419a2be",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "chat-message-handling"
  },
  "kind": "decision",
  "cites": [
    "bafyreicuej4yy27jencipghru2i7w4k5muolpbeqekjqavk22ye47j5bpi"
  ],
  "rev": "223muy3m5krii",
  "seq": 8,
  "of": 12,
  "text_len": 272,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgVCJ5jGvpI0SHmPGmkftxXWUct4SQIpMAVVrWCc+noXpmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx1Y2hhdC1tZXNzYWdlLWhhbmRsaW5naWFydGlmYWN0c4GhZkNvbW1pdHgoZmIyOGU2ZjI0YTE1MzczYjdjZjgxM2I2NjNiZWU0NjQ0ZjBkZTRhOGl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABlrwZDhbQA=="
}
---

adversarial review of chat-message-handling: APPROVE-WITH-FOLLOW-UPS — All 7 REQ / 6 AC met with genuine tests; build/test(220)/clippy/fmt green; href allowlist has no bypass, no innerHTML, no path-leak, no external dep. Two cosmetic follow-ups within the scoped subset.
***8<***
---
{
  "v": 3,
  "cid": "bafyreia5mnphqnxbfmka4vpvllwl5ojsoj4tk5xilfyii7wpa6vzcx3ppu",
  "sig": "8d6f4cfcc27153701cd415d79f20d6c11c9d8dfcb3333d02adfed0514dde53743d5519b8ae8f937bbf0b368c60575a5d20e02937ca9f6792a1167482d3e8ec26",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "chat-message-handling"
  },
  "kind": "observation",
  "cites": [
    "bafyreiezjxa633xbcr35axrfnfao5wygbldpumfhbbtsn2xbhvczmoh52i"
  ],
  "rev": "223muy3mftixc",
  "seq": 9,
  "of": 12,
  "text_len": 204,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgmU3B7e7hFHfQXiVpQO7bBgrG+jCnCGcm6uE9RZY4/dJmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx1Y2hhdC1tZXNzYWdlLWhhbmRsaW5naWFydGlmYWN0c4GhZkNvbW1pdHgoZmIyOGU2ZjI0YTE1MzczYjdjZjgxM2I2NjNiZWU0NjQ0ZjBkZTRhOGl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABlrwZLy7IA=="
}
---

Review follow-up F1: mdList (src/web/index.html) ignores the server-supplied item.num, so an ordered list starting at a non-1 value renders from 1. Either honor it.num via <li value> or stop shipping num.
***8<***
---
{
  "v": 3,
  "cid": "bafyreigi367is6osttithxfavadwg6kht66bk4e5taggfw3fn3a3exik4i",
  "sig": "28883c29295fe57c4f21bd52d7c61cf2cbe069ffede2eedfff9749eb7df54a7f4f2fe8ffb81195e2027ae198e5bb6f84631faa98aa7033971109895cf93eb8a5",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "chat-message-handling"
  },
  "kind": "observation",
  "cites": [
    "bafyreiezjxa633xbcr35axrfnfao5wygbldpumfhbbtsn2xbhvczmoh52i"
  ],
  "rev": "223muy3mgjzzp",
  "seq": 10,
  "of": 12,
  "text_len": 250,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgmU3B7e7hFHfQXiVpQO7bBgrG+jCnCGcm6uE9RZY4/dJmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx1Y2hhdC1tZXNzYWdlLWhhbmRsaW5naWFydGlmYWN0c4GhZkNvbW1pdHgoZmIyOGU2ZjI0YTE1MzczYjdjZjgxM2I2NjNiZWU0NjQ0ZjBkZTRhOGl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABlrwZMf/gQ=="
}
---

Review follow-up F2: a code block or heading nested inside a list item escapes the list (flush_para at Start(CodeBlock) in src/markdown.rs), landing at top level and leaving an empty <li> behind. Minor; within the design subset that degrades to text.
***8<***
---
{
  "v": 3,
  "cid": "bafyreibhd2deioawlqfvmmen3g5zazdgcluganm6k3omt2ub5ta2j36kvu",
  "sig": "ebb5fa5542359d5ec17218867d5da495a2f9160dbcc97dd853a9a8206293a3c222a3af504c59a8ddc2ed4afe66165ffa563cf32bf46fc43ee69f4edba680a52d",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "chat-message-handling"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mv23plhkyi",
  "seq": 11,
  "of": 12,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx1Y2hhdC1tZXNzYWdlLWhhbmRsaW5naWFydGlmYWN0c4GhZkNvbW1pdHgoOTVlOTQ3YTRiMjE3ZDI4ZDkzN2ZmMTZkMjQ5MWI4N2Q5ZGY4MTZkOWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABlsAaxbDUA=="
}
---
