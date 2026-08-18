//! cospan — prototype driver.
//!
//!   cospan demo
//!       Anchor a comment to a line, then apply a scripted sequence of edits an
//!       agent might make, printing the comment's localization at each step.
//!
//!   cospan watch <file> --line <N> [--ctx <N>]
//!       Pin a comment to line N (1-based) of <file>, then poll the file and
//!       re-localize live as you (or an agent) edit it. Ctrl-C to stop.
//!
//!   cospan watch-repo <path> [--once]
//!       P0 spine: watch a kan/day repo's `.kan/log/HEAD` and, on every change,
//!       re-fold and redraw a dashboard — process position (day), sessions, and a
//!       claims-by-subject summary (kan). `--once` renders a single frame.
//!
//! Poll, don't subscribe: the whole kan/day substrate has no push channel, so the
//! live tool watches by polling mtime + re-folding. This mirrors that.

use cospan::substrate::{self, Dashboard};
use cospan::{relocalize, Anchor, Localization, State};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("demo") => demo(),
        Some("watch") => watch(&args[1..]),
        Some("watch-repo") => watch_repo(&args[1..]),
        Some("subject") => subject_cmd(&args[1..]),
        _ => {
            eprintln!(
                "usage:\n  cospan demo\n  cospan watch <file> --line <N> [--ctx <N>]\n  \
                 cospan watch-repo <path> [--once]\n  cospan subject <repo> <subject>"
            );
            std::process::exit(2);
        }
    }
}

fn render(loc: &Localization) -> String {
    let tag = match loc.state {
        State::Anchored => "ANCHORED   ",
        State::Drifted => "DRIFTED    ",
        State::Unresolvable => "UNRESOLVED ",
    };
    let where_ = match loc.span {
        Some((a, b)) if a == b => format!("line {}", a + 1),
        Some((a, b)) => format!("lines {}-{}", a + 1, b + 1),
        None => "—".to_string(),
    };
    format!("{tag} {where_:<12} conf {:.2}", loc.confidence)
}

fn demo() {
    let v0 = "\
fn login(user: &str) -> bool {
    let token = fetch_token(user);
    validate(token)
}";
    // Pin the comment to the `let token` line (0-based line 1).
    let anchor = Anchor::from_file(v0, 1, 1);
    println!("comment pinned to: {:?}\n", anchor.target.trim());

    let steps: [(&str, String); 4] = [
        ("v0  unchanged", v0.to_string()),
        (
            "v1  agent prepends docs (pure line-shift)",
            format!("/// Logs a user in.\n/// Returns true on success.\n{v0}"),
        ),
        (
            "v2  agent renames the call (target text edited)",
            v0.replace("fetch_token(user)", "fetch_token_cached(user)"),
        ),
        (
            "v3  agent rewrites the whole function (target gone)",
            "fn login(u: &str) -> Result<Session, AuthError> {\n    \
             Session::establish(u)\n}"
                .to_string(),
        ),
    ];

    for (label, content) in steps {
        let loc = relocalize(&anchor, &content);
        println!("{:<46} {}", label, render(&loc));
    }
    println!("\n(v3 lands on the resolve-by-hand list — exactly what should happen.)");
}

fn watch(args: &[String]) {
    let file = match args.first() {
        Some(f) => f.clone(),
        None => {
            eprintln!("watch needs a <file>");
            std::process::exit(2);
        }
    };
    let mut line = 1usize;
    let mut ctx = 2usize;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--line" => {
                line = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(1);
                i += 2;
            }
            "--ctx" => {
                ctx = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(2);
                i += 2;
            }
            _ => i += 1,
        }
    }

    let initial = match std::fs::read_to_string(&file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("cannot read {file}: {e}");
            std::process::exit(1);
        }
    };
    let anchor = Anchor::from_file(&initial, line.saturating_sub(1), ctx);
    println!("watching {file}");
    println!("comment pinned to line {line}: {:?}", anchor.target.trim());
    println!("edit the file; Ctrl-C to stop.\n");

    let mut last = String::new();
    loop {
        if let Ok(content) = std::fs::read_to_string(&file) {
            if content != last {
                let loc = relocalize(&anchor, &content);
                println!("{}", render(&loc));
                last = content;
            }
        }
        std::thread::sleep(Duration::from_millis(250));
    }
}

// --- Per-subject claim drill-in: one-shot read of a subject's live claims ---

fn subject_cmd(args: &[String]) {
    let (repo, subject) = match (args.first(), args.get(1)) {
        (Some(r), Some(s)) => (PathBuf::from(r), s.clone()),
        _ => {
            eprintln!("usage: cospan subject <repo> <subject>");
            std::process::exit(2);
        }
    };

    let claims = match substrate::subject_claims(&repo, &subject) {
        Ok(claims) => claims,
        Err(e) => {
            eprintln!("cospan: {subject}: {e}");
            std::process::exit(1);
        }
    };

    if claims.is_empty() {
        // kan's model has no "unknown subject": a subject is its claims, so an
        // empty fold is indistinguishable from a never-used name. Say so rather
        // than let a bare "0 claims" read as a lookup that succeeded emptily.
        println!("{subject}  (no live claims — unused, or all claims retracted)");
        return;
    }

    let plural = if claims.len() == 1 { "" } else { "s" };
    println!("{subject}  ({} live claim{plural})", claims.len());
    for c in &claims {
        println!(
            "  {:<11} {:<8}  {:<16}  {}",
            c.kind,
            c.short_author(),
            c.recorded_utc(),
            c.summary()
        );
    }
}

// --- P0 spine: watch a kan/day repo and redraw on change --------------------

fn watch_repo(args: &[String]) {
    let repo: PathBuf = args
        .iter()
        .find(|a| !a.starts_with("--"))
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    let once = args.iter().any(|a| a == "--once");

    if !repo.join(".kan").is_dir() {
        eprintln!("warning: {} has no .kan/ — is this a kan repo?", repo.display());
    }
    let head = repo.join(".kan/log/HEAD");

    let mut last: Option<SystemTime> = None;
    let mut tick: u64 = 0;
    loop {
        let m = std::fs::metadata(&head).and_then(|md| md.modified()).ok();
        if last.is_none() || m != last {
            tick += 1;
            let dash = substrate::collect(&repo);
            render_dashboard(&repo, &dash, tick);
            last = m;
        }
        if once {
            break;
        }
        std::thread::sleep(Duration::from_millis(500));
    }
}

fn render_dashboard(repo: &Path, dash: &Dashboard, tick: u64) {
    print!("\x1b[2J\x1b[H"); // clear screen + cursor home
    let rule = "─".repeat(64);

    println!("cospan · {}  (fold #{tick})", repo.display());
    println!("{rule}");

    // Process position — day's own honest, ambiguity-preserving text.
    println!("PROCESS  (day status)");
    match &dash.day_status {
        Some(text) if !text.is_empty() => {
            for line in text.lines().take(14) {
                println!("  {line}");
            }
            if text.lines().count() > 14 {
                println!("  … (run `day status` for the full picture)");
            }
        }
        _ => println!("  (unavailable)"),
    }
    println!("{rule}");

    // Sessions — the flat agents/handoff registry (no hierarchy yet; see 02).
    let sessions = dash.sessions();
    println!("SESSIONS  (agents/handoff · {} live)", sessions.len());
    for s in &sessions {
        let short = s.name.trim_start_matches("agents/handoff/");
        println!("  · {:<28} [{}]", short, s.state);
    }
    if sessions.is_empty() {
        println!("  (none)");
    }
    println!("{rule}");

    // Claims by subject namespace.
    println!("CLAIMS  ({} subjects total)", dash.subjects.len());
    for (ns, n) in dash.namespace_counts() {
        println!("  {n:>4}  {ns}");
    }

    if !dash.errors.is_empty() {
        println!("{rule}");
        println!("NOTES");
        for e in &dash.errors {
            println!("  ! {e}");
        }
    }
    println!("{rule}");
    println!("watching {} · Ctrl-C to stop", repo.join(".kan/log/HEAD").display());
}
