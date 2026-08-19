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

use cospan::comments::{self, Author, Comment, StoredAnchor};
use cospan::substrate;
use cospan::tui;
use cospan::{relocalize, Anchor, Localization, State};
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("demo") => demo(),
        Some("watch") => watch(&args[1..]),
        Some("watch-repo") => watch_repo(&args[1..]),
        Some("subject") => subject_cmd(&args[1..]),
        Some("comment") => comment_cmd(&args[1..]),
        Some("comments") => comments_cmd(&args[1..]),
        _ => {
            eprintln!(
                "usage:\n  cospan demo\n  cospan watch <file> --line <N> [--ctx <N>]\n  \
                 cospan watch-repo <path> [--once]\n  cospan subject <repo> <subject>\n  \
                 cospan comment add <file> --line <N> [--ctx <C>] <body>\n  \
                 cospan comments <file>"
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

fn first_line(s: &str) -> &str {
    s.lines().next().unwrap_or("")
}

// --- Comment sidecar: drop a comment, and list comments with live state ------

fn comment_cmd(args: &[String]) {
    if args.first().map(String::as_str) != Some("add") {
        eprintln!("usage: cospan comment add <file> --line <N> [--ctx <C>] <body>");
        std::process::exit(2);
    }
    let rest = &args[1..];

    // Single pass: --line/--ctx consume their value; the first remaining bare
    // token is the file, the rest join into the body. (Parsing the file after
    // consuming flag values is what stops `--line 1 file` reading "1" as the
    // file.)
    let mut line = 1usize;
    let mut ctx = 2usize;
    let mut file: Option<String> = None;
    let mut body_parts: Vec<&str> = Vec::new();
    let mut i = 0;
    while i < rest.len() {
        match rest[i].as_str() {
            "--line" => {
                line = rest.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(1);
                i += 2;
            }
            "--ctx" => {
                ctx = rest.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(2);
                i += 2;
            }
            other if file.is_none() => {
                file = Some(other.to_string());
                i += 1;
            }
            other => {
                body_parts.push(other);
                i += 1;
            }
        }
    }
    let file = match file {
        Some(f) => f,
        None => {
            eprintln!("comment add needs a <file>");
            std::process::exit(2);
        }
    };
    let body = body_parts.join(" ");
    if body.is_empty() {
        eprintln!("comment add needs a <body>");
        std::process::exit(2);
    }

    let content = match std::fs::read_to_string(&file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("cospan: cannot read {file}: {e}");
            std::process::exit(1);
        }
    };

    let path = comments::sidecar_path(&file);
    let mut existing = match comments::load(&path) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("cospan: {e}");
            std::process::exit(1);
        }
    };

    let anchor = StoredAnchor::capture(&content, line.saturating_sub(1), ctx);
    let created_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_micros() as i64)
        .unwrap_or(0);
    let comment = Comment {
        // created_at plus a per-file counter, so two comments added in the same
        // microsecond still get distinct ids.
        id: format!("c_{created_at}_{}", existing.len()),
        anchor,
        body,
        author: Author {
            who: "human".into(),
            id: std::env::var("USER").unwrap_or_else(|_| "local".into()),
        },
        created_at,
        resolved: false,
    };

    let loc = relocalize(&comment.anchor.as_anchor(), &content);
    existing.push(comment.clone());
    if let Err(e) = comments::save(&path, &existing) {
        eprintln!("cospan: {e}");
        std::process::exit(1);
    }
    println!("added {} → {}", comment.id, path.display());
    println!("  {}  {}", render(&loc), first_line(&comment.body));
}

fn comments_cmd(args: &[String]) {
    let file = match args.first() {
        Some(f) => f.clone(),
        None => {
            eprintln!("usage: cospan comments <file>");
            std::process::exit(2);
        }
    };
    let content = match std::fs::read_to_string(&file) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("cospan: cannot read {file}: {e}");
            std::process::exit(1);
        }
    };
    let path = comments::sidecar_path(&file);
    let mut records = match comments::load(&path) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("cospan: {e}");
            std::process::exit(1);
        }
    };
    if records.is_empty() {
        println!("{file}: (no comments)");
        return;
    }
    let plural = if records.len() == 1 { "" } else { "s" };
    println!("{file}  ({} comment{plural})", records.len());
    for c in &mut records {
        let loc = comments::localize_and_update(c, &content);
        println!("  {}  {}  {}", render(&loc), first_line(&c.body), c.id);
    }
    // Persist the updated last-seen anchors.
    if let Err(e) = comments::save(&path, &records) {
        eprintln!("cospan: {e}");
    }
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
        println!("  {}", c.display_line());
    }
}

// --- P0 spine: watch a kan/day repo, drawn as an interactive TUI ------------

fn watch_repo(args: &[String]) {
    let repo: PathBuf = args
        .iter()
        .find(|a| !a.starts_with("--"))
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    let once = args.iter().any(|a| a == "--once");

    if once {
        // Non-interactive single frame: scriptable, CI-friendly, testable.
        if !repo.join(".kan").is_dir() {
            eprintln!(
                "warning: {} has no .kan/ — is this a kan repo?",
                repo.display()
            );
        }
        let dash = substrate::collect(&repo);
        let state = tui::AppState::new(repo, dash, None);
        print!("{}", tui::plain_frame(&state));
    } else if let Err(e) = tui::run(repo) {
        eprintln!("cospan: tui error: {e}");
        std::process::exit(1);
    }
}
