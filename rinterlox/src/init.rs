use std::env;
use std::fs;
use std::io;
use std::io::Write;

use super::scanner::token::Token;
use super::scanner::scanner::Scanner;

pub struct Lox {
    had_err: bool,
}

impl Lox {
    pub fn new() -> Self {
        Self {
            had_err: false,
        }
    }

    pub fn start(&mut self) {
        let args: Vec<String> = env::args().collect();
        if args.len() > 2 {
            println!("[Usage]: jlox [script]");
            std::process::exit(64);
        } else if args.len() == 2 {
            self.run_file(&args[1]);
        } else {
            let _ = self.run_prompt();
        }
    }

    fn run_file(&mut self, path: &str) {
        let file_path = std::path::Path::new(&path);
        let input = fs::read_to_string(&file_path).expect("[Error]: Cannot read file.");
        self.run(&input);
        if self.had_err { std::process::exit(65) }
    }

    fn run_prompt(&mut self) -> io::Result<()> {
        let mut buffer = String::new();
        let stdin = io::stdin();

        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            match stdin.read_line(&mut buffer) {
                Ok(n) => {
                    if n == 0 { break; }
                    self.run(&buffer);
                    self.had_err = false;
                    buffer.clear();
                },
                Err(error) => println!("Error: {error}"),
            }
        }
        Ok(())
    }

    fn run(&mut self, input: &str) {
        let mut scanner: Scanner = Scanner::new(input);
        let tokens: Vec<Token> = scanner.scan_tokens(self);

        for tok in tokens {
            println!("{:?}", tok);
        }
    }

    pub fn error(&mut self, line: usize, col: usize, msg: &str) {
        self.report(line, col, "", msg);
    }

    fn report(&mut self, line: usize, col: usize, poc: &str, msg: &str) {
        println!("[line {}: col {}] Error{}: {}", line, col, poc, msg);
        self.had_err = true;
    }
}
