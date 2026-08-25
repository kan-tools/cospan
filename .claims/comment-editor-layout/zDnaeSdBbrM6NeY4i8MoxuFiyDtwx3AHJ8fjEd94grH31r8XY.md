---
{
  "v": 3,
  "cid": "bafyreih4mzqwx2meoj2ovx4s2r57xck2eptisdg67rffyu4mycjr7l7nyy",
  "sig": "25ba83d69b39fbfddc052325d559e45d681bdcba3baa243067aa787f5190c58c6702151c41aaffa3cf38871b4f6f75a9743e8239fe93342fee5df635bbb659a6",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-editor-layout"
  },
  "kind": "observation",
  "cites": [],
  "rev": "223mtuenvwfpi",
  "seq": 0,
  "of": 12,
  "text_len": 196,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2FsdWNvbW1lbnQtZWRpdG9yLWxheW91dGlhcnRpZmFjdHOBoWZDb21taXR4KDU4ODRkMTY5ODIxZjgzYTdmZjA0NWRmNWI5OTJiZjcxNGE4N2Q1NTBpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZZ0qe+K/Q="
}
---

design doc .design/comment-editor-layout.md checked against the live design-doc schema: validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s) [doc 10178:7c17684688d0d64a]
***8<***
---
{
  "v": 3,
  "cid": "bafyreidhqosy7t3bmnp34xarehi2v43e6gpmeyffr3cgs7bmciqpm4jxhq",
  "sig": "5152c4a717a15988f733f4982360254bd58478892b8a9b987e404a80e4be2429515ccf68ee97cd95e16eef7688695654b90cd57ab850e9555376cf569c414062",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-editor-layout"
  },
  "kind": "plan",
  "cites": [
    "bafyreih4mzqwx2meoj2ovx4s2r57xck2eptisdg67rffyu4mycjr7l7nyy"
  ],
  "rev": "223mtuenwbwha",
  "seq": 1,
  "of": 12,
  "text_len": 763,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4HYKlglAAFxEiD8ZmFr6YRydOrfktR7+4laI+aJDN78SlxTjMCTH6/txmZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZVNkQmJyTTZOZVk0aThNb3h1Rml5RHR3eDNBSEo4ZmpFZDk0Z3JIMzFyOFhZZWFnZW509mdzdWJqZWN0oWVMb2NhbHVjb21tZW50LWVkaXRvci1sYXlvdXRpYXJ0aWZhY3RzgaFmQ29tbWl0eCg1ODg0ZDE2OTgyMWY4M2E3ZmYwNDVkZjViOTkyYmY3MTRhODdkNTUwaXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWdKnw/Ek"
}
---

comment-editor-layout design (.design/comment-editor-layout.md): Reshape the Comments tab's layout so the code and comments own the full frame: make the existing file-tree rail a **toggleable tray** (auto-collapsed while reading a file), **remove the fixed bottom strip**, rehome the two jobs the strip did — the selected comment's full thread moves into an **Enter-triggered popup**, and the `Unresolvable` comments become a **pinned group at the top of the note column** so they stay visible (`telos/honest-ambiguity`). This is Slice A of the `.dropbox/05-views-ux.md` section (2) vision; live git-diff rendering in the editor pane is deliberately split into a follow-up slice. [validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s)]
***8<***
---
{
  "v": 3,
  "cid": "bafyreidfprk6feogbo32a2ksmas5rpdsiav3orn453sxijscm6fbdta5pq",
  "sig": "cbab5c976b345d325da100c9db71fdb5a7082b55f9108bee002831d957de2b9d7c2ce4f236e56b1653fdbd1b6ffaee946584113cb9b11e4d87760bf4d3e5442e",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-editor-layout"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223mtuenwfqhv",
  "seq": 2,
  "of": 12,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXgxRmVhdHVyZTogQ29tbWVudHMtdGFiIGVkaXRvci12aWV3IGxheW91dCByZWRlc2lnbmxzdWJqZWN0X2tpbmRkSWRlYWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx1Y29tbWVudC1lZGl0b3ItbGF5b3V0aWFydGlmYWN0c4GhZkNvbW1pdHgoNTg4NGQxNjk4MjFmODNhN2ZmMDQ1ZGY1Yjk5MmJmNzE0YTg3ZDU1MGl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABlnSp8XZTA=="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreiagyyni7xltongqqmjp5tcdnfxnkrh5yf43tnpbegetkn4bhy6doi",
  "sig": "077145b69d11a4221ae00d43965b1c8c7be1e9125fd4ab077327fb4687a1b24c02c66735cc506c4b28ad442da50b9aea1e02659f52d13e8bd8b4cedf97a07bea",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-editor-layout"
  },
  "kind": "decision",
  "cites": [
    "bafyreidhqosy7t3bmnp34xarehi2v43e6gpmeyffr3cgs7bmciqpm4jxhq"
  ],
  "rev": "223mtuenwsb2e",
  "seq": 3,
  "of": 12,
  "text_len": 326,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgZ4Olj89hY1++XBEh0arzZPGewmCljsRpfCwSIPZxNzxmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx1Y29tbWVudC1lZGl0b3ItbGF5b3V0aWFydGlmYWN0c4GhZkNvbW1pdHgoNTg4NGQxNjk4MjFmODNhN2ZmMDQ1ZGY1Yjk5MmJmNzE0YTg3ZDU1MGl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABlnSp8wbiw=="
}
---

RQ-1: Scope is split. This design is Slice A — the toggleable tray, strip removal, overflow popup, and pinned unresolvable group. Live git working-tree visual diffs in the editor pane are a separate follow-up slice (Slice B) with their own design/build/review, because they are greenfield and independent of the layout work.
***8<***
---
{
  "v": 3,
  "cid": "bafyreigbjc7mbqqpz524ao344fqfoeviu33thb6smj42o2ukni3twvqu5y",
  "sig": "823dc181258061efa5cd47a2e21772a7ba3a5b83661a9a30f5e4dc66daa79c0e2ff80e7bbb4194c0caa94aac1d324b149c49e51a7c215cab638bf60af8610f2f",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-editor-layout"
  },
  "kind": "decision",
  "cites": [
    "bafyreidhqosy7t3bmnp34xarehi2v43e6gpmeyffr3cgs7bmciqpm4jxhq"
  ],
  "rev": "223mtuenx5w7s",
  "seq": 4,
  "of": 12,
  "text_len": 229,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgZ4Olj89hY1++XBEh0arzZPGewmCljsRpfCwSIPZxNzxmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx1Y29tbWVudC1lZGl0b3ItbGF5b3V0aWFydGlmYWN0c4GhZkNvbW1pdHgoNTg4NGQxNjk4MjFmODNhN2ZmMDQ1ZGY1Yjk5MmJmNzE0YTg3ZDU1MGl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABlnSp9HwQw=="
}
---

RQ-2: The `Unresolvable` comments are rehomed to a pinned "unresolvable (N)" group at the top of the note column, always visible rather than behind a keypress, honoring `telos/honest-ambiguity`'s resolve-by-hand contract (REQ-4).
***8<***
---
{
  "v": 3,
  "cid": "bafyreifxgupat4c5mx6ccvmec3dre5oaxtryjylc3ketcefqqgkg6llpea",
  "sig": "91096aedd264ba50e01e3bb0f2659e7720ca56c34257b9c104f727c5e026bf596f9a9e8eb2d1a1b0edcd1c79f50cc853913fe6c588228c84e0fc23171c65acbb",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-editor-layout"
  },
  "kind": "decision",
  "cites": [
    "bafyreidhqosy7t3bmnp34xarehi2v43e6gpmeyffr3cgs7bmciqpm4jxhq"
  ],
  "rev": "223mtuenxjfh7",
  "seq": 5,
  "of": 12,
  "text_len": 190,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgZ4Olj89hY1++XBEh0arzZPGewmCljsRpfCwSIPZxNzxmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx1Y29tbWVudC1lZGl0b3ItbGF5b3V0aWFydGlmYWN0c4GhZkNvbW1pdHgoNTg4NGQxNjk4MjFmODNhN2ZmMDQ1ZGY1Yjk5MmJmNzE0YTg3ZDU1MGl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABlnSp9etMQ=="
}
---

RQ-3: Full-thread reading is done through the Enter popup (REQ-5); the column notes stay compact — the `BODY_CAP = 3` cap and reflow are unchanged (REQ-6) — rather than expanding inline.
***8<***
---
{
  "v": 3,
  "cid": "bafyreifybg23zvyxbzimyqccnt3dg4ka4ccvg2ksjpe6o663jmhl7zk2py",
  "sig": "35d098ad6327ef476dcb637f093e5beb737ef289aaa5717bd56b2864194669ab0f25504a4a281bde5d08ff7ed2dd143803e85544a947874b43cb0d2c34691f20",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-editor-layout"
  },
  "kind": "decision",
  "cites": [
    "bafyreidhqosy7t3bmnp34xarehi2v43e6gpmeyffr3cgs7bmciqpm4jxhq"
  ],
  "rev": "223mtuenxurvs",
  "seq": 6,
  "of": 12,
  "text_len": 192,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgZ4Olj89hY1++XBEh0arzZPGewmCljsRpfCwSIPZxNzxmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx1Y29tbWVudC1lZGl0b3ItbGF5b3V0aWFydGlmYWN0c4GhZkNvbW1pdHgoNTg4NGQxNjk4MjFmODNhN2ZmMDQ1ZGY1Yjk5MmJmNzE0YTg3ZDU1MGl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABlnSp91fBQ=="
}
---

RQ-4: The file tray auto-behaves: shown when no file is open (needed to pick one), auto-collapsed once a file opens so code + notes get full width, and toggled anytime with `t` (REQ-1, REQ-2).
***8<***
---
{
  "v": 3,
  "cid": "bafyreiefcw573h32mynjcicwkjubrfi7zvhnvptqnojan3xuv2yeb6qczm",
  "sig": "0cbe1ea6a6897c3176c45fe7a76cc930d5a1dbc3158647da03e97459a801193673e122d5ddfd609a22f5aefca4d5286ef630c1b08366cec78602503c1b5d2b4d",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-editor-layout"
  },
  "kind": "decision",
  "cites": [
    "bafyreidhqosy7t3bmnp34xarehi2v43e6gpmeyffr3cgs7bmciqpm4jxhq"
  ],
  "rev": "223mtuenya5as",
  "seq": 7,
  "of": 12,
  "text_len": 123,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgZ4Olj89hY1++XBEh0arzZPGewmCljsRpfCwSIPZxNzxmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx1Y29tbWVudC1lZGl0b3ItbGF5b3V0aWFydGlmYWN0c4GhZkNvbW1pdHgoNTg4NGQxNjk4MjFmODNhN2ZmMDQ1ZGY1Yjk5MmJmNzE0YTg3ZDU1MGl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABlnSp+MMZA=="
}
---

RQ-5: The popup is actionable, not read-only: `r`/`e`/`d`/`x` act on the comment being read while it is open (REQ-5, AC-7).
***8<***
---
{
  "v": 3,
  "cid": "bafyreidzobvalpwlev3pcpfblmmcmceroynug2gwf4zpgavwysu6sh436a",
  "sig": "d2b4f937e60d8ddb08cf0da4f822b39208c425584fb2695513aee422e785669a100e3e1fa8aa75878a11ce57fb49993ee22377393623ad9785901ad781209e84",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-editor-layout"
  },
  "kind": "result",
  "cites": [],
  "rev": "223mtuhruzlqk",
  "seq": 8,
  "of": 12,
  "text_len": 1193,
  "content": "p2Rib2R5oWZSZXN1bHShZHRleHRgZWNpdGVzgGZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZVNkQmJyTTZOZVk0aThNb3h1Rml5RHR3eDNBSEo4ZmpFZDk0Z3JIMzFyOFhZZWFnZW509mdzdWJqZWN0oWVMb2NhbHVjb21tZW50LWVkaXRvci1sYXlvdXRpYXJ0aWZhY3RzgaFmQ29tbWl0eChiM2FkNDZlNjQ2MjYxMzFmMGNmNzE2MGNhYmM2YjlkODIxZmQyZmI3aXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWdNvr8Zm"
}
---

Adversarial review of comment-editor-layout (Slice A): BLOCK then fixed. An independent hostile Opus reviewer (fresh subagent) verified build/clippy/test clean and returned BLOCK on two confirmed findings: (1) opening a file did not auto-collapse the tray — once toggled open it stayed open across files, missing REQ-1, and the AC-1 test was tautological; (2) narrow mode (width < 100) silently dropped the always-visible unresolvable list, breaking telos/honest-ambiguity. Both fixed in b3ad46e: open_path now resets tray_open, AC-1 rewritten to exercise it, and the narrow arm pins the unresolvable band below the code pane. Also fixed the reviewer notes: the unresolvable band keeps the selected comment visible past its 4-row cap (REQ-4), and centered_rect does percentage math in u32 to avoid a u16 overflow on very wide terminals. Two operator eyeball requests landed in the same commit: full-row background band on comment-covered lines (fill_line_bg), and sticky comment-cursor scrolling that no longer snaps to the top (sticky_top + a note_scroll viewport cache). 177 tests pass, clippy -D warnings clean, fmt clean. Pending: an operator re-eyeball of the new visuals before the PR.
***8<***
---
{
  "v": 3,
  "cid": "bafyreibc5uoliobcd4ixhxltx2pqfkyzp6243bunkzuakln3z4vdrj7rhy",
  "sig": "b48b82d357cb008d985645eea552c84b261fd49f890fb837ffd77f9fd4f06d6475bd78cb325edc77626e9f55dca185db8f8e351a41cf4289777250cc87584029",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-editor-layout"
  },
  "kind": "result",
  "cites": [],
  "rev": "223mtult4tqyj",
  "seq": 9,
  "of": 12,
  "text_len": 1164,
  "content": "p2Rib2R5oWZSZXN1bHShZHRleHRgZWNpdGVzgGZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZVNkQmJyTTZOZVk0aThNb3h1Rml5RHR3eDNBSEo4ZmpFZDk0Z3JIMzFyOFhZZWFnZW509mdzdWJqZWN0oWVMb2NhbHVjb21tZW50LWVkaXRvci1sYXlvdXRpYXJ0aWZhY3RzgaFmQ29tbWl0eCg4NzBiNWU0MGExMzNmZGQ2MzNiM2Y3YjkzZDNmYjE0ZGM1Zjg5MzI3aXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWdRyLNmo"
}
---

Re-review of the post-BLOCK delta (commits b3ad46e + 870b5e4): SHIP. A fresh independent hostile Opus reviewer verified the two prior blockers are genuinely fixed with non-tautological tests -- open_path resets tray_open so a toggled-open tray collapses when another file opens (REQ-1), and the narrow arm pins the unresolvable band below the code pane so honest-ambiguity holds at every width. The new helpers were attacked fresh and found correct and panic-free: fill_line_bg pads only banded lines to the pane inner width with no underflow; sticky_top is correct at all viewport boundaries with no usize underflow; the note_scroll Cell shared between the wide reading view and the pick-line picker is safe (self-corrects each frame, all indexing clamped); unresolvable_group windowing keeps the selected comment visible past the cap with an honest +N count; centered_rect does u32 math. Gates green: 178 tests, clippy -D warnings clean, fmt clean. One cosmetic non-blocking caveat: fill_line_bg measures width by char count, so tabs/CJK could over- or under-pad by a cell (harmless, clipped by ratatui). Pending only the operator live-TTY eyeball before the PR.
***8<***
---
{
  "v": 3,
  "cid": "bafyreiarfzb2p37ybtty4c343yslqid7lq3ibeb3nduyqdnesihy3ae7im",
  "sig": "30a0d123187ebd4a20a68eb84b40add0f22f017d447eb2b69cafd907035a2c0e0cbc7c43cd016f742e4ef8edf37fa4eee13559dfd58c9268b25d7847890c5eb9",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-editor-layout"
  },
  "kind": "result",
  "cites": [],
  "rev": "223mtupkptvzp",
  "seq": 10,
  "of": 12,
  "text_len": 888,
  "content": "p2Rib2R5oWZSZXN1bHShZHRleHRgZWNpdGVzgGZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZVNkQmJyTTZOZVk0aThNb3h1Rml5RHR3eDNBSEo4ZmpFZDk0Z3JIMzFyOFhZZWFnZW509mdzdWJqZWN0oWVMb2NhbHVjb21tZW50LWVkaXRvci1sYXlvdXRpYXJ0aWZhY3RzgaFmQ29tbWl0eChkYmY4NTFiMWFkYzI1OTY1MDJiZmE5ZjMwNDFkM2RmNGUwYzBlZWJiaXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWdVhXO42"
}
---

Final review of commit dbf851b (PgUp/PgDn in the line picker + dynamic width-capped compose popup): SHIP. A fresh independent hostile Opus reviewer verified both deltas: the PickLine PageUp/PageDown paging is saturating and correctly clamped at both ends, uses the same page size as the read view, and its computation of page from self.body_h by disjoint-field access is borrow-sound; compose_popup_y places the popup adjacent to the target line (below when it fits, else above, else clamped) with saturating math that can never land outside body or cover the target line, and the 80-column width cap (COMPOSE_MAX_W) composes correctly with the existing clamp. Both new tests genuinely assert the behavior. Gates green: 180 tests, clippy -D warnings clean, fmt clean. This completes the review coverage of the whole comment-editor-layout branch (PR #23), CI green, operator eyeball clean.
***8<***
---
{
  "v": 3,
  "cid": "bafyreieynmf22xp6nwq4eubzzaiur42y7w7foigcnxvndk6gbmyzxiat74",
  "sig": "04699dace3a9be1cbeb7e697b1862ed6d149ee882b77ee4b5d4d8def71abcc205867fb6cfd5367d61f6c3bd67b318fdbcdf89e44b6cbad38ea73d19107c67fad",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "comment-editor-layout"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mtwoonvk6c",
  "seq": 11,
  "of": 12,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx1Y29tbWVudC1lZGl0b3ItbGF5b3V0aWFydGlmYWN0c4GhZkNvbW1pdHgoMGI0Mzk1NjZlYjY5OGU2OWU1ZDYzZTU2N2FmZjY4OTJkMDRiM2RkNWl3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABlnlKT3AFQ=="
}
---
