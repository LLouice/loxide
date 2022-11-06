use std::env;
use std::io::{self, BufRead, Write};
use std::path::Path;

use loxide::Lexer;

fn main() {
    let mut args = env::args();
    match args.size_hint() {
        (1, _) => {
            run_prompt();
        }
        (2, _) => run_file(Path::new(&args.nth(1).unwrap())),
        (_, _) => {
            println!("Usage: loxide: loxide [script]");
            std::process::exit(64);
        }
    }
}

fn run_prompt() {
    let mut stdin = io::stdin().lock();
    let mut line = String::new();
    let mut stdout = io::stdout().lock();
    loop {
        print!("> ");
        stdout.flush().unwrap();

        line.clear();
        stdin.read_line(&mut line).unwrap();

        let line_cleaned = line.trim();
        if line_cleaned == ":q" || line_cleaned == ":quit" {
            break;
        }
        run(&line);
    }
}

fn run(source: &str) {
    let mut lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens();
    tokens.into_iter().for_each(|tok| println!("{tok}"));
}

fn run_file(file: impl AsRef<Path>) {
    let file = file.as_ref();
    println!("run script: {}", file.display());
    println!("run script: {}", file.canonicalize().unwrap().display());
}
