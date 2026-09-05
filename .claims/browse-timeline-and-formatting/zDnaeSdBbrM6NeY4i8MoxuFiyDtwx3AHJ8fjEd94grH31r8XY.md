---
{
  "v": 3,
  "cid": "bafyreieypz6hscatqxgz5stsy7l3puoqcd734x56bsp445h6odv45lxwma",
  "sig": "5b5cd1d5a554e9e32de2b4578943f63d6d562c551132477148fcf4b4d42317ad68a3a01f8c7dffe6d11d1782cb7603e741218c9e1d268f63f20aa047db4ed8df",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "browse-timeline-and-formatting"
  },
  "kind": "observation",
  "cites": [],
  "rev": "223mupbn4xe72",
  "seq": 0,
  "of": 14,
  "text_len": 205,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2FseB5icm93c2UtdGltZWxpbmUtYW5kLWZvcm1hdHRpbmdpYXJ0aWZhY3RzgaFmQ29tbWl0eChiODI4YzhhODE1Yzc4MDkwMWQ2MjliMWUzNzI0MjM1Y2U2MWQzYzE2aXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWqnmLqgn"
}
---

design doc .design/browse-timeline-and-formatting.md checked against the live design-doc schema: validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s) [doc 10143:993c0021b0ce7edf]
***8<***
---
{
  "v": 3,
  "cid": "bafyreihisrt4pmqvx7l5sxfdgxb4tybkvh5w7tx7h75bfyc57mqary3foy",
  "sig": "8982310b2f1cc968a0fde69f41debdef4541771f597ad7e27e6144be48dcb2545133d48297deb425166bfe596db7cb623252dedbfc9d534657036a02053d5a4b",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "browse-timeline-and-formatting"
  },
  "kind": "plan",
  "cites": [
    "bafyreieypz6hscatqxgz5stsy7l3puoqcd734x56bsp445h6odv45lxwma"
  ],
  "rev": "223mupbn5jfmy",
  "seq": 1,
  "of": 14,
  "text_len": 654,
  "content": "p2Rib2R5oWRQbGFuoWR0ZXh0YGVjaXRlc4HYKlglAAFxEiCYfnx5CBOFzZ7KcsfXt9HQEP++X74Mn850/nDrzq72YGZhdXRob3KiY2RpZHg5ZGlkOmtleTp6RG5hZVNkQmJyTTZOZVk0aThNb3h1Rml5RHR3eDNBSEo4ZmpFZDk0Z3JIMzFyOFhZZWFnZW509mdzdWJqZWN0oWVMb2NhbHgeYnJvd3NlLXRpbWVsaW5lLWFuZC1mb3JtYXR0aW5naWFydGlmYWN0c4GhZkNvbW1pdHgoYjgyOGM4YTgxNWM3ODA5MDFkNjI5YjFlMzcyNDIzNWNlNjFkM2MxNml3b3Jrc3BhY2WhaVdvcmtzcGFjZXhAYTIxMzIzZmM2OTcyNjdjMmMzNjg3ODU3NTg2OGQ0OGUzOTMyZTQzOGQwNzcwOWIwYTZlYmNmMjRkMmFkZjhlZmtyZWNvcmRlZF9hdBsABlqp5jet4A=="
}
---

browse-timeline-and-formatting design (.design/browse-timeline-and-formatting.md): Rework the serve page's Browse tab into a master-detail view with two list modes — a subject list (each row carrying a current-state summary) and a chronological timeline of all claims — plus striking per-operation formatting so each claim kind reads at a glance. Tapping a subject or a timeline claim opens its detail in the detail pane. Also completes the Browse master-detail conversion deferred from the responsive-layout slice. All data is already in the fold; no endpoint change. [validation: 9 check(s), 0 failed, 0 warning(s), 0 unchecked, 0 open question(s)]
***8<***
---
{
  "v": 3,
  "cid": "bafyreicho5qthcg47hlsmbrccire23rrgh3n6wwonrorn5kogmbovibpmq",
  "sig": "59fdf8e0b080cb17942be4d3e9301b6ae6794321434e83031097057847a8d0c84561026c08fd430a9eade9e5dca8ea76c8336a2caf2533001aee936fb20fcc72",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "browse-timeline-and-formatting"
  },
  "kind": "subject",
  "cites": [],
  "rev": "223mupbn5pefb",
  "seq": 2,
  "of": 14,
  "content": "p2Rib2R5oWdTdWJqZWN0omV0aXRsZXhWRmVhdHVyZTogQnJvd3NlIHJld29yayDigJQgbWFzdGVyLWRldGFpbCwgdGltZWxpbmUsIHBlci1vcCBmb3JtYXR0aW5nLCBzdGF0ZSBzdW1tYXJpZXNsc3ViamVjdF9raW5kZElkZWFlY2l0ZXOAZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2FseB5icm93c2UtdGltZWxpbmUtYW5kLWZvcm1hdHRpbmdpYXJ0aWZhY3RzgaFmQ29tbWl0eChiODI4YzhhODE1Yzc4MDkwMWQ2MjliMWUzNzI0MjM1Y2U2MWQzYzE2aXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWqnmOqhv"
}
---
***8<***
---
{
  "v": 3,
  "cid": "bafyreihylc4k6nhjponhur7svd3hsyc2ihqscc623apax3pec3hmettexq",
  "sig": "8f0b41515a14f648758fbb4953b72b3c58e66d5dd8f571077ace3f77dfe7a536751ffa3e11c52a2581013da2453f32edb8898da7cfbb8c4353e9c0902ecb2e4a",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "browse-timeline-and-formatting"
  },
  "kind": "decision",
  "cites": [
    "bafyreihisrt4pmqvx7l5sxfdgxb4tybkvh5w7tx7h75bfyc57mqary3foy"
  ],
  "rev": "223mupbn6aroo",
  "seq": 3,
  "of": 14,
  "text_len": 337,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIg6JRnx7IVv9fZXKM1w8ngKqn7b87/P/oS4F37IAjjZXZmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx4HmJyb3dzZS10aW1lbGluZS1hbmQtZm9ybWF0dGluZ2lhcnRpZmFjdHOBoWZDb21taXR4KGI4MjhjOGE4MTVjNzgwOTAxZDYyOWIxZTM3MjQyMzVjZTYxZDNjMTZpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZaqeZDXh0="
}
---

RQ-1: Master-detail with a `[Subjects | Timeline]` toggle, not a single-pane layered view — Browse joins the `.view.md` tabs (finishing the slice-1 deferral), the toggle switches the list-pane mode, and the detail pane shows a tapped subject's claims or a tapped claim's detail. This uses the desktop width and gives the timeline room.
***8<***
---
{
  "v": 3,
  "cid": "bafyreibinxpu425uxurbx55i6sgoat6mxck4smdccoyntexsj35eyjbpim",
  "sig": "166d1c5e53e87fea3d8e9afbaf7412b7cc21d92a1c3508ae5dc97d6f7293dabe5aabd58bd3aa0f71b6336eef2b876c344acb4327b5684d1cb9af4c5cd26f13fd",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "browse-timeline-and-formatting"
  },
  "kind": "decision",
  "cites": [
    "bafyreihisrt4pmqvx7l5sxfdgxb4tybkvh5w7tx7h75bfyc57mqary3foy"
  ],
  "rev": "223mupbn6sdtt",
  "seq": 4,
  "of": 14,
  "text_len": 207,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIg6JRnx7IVv9fZXKM1w8ngKqn7b87/P/oS4F37IAjjZXZmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx4HmJyb3dzZS10aW1lbGluZS1hbmQtZm9ybWF0dGluZ2lhcnRpZmFjdHOBoWZDb21taXR4KGI4MjhjOGE4MTVjNzgwOTAxZDYyOWIxZTM3MjQyMzVjZTYxZDNjMTZpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZaqeZMJsE="
}
---

RQ-2: The timeline is flat, newest-first, capped at 200, and filterable — not day-grouped — with a "showing N of M" note; the existing filter box narrows it. The cap keeps the DOM light over 578+ claims.
***8<***
---
{
  "v": 3,
  "cid": "bafyreigxzhtmcfbxswxencepdxrjtvgvzh2jzrvsosmfyt7v2uw4xhdde4",
  "sig": "4e35e26da9517c1e7484d19d5a618252e1ef9e25f7f108fbd4b2f496c897a4f360743af4fa009e32ea7c586d4e4a4d4b28ad1948dc89cf01c5204ed258b0b609",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "browse-timeline-and-formatting"
  },
  "kind": "decision",
  "cites": [
    "bafyreihisrt4pmqvx7l5sxfdgxb4tybkvh5w7tx7h75bfyc57mqary3foy"
  ],
  "rev": "223mupbn7eemo",
  "seq": 5,
  "of": 14,
  "text_len": 290,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIg6JRnx7IVv9fZXKM1w8ngKqn7b87/P/oS4F37IAjjZXZmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx4HmJyb3dzZS10aW1lbGluZS1hbmQtZm9ybWF0dGluZ2lhcnRpZmFjdHOBoWZDb21taXR4KGI4MjhjOGE4MTVjNzgwOTAxZDYyOWIxZTM3MjQyMzVjZTYxZDNjMTZpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZaqeZVKdQ="
}
---

RQ-3: The state summary shows most-recent kind + published + retracted + a neutral status marker, computed client-side; the Status *value* (resolved/blocked/open) is not serialized in the fold, so it is deferred to a later slice that would add it to the fold — this slice stays page-only.
***8<***
---
{
  "v": 3,
  "cid": "bafyreieg6mhdspjeacf2jz5x2sejfx5nzqhfcvdxxlbojyavh6qjv2lciu",
  "sig": "96b0bbc45393b947ffe6841b941bb5395562a285c072c18a178a31880d5f809832cf8c8ef0cf0f9943832f10b173904ea7d2c7010cb8d30bece8e8c766b80fe5",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "browse-timeline-and-formatting"
  },
  "kind": "result",
  "cites": [
    "bafyreihisrt4pmqvx7l5sxfdgxb4tybkvh5w7tx7h75bfyc57mqary3foy"
  ],
  "rev": "223mupbwy6h7j",
  "seq": 6,
  "of": 14,
  "text_len": 2253,
  "content": "p2Rib2R5oWZSZXN1bHShZHRleHRgZWNpdGVzgdgqWCUAAXESIOiUZ8eyFb/X2VyjNcPJ4Cqp+2/O/z/6EuBd+yAI42V2ZmF1dGhvcqJjZGlkeDlkaWQ6a2V5OnpEbmFlU2RCYnJNNk5lWTRpOE1veHVGaXlEdHd4M0FISjhmakVkOTRnckgzMXI4WFllYWdlbnT2Z3N1YmplY3ShZUxvY2FseB5icm93c2UtdGltZWxpbmUtYW5kLWZvcm1hdHRpbmdpYXJ0aWZhY3RzgaFmQ29tbWl0eChiODI4YzhhODE1Yzc4MDkwMWQ2MjliMWUzNzI0MjM1Y2U2MWQzYzE2aXdvcmtzcGFjZaFpV29ya3NwYWNleEBhMjEzMjNmYzY5NzI2N2MyYzM2ODc4NTc1ODY4ZDQ4ZTM5MzJlNDM4ZDA3NzA5YjBhNmViY2YyNGQyYWRmOGVma3JlY29yZGVkX2F0GwAGWqn54jIC"
}
---

generative-build complete for the Browse rework (page-only slice). All changes in src/web/index.html plus test updates in server.rs; no Rust behavior/API change.

Code-change: #view-browse became a master-detail .view.md with a persistent list-pane shell (a [Subjects|Timeline] segmented toggle + the retained #filter + #browse-content) and #browse-detail, reusing slice-1's pane machinery. render() now calls renderBrowse (replacing renderSubjects); renderBrowse fills only #browse-content per browseMode and runs on every fold tick, never touching the detail pane (open detail survives /stream; renderTeloi pattern), and keeps the #subjBadge count. Subjects mode: grouped subject rows each with a state summary (subjectState: most-recent kind + published/retracted/status badges), tap -> openBrowseSubject. Timeline mode: renderTimeline flattens all claims, sorts by recorded_at desc, applies the filter over subject+kind+summary, caps at BROWSE_TIMELINE_CAP=200 with a "showing N of M" note, striking per-op rows (KIND_GLYPH + colored kind badge), tap -> openBrowseClaim. Detail pane: openBrowseSubject (state header + the subject's claims via the shared claimEl) / openBrowseClaim (subject header tappable back to the subject + claimEl). Per-op formatting: KIND_GLYPH map + .kind.<Kind> color rules for all nine kinds (added Publication/Relation, split Retraction->bad). The Status VALUE is not in the fold, so only a neutral status marker is shown (deferred). Removed the now-unused inline subjectEl/expanded. All from the fold; no endpoint change; one include_str! doc, no new dependency.

Evidence: cargo test 214 unit + 14 integration green (new index_html_wires_the_browse_view asserts the shell/toggle/filter, the timeline cap+sort+drill-in, the state summary from Publication/Retraction/Status, the KIND_GLYPH map + a .kind rule for each of the nine kinds, claimEl reuse via paneDetail("browse"), renderBrowse targeting #browse-content, and no new dependency; the responsive test's md/pane counts updated 3->4 as Browse joined the master-detail tabs); clippy -D warnings clean; fmt clean. UNVERIFIED LAYER: the visual render (timeline rows, per-op glyphs/colors, state badges, master-detail geometry at both widths) needs an operator eyeball.
***8<***
---
{
  "v": 3,
  "cid": "bafyreiakmep5esolgg7hc3gl43hikedihuag3hw57vngniwvj4cj7ebwp4",
  "sig": "66813c3168ddfbe18b0fe27d0ad6060aa24c5f04c6ad7fd665349f465899ab9b0a3905d274b02852c7ff239e335bfd7f5d82cc3f211cae03464715f1ea533cdd",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "browse-timeline-and-formatting"
  },
  "kind": "decision",
  "cites": [
    "bafyreihisrt4pmqvx7l5sxfdgxb4tybkvh5w7tx7h75bfyc57mqary3foy"
  ],
  "rev": "223mupcbtva7k",
  "seq": 7,
  "of": 14,
  "text_len": 445,
  "content": "p2Rib2R5oWhEZWNpc2lvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIg6JRnx7IVv9fZXKM1w8ngKqn7b87/P/oS4F37IAjjZXZmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx4HmJyb3dzZS10aW1lbGluZS1hbmQtZm9ybWF0dGluZ2lhcnRpZmFjdHOBoWZDb21taXR4KGI4MjhjOGE4MTVjNzgwOTAxZDYyOWIxZTM3MjQyMzVjZTYxZDNjMTZpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZaqg+dktE="
}
---

adversarial review of browse-timeline-and-formatting: APPROVE-WITH-FOLLOW-UPS — Cold Opus review vs real 586-claim fold: master-detail Browse + timeline + per-op formatting + state summaries all met; live-safe, honest about the un-served Status value, XSS-clean, dependency-free; build+test+clippy+fmt green. F1 (dead subj-tree CSS) and F2 (timeline scroll reset on fold tick) fixed in-round; F3 (open detail is a snapshot) accepted by design.
***8<***
---
{
  "v": 3,
  "cid": "bafyreic6hbfnmmyllpdoblbuswp5gdhq4jbqybbm6u3m24ztxfbxjk7ht4",
  "sig": "bd89bc17aabc2ed04f19e9af2bb6275b61edfb2b1715aa5fb678f01ef92048694ef005cb570f6663bfd1ea5b34c250861c74d5da35c4bdd5b1359f221b25d4d8",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "browse-timeline-and-formatting"
  },
  "kind": "observation",
  "cites": [
    "bafyreiakmep5esolgg7hc3gl43hikedihuag3hw57vngniwvj4cj7ebwp4"
  ],
  "rev": "223mupcbughae",
  "seq": 8,
  "of": 14,
  "text_len": 279,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgCmEf0knLMb5xbMvmzoUQaD0AbZ7d/VpmotVPBJ+QNn9mYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx4HmJyb3dzZS10aW1lbGluZS1hbmQtZm9ybWF0dGluZ2lhcnRpZmFjdHOBoWZDb21taXR4KGI4MjhjOGE4MTVjNzgwOTAxZDYyOWIxZTM3MjQyMzVjZTYxZDNjMTZpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZaqg+mNEg="
}
---

Review finding F1 (FIXED in-round): the Browse rework left ~14 lines of dead CSS (.subj-group, details.subj and its summary/.name/.n/.chev, .claims) after removing the inline subjectEl <details> tree — dead weight against telos/disposable. Removed; grep confirms zero live use.
***8<***
---
{
  "v": 3,
  "cid": "bafyreieiisa2dcqffrxzhvt2m5ud2xcfefe3kzdrwnv4i3fckmby2tnuzm",
  "sig": "8b24a36990e2f76bb1b9bc57a41af5681e55e81a4d556ac33c9897fe1fb46d9f4d4bd5d7217df3cf9247ce4a3cb0fea7e8a93e9fe155adcfeba62b9d443951a5",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "browse-timeline-and-formatting"
  },
  "kind": "observation",
  "cites": [
    "bafyreiakmep5esolgg7hc3gl43hikedihuag3hw57vngniwvj4cj7ebwp4"
  ],
  "rev": "223mupcbv2bsy",
  "seq": 9,
  "of": 14,
  "text_len": 533,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgCmEf0knLMb5xbMvmzoUQaD0AbZ7d/VpmotVPBJ+QNn9mYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx4HmJyb3dzZS10aW1lbGluZS1hbmQtZm9ybWF0dGluZ2lhcnRpZmFjdHOBoWZDb21taXR4KGI4MjhjOGE4MTVjNzgwOTAxZDYyOWIxZTM3MjQyMzVjZTYxZDNjMTZpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZaqg+wHqk="
}
---

Review finding F2 (FIXED in-round): renderBrowse rebuilt #browse-content on every /stream tick (host.innerHTML=''), resetting scroll to top on the long Timeline. Fixed with a cheap browseSig() (mode+filter+subject/claim counts+newest recorded_at): renderBrowse now skips the rebuild when the signature is unchanged, so a fold tick with identical data preserves the list DOM and scroll; a toggle or keystroke still re-renders (mode/filter are in the signature). Also removes the per-tick flatten/sort/render cost when nothing changed.
***8<***
---
{
  "v": 3,
  "cid": "bafyreihn7hk4aq5zxeoacngxowdldve7p3j2rggfdp7inchn4bcoir3ee4",
  "sig": "b9bc950c420f87d38474d4daf72205e466a3f1bb801ac127e2204159760aa4547a144fc3e73d1c47e07167e8919a9557f22facf9c95cf150fc026231978f864b",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "browse-timeline-and-formatting"
  },
  "kind": "observation",
  "cites": [
    "bafyreiakmep5esolgg7hc3gl43hikedihuag3hw57vngniwvj4cj7ebwp4"
  ],
  "rev": "223mupcbvlqlw",
  "seq": 10,
  "of": 14,
  "text_len": 285,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgCmEf0knLMb5xbMvmzoUQaD0AbZ7d/VpmotVPBJ+QNn9mYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx4HmJyb3dzZS10aW1lbGluZS1hbmQtZm9ybWF0dGluZ2lhcnRpZmFjdHOBoWZDb21taXR4KGI4MjhjOGE4MTVjNzgwOTAxZDYyOWIxZTM3MjQyMzVjZTYxZDNjMTZpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZaqg+42bU="
}
---

Review finding F3 (accepted by design, not fixed): an open Browse detail renders fold.claims[subject] once and is a snapshot — a later fold does not refresh it (detail survives the /stream tick by design, REQ-2/REQ-6). Re-tapping refreshes. Consistent with the teloi detail behavior.
***8<***
---
{
  "v": 3,
  "cid": "bafyreifgxezb7hdiagesbrpqznhhrunfvmrqcfbjgzhiiebbuqum6gamc4",
  "sig": "99740b55aefdf937b2e03a48dd274bc645800d220b35f8732848aca1b4c3be5566d03098fdcfcd7135ad6a5e73e9d69b2c6a4ddf308e106c82816098ec73c45e",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "browse-timeline-and-formatting"
  },
  "kind": "observation",
  "cites": [
    "bafyreiakmep5esolgg7hc3gl43hikedihuag3hw57vngniwvj4cj7ebwp4"
  ],
  "rev": "223muqe4w4kzs",
  "seq": 11,
  "of": 14,
  "text_len": 777,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgCmEf0knLMb5xbMvmzoUQaD0AbZ7d/VpmotVPBJ+QNn9mYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx4HmJyb3dzZS10aW1lbGluZS1hbmQtZm9ybWF0dGluZ2lhcnRpZmFjdHOBoWZDb21taXR4KGI4MjhjOGE4MTVjNzgwOTAxZDYyOWIxZTM3MjQyMzVjZTYxZDNjMTZpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZasoXBQdw="
}
---

Eyeball fix (detail-pane horizontal scaling): the operator reported the Browse detail pane wasn't scaling horizontally on resize. Cause: .pane-detail carried a fixed max-width:820px (the responsive-slice F1 readability cap), so as the window widened the detail column grew but the pane content stayed pinned at 820 with dead space beside it. Fix: .pane-detail now fills its grid column (min-width:0 only); the reading width is bounded by the whole two-pane area's .view.md.active max-width:1500px instead of a fixed per-pane cap. Applies to all master-detail tabs for consistency; relaxes the F1 per-pane cap per operator preference (text still wraps, so no horizontal overflow). Updated the responsive test's AC-3 assertion to check the grid-level cap. Tests green, fmt clean.
***8<***
---
{
  "v": 3,
  "cid": "bafyreicuzpmm6ytp3gvx23dswpnpieffz6h24wvgvlv3ptc6m324f3fxti",
  "sig": "3733704c42c99811455989ff702ba65dd6a0a8579642feb45b0e35d3bee9ec1828c9ab8578799ede7936dbb23c4b9598572b1b6674183c8d2ca8e881923bf8d5",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "browse-timeline-and-formatting"
  },
  "kind": "observation",
  "cites": [
    "bafyreiakmep5esolgg7hc3gl43hikedihuag3hw57vngniwvj4cj7ebwp4"
  ],
  "rev": "223muqeptorje",
  "seq": 12,
  "of": 14,
  "text_len": 982,
  "content": "p2Rib2R5oWtPYnNlcnZhdGlvbqFkdGV4dGBlY2l0ZXOB2CpYJQABcRIgCmEf0knLMb5xbMvmzoUQaD0AbZ7d/VpmotVPBJ+QNn9mYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx4HmJyb3dzZS10aW1lbGluZS1hbmQtZm9ybWF0dGluZ2lhcnRpZmFjdHOBoWZDb21taXR4KGI4MjhjOGE4MTVjNzgwOTAxZDYyOWIxZTM3MjQyMzVjZTYxZDNjMTZpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZasquaWus="
}
---

Eyeball fix round 2 (Browse master-detail sizing): two real bugs. (1) SPECIFICITY: the responsive slice's '#view-now,#view-telos,#view-browse { max-width:820px }' uses id selectors, which beat '.view.md.active { max-width:1500px }' (class); once Teloi (slice2) and Browse (slice3) became master-detail, their two-pane area was still id-capped at 820px -> squeezed with a big right gutter. Fixed by removing #view-telos/#view-browse from the single-pane cap (only #view-now remains single-pane). (2) FULL HEIGHT: .view.md.active had height:100% but the implicit grid row was content-height, so panes didn't reach the bottom; added grid-template-rows:minmax(0,1fr) so both panes fill height and scroll internally. Also centered the capped areas (margin:0 auto on .view.md.active and #view-now) so the fixed-width cap reads as an intentional centered page with symmetric gutters rather than a left-aligned block with an awkward right gutter (operator guidance). Tests green, fmt clean.
***8<***
---
{
  "v": 3,
  "cid": "bafyreihn7levs25fdi42tmy2bzt7drseiv4y7ha4vfbunmxbhythnl3mnq",
  "sig": "f00f58998256ba151be973ae41f0db53c2203405fa1f33a3b81c0670929f4acf3c738300eb4efae8cae70ffe8c46747b16e4d1475ad06de54c40ed56658f0a1a",
  "author": "did:key:zDnaeSdBbrM6NeY4i8MoxuFiyDtwx3AHJ8fjEd94grH31r8XY",
  "subject": {
    "local": "browse-timeline-and-formatting"
  },
  "kind": "publication",
  "cites": [],
  "rev": "223muqmdkcwbd",
  "seq": 13,
  "of": 14,
  "content": "p2Rib2R5oWtQdWJsaWNhdGlvbqFlbGF5ZXJnR2l0VHJlZWVjaXRlc4BmYXV0aG9yomNkaWR4OWRpZDprZXk6ekRuYWVTZEJick02TmVZNGk4TW94dUZpeUR0d3gzQUhKOGZqRWQ5NGdySDMxcjhYWWVhZ2VudPZnc3ViamVjdKFlTG9jYWx4HmJyb3dzZS10aW1lbGluZS1hbmQtZm9ybWF0dGluZ2lhcnRpZmFjdHOBoWZDb21taXR4KDgxZmY2MjQ4YjVkNTM1ZTgzZTAzMzg5MGVhOWQwMzE3NGVkYzY4ZTZpd29ya3NwYWNloWlXb3Jrc3BhY2V4QGEyMTMyM2ZjNjk3MjY3YzJjMzY4Nzg1NzU4NjhkNDhlMzkzMmU0MzhkMDc3MDliMGE2ZWJjZjI0ZDJhZGY4ZWZrcmVjb3JkZWRfYXQbAAZatJMEcHo="
}
---
