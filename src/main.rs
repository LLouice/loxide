use std::env;
use std::io::{self, BufRead, Write};
use std::path::Path;

use loxide::{Lexer};

fn main() {
    let args = env::args();
    match args.size_hint() {
        (1, _) => {
            run_prompt();
        }
        (2, _) => run_file(Path::new(&args.skip(1).next().unwrap())),
        (_, _) => {
            println!("Usage: loxide: loxide [script]");
            std::process::exit(64);
        }
    }
}

#[inline(always)]
fn print_prompt() {}

fn run_prompt() {
    let mut stdin = io::stdin().lock();
    let mut line = String::new();
    let mut stdout = io::stdout().lock();
    loop {
        print!("> ");
        stdout.flush().unwrap();

        line.clear();
        stdin.read_line(&mut line).unwrap();

        let line = line.trim();
        if line == ":q" || line == ":quit" {
            break;
        }
        run(&line);
    }
}

fn run(source: &str) {
    println!("{source}");
    let lexer = Lexer::new(source);
    let tokens = lexer.parse_tokens();
    tokens.into_iter().for_each(|tok|println!("{tok}"));
}

fn run_file(file: impl AsRef<Path>) {
    let file = file.as_ref();
    println!("run script: {}", file.display());
    println!("run script: {}", file.canonicalize().unwrap().display());
}
