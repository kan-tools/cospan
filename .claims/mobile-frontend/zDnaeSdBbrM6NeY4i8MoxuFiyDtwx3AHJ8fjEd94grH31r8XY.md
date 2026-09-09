---
{
  "v": 3,
  "cid": "bafyreigkmdjmk5ubn4s4e7ec32kpgywedcyygktmsiyy5xp6w3e2djv4nq",
  "sig": "bd9ad6248310b5b5f9fed33d3ed4747049d622ce5e6af2967d1daebfd5b9044c0925a80fc6748d55073799ce5c617fcbc32de148af74df26d637762507346df9",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "mobile-frontend"
  },
  "kind": "observation",
  "cites": [],
  "rev": "223mtqanifwqx",
  "seq": 0,
  "of": 4,
  "text_len": 1757,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2Fsb21vYmlsZS1mcm9udGVuZGlhcnRpZmFjdHOBoWZDb21taXR4KDk2YzljNTE5ZWUxYjkyOWMyZjg3ODFiYmQ0ODY2NmRjNmY0OTdmOGJpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZZsabl7OY="
}
---

Recorded vision (not started; deferred until the comment authoring milestone wraps): make cospan host all the data that drives the TUI as a thin local HTTP/WS API and expose it over a secure socket to a web client on the phone -- a mobile frontend to Chat/Comments/Ledger/Process and eventually control. Not a rewrite: substrate::Fold is a presentation-agnostic model and the TUI is one renderer over it; the phone client is a second renderer over the same fold. The operations core the S5 MCP server needs (read the fold, write a comment) is the same core this API needs -- one core, several transports (stdio-MCP for agents, HTTP/WS for the phone), so S5 is a down payment. Three new pieces: (1) a thin axum/tokio server that serializes the fold and pushes on each re-fold (poll-dont-subscribe governs reading the substrate, not pushing to cospan-owned clients); (2) a responsive PWA web client -- the real net-new cost, a separate non-Rust codebase; (3) secure transport, the crux -- an overlay network (Tailscale/WireGuard), no public exposure, app-level auth on top, and critically the kan signing seed stays on the laptop while the phone sends intents that cospan executes and signs (so every mobile action is a signed audited claim, kan-is-truth). Serves telos/observe-now-control-later (read-only first, WriteChannel seam reserved for control later) and kan-is-truth; in explicit tension with telos/disposable (a long-lived server + maintained web app + overlay network is materially more infrastructure than a throwaway sidecar) -- name and decide that deliberately. Phasing: localhost read-only API -> LAN read-only client -> secure remote + auth -> comment writes -> full P3 control plane (the sensitive tier). See .dropbox/08-mobile-frontend.md.
***8<***
---
{
  "v": 3,
  "cid": "bafyreigzdtvq4r5cywjchels3gds4zqkhushqjgizfvmafkudiilanfqeq",
  "sig": "2fd974b42bb39c732614cfff69eeadfbcddf92999610225587b3b2aea76e226a5caa08f0f41c56aec4430565618f8a8bc2adaf3abf03154afe603800b889044f",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "mobile-frontend"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223mtqanijasn",
  "seq": 1,
  "of": 4,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXhKTW9iaWxlIGZyb250ZW5kOiBjb3NwYW4gYXMgYSBob3N0ZWQgQVBJICsgc2VjdXJlIHNvY2tldCArIHBob25lIHdlYiBjbGllbnRsc3ViamVjdF9raW5kZElkZWFlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2Fsb21vYmlsZS1mcm9udGVuZGlhcnRpZmFjdHOBoWZDb21taXR4KDk2YzljNTE5ZWUxYjkyOWMyZjg3ODFiYmQ0ODY2NmRjNmY0OTdmOGJpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZZsabnmq0="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreialmytibf24yxd7psbn6c2fkpfmoyia75om4ytykajfnnfhksjali",
  "sig": "e2c2bb6ad7048780c4a2c92f08629bcc197af0f9d5f365ab4dd934199c653797591604fbe45d49577290a5c05e59ed6724eb99b4f597c1715c1b11b8daf5d613",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "mobile-frontend"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223mtqaniss4n",
  "seq": 2,
  "of": 4,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWxvbW9iaWxlLWZyb250ZW5kaWFydGlmYWN0c4GhZkNvbW1pdHgoOTZjOWM1MTllZTFiOTI5YzJmODc4MWJiZDQ4NjY2ZGM2ZjQ5N2Y4Yml3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABlmxpuxf7A=="
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreie4dudh4vlk22qqtyg37gl2skjpo3glb36nrojhekrrzpqef6u3du",
  "sig": "f6dffdcc48f19e41086435dedb440a7d7def9abf0262d2e819e5387caadb0d9463cccdb37ab87f2e64ca40d555b68db406c4093244ce07b071b41587d3c5887d",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "mobile-frontend"
  },
  "kind": "plan",
  "cites": [
    "bafyreiawteatremkiotyssugunupsk7lw5esffddxsfmfobpzo75nvs2zy"
  ],
  "rev": "223muakzupzim",
  "seq": 3,
  "of": 4,
  "text_len": 1109,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4HYKlglAAFxEiAWmQE4kYpDp4lKhqNo+Svrt0kilGO8isK4L8u/1tZazmZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZVNkQmJyTTZOZVk0aThNb3h1Rml5RHR3eDNBSEo4ZmpFZDk0Z3JIMzFyOFhZZWFnZW509mdzdWJqZWN0oWVMb2NhbG9tb2JpbGUtZnJvbnRlbmRpYXJ0aWZhY3RzgaFmQ29tbWl0eChkNzdkZmZkMzkyYjFiYTY0Nzk2NGVlODE0ODE0ODlmZGUzYjczYTBhaXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWjQ/qvpL"
}
---

Roadmap after alpha.20 (Phases 1-2 shipped: read API + interim embedded web view). Agreed slice order, each its own design->build->review->release: (B) HARDEN + AUTH the remote channel — app-level auth token on cospan serve + the two server follow-ups (resolve repo-internal symlinks in the path guard; cap concurrent /stream connections). The honest prerequisite now that Tailscale already makes serve remote. (A) UX PASS on the embedded GET / view — surface Comments (/comments + /thread already exist), claim drill-in, close the frontend follow-ups (reconnect backoff + visibility gating; redundant initial render); decide embedded-page vs separate PWA. (C) COMMENT WRITES over the API — the observe->control transition (Phase 4): phone sends intents, laptop signs (seed stays local), reusing the S5 write core; depends on B. (Chat) CHAT READS over the API — surface the Chat/transcripts tab (needs Serialize on the transcripts types, deferred by the mobile-api-server design). Sequencing follows telos/observe-now-control-later: authenticate+harden the observe channel, then cross the first write.
