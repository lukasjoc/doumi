use std::fs;
use std::path::PathBuf;

use clap::Parser;
use rustyline::error::ReadlineError;
use rustyline::validate::MatchingBracketValidator;
use rustyline::{Cmd, Editor, EventHandler, KeyCode, KeyEvent, Modifiers};
use rustyline_derive::{Completer, Helper, Highlighter, Hinter, Validator};

#[derive(Debug)]
enum Ops {
    Dec,
    Inc,
    Reset,
    Square,
    Print,
    Out,
}

#[derive(Default, Debug)]
struct Deadfish {
    stack: Box<Vec<i64>>,
    tokens: Vec<Ops>,
}

impl Deadfish {
    fn new() -> Self {
        Self::default()
    }

    fn peak(&self) -> i64 {
        let current_size = self.stack.len();
        if current_size > 0 {
            return self.stack[current_size - 1];
        }
        0
    }

    fn error_with_program(program: String) {
        println!("Error: {:?} is not supported", program);
    }

    fn tokenize(&mut self, program: String) {
        let mut tokens: Vec<Ops> = Vec::with_capacity(program.len());
        let mut in_comment_scope = false;
        for op in program.chars() {
            match op.to_ascii_lowercase() {
                's' => {
                    if in_comment_scope {
                        continue;
                    }
                    tokens.push(Ops::Square)
                }
                'd' => {
                    if in_comment_scope {
                        continue;
                    }
                    tokens.push(Ops::Dec)
                }
                'i' => {
                    if in_comment_scope {
                        continue;
                    }
                    tokens.push(Ops::Inc)
                }
                'o' => {
                    if in_comment_scope {
                        continue;
                    }
                    tokens.push(Ops::Out)
                }
                'r' => {
                    if in_comment_scope {
                        continue;
                    }
                    tokens.push(Ops::Reset)
                }
                'p' => {
                    if in_comment_scope {
                        continue;
                    }
                    tokens.push(Ops::Print)
                }
                // Support for single line comments
                // # SOME_COMMENT \n
                '#' => {
                    in_comment_scope = true;
                }
                '\n' => {
                    in_comment_scope = false;
                    continue;
                }
                ' ' => continue,
                _ => {
                    if in_comment_scope {
                        continue;
                    }
                    Self::error_with_program(program);
                    break;
                }
            }
        }
        self.tokens = tokens;
    }

    fn run(&mut self) {
        let token_size = self.tokens.len();
        let mut token_count = 0usize;

        while token_count < token_size {
            let tok = &self.tokens[token_count];
            match tok {
                Ops::Reset => self.stack.push(0),
                Ops::Dec => self.stack.push(self.peak() - 1),
                Ops::Inc => self.stack.push(self.peak() + 1),
                Ops::Square => self.stack.push(self.peak() * self.peak()),
                Ops::Out => println!("{}", self.peak()),
                Ops::Print => {
                    if self.peak() > u8::MAX.into() || self.peak() < u8::MIN.into() {
                        println!("{}", self.peak());
                    } else {
                        print!("{}", char::from(self.peak() as u8));
                    }
                }
            }

            // checking for the deadfish intrinsics
            if self.peak() < 0 || self.peak() == 256 {
                self.stack.push(0);
            }
            token_count += 1;
        }
    }
}

#[derive(Completer, Helper, Highlighter, Hinter, Validator)]
struct InputValidator {
    #[rustyline(Validator)]
    brackets: MatchingBracketValidator,
}

fn repl() -> rustyline::Result<()> {
    let h = InputValidator {
        brackets: MatchingBracketValidator::new(),
    };
    let mut rl = Editor::new()?;

    rl.set_helper(Some(h));
    rl.bind_sequence(
        KeyEvent(KeyCode::Char('s'), Modifiers::CTRL),
        EventHandler::Simple(Cmd::Newline),
    );

    let mut fish = Deadfish::new();
    print!("Doumi v0.1.0\n");
    print!("Type  help  for info about available commands\n\n");
    loop {
        let readline = rl.readline(">>> ");
        match readline {
            Ok(line) => {
                match line.as_str() {
                    "help" => {
                        println!("type i to increase");
                        println!("type d to decrease");
                        println!("type s to square");
                        println!("type r to reset");
                        println!("type o to ouput raw value");
                        println!("type p to output value utf8 decoded (fallback to raw, when output is not in the range {:?}-{:?} is automatic)", u8::MIN, u8::MAX);
                        println!("type # to comment something");
                        println!("type help to print this help");
                        println!("type tokens to print currently-parsed tokens");
                        println!("type CTRL-s to go into multi line mode");
                    }
                    "tokens" => {
                        println!("{:#?}", fish.tokens);
                    }
                    _ => {
                        // TODO: wrap these in Results to handle errors better
                        fish.tokenize(line.to_string());
                        fish.run();
                        print!("\n");
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("Caught Interrupt. Type CTRL-D to quit the REPL (to reset type ro)");
            }
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}

#[derive(Parser, Debug)]
struct DoumiArgs {
    #[arg(short, long, value_name = "SOURCE_FILE")]
    file: Option<PathBuf>,
}

fn main() {
    let args = DoumiArgs::parse();
    if let Some(file) = args.file.as_deref() {
        let mut fish = Deadfish::new();
        let program = fs::read_to_string(file).expect("could not read sourcefile provided");
        fish.tokenize(program);
        fish.run();
        print!("\n");
    } else {
        match repl() {
            Ok(_) => {}
            Err(_) => {}
        }
    }
}
