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
//! Poll, don't subscribe: the whole kan/day substrate has no push channel, so the
//! live tool watches by polling mtime + re-folding. This mirrors that.

use cospan::{relocalize, Anchor, Localization, State};
use std::time::Duration;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("demo") => demo(),
        Some("watch") => watch(&args[1..]),
        _ => {
            eprintln!("usage:\n  cospan demo\n  cospan watch <file> --line <N> [--ctx <N>]");
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
