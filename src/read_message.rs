use std::borrow::Cow::{self, Owned};

use rustyline::error::ReadlineError;
use rustyline::Context;
use rustyline::Result;
use rustyline::{
    completion::{Completer, Pair},
    highlight::Highlighter,
    hint::Hinter,
    validate::Validator,
    Editor, Helper,
};

struct RustKeywordCompleter;

impl Completer for RustKeywordCompleter {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        _pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        let keywords = vec![
            "fn", "let", "mut", "struct", "enum", "impl", "for", "loop", "match",
        ];
        let mut completions = Vec::new();
        for keyword in keywords {
            if keyword.starts_with(line) {
                completions.push(Pair {
                    display: keyword.to_string(),
                    replacement: keyword.to_string(),
                });
            }
        }

        Ok((0, completions))
    }
}

impl Hinter for RustKeywordCompleter {
    type Hint = String;

    fn hint(&self, line: &str, _pos: usize, _ctx: &Context<'_>) -> Option<Self::Hint> {
        let change_types = [
            "feat", "fix", "docs", "style", "refactor", "perf", "test", "chore",
        ];
        let mut hint_text = String::new();
        if !line.is_empty() {
            for change_type in change_types {
                if change_type.starts_with(line) {
                    hint_text.push_str(change_type.replace(line, "").as_str());
                    hint_text.push_str(": ");
                    break;
                }
            }
        }
        Some(hint_text)
    }
}

impl Highlighter for RustKeywordCompleter {
    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned("\x1b[2m".to_owned() + hint + "\x1b[m")
    }
}
impl Validator for RustKeywordCompleter {}
impl Helper for RustKeywordCompleter {}

pub fn read_message() -> Result<String> {
    // `()` can be used when no completer is required
    let mut rl = Editor::new()?;
    rl.set_helper(Some(RustKeywordCompleter));
    loop {
        let readline = rl.readline("❯ ");
        match readline {
            Ok(line) => {
                return Ok(line);
            }
            Err(ReadlineError::Interrupted) => {
                // println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                // println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    panic!("sorry man")
}
