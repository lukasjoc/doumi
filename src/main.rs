#![allow(dead_code)]
#![allow(unused_assignments)]

use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;

use clap::{Parser, Subcommand, ValueEnum};
use rustyline::error::ReadlineError;
use rustyline::validate::MatchingBracketValidator;
use rustyline::{Cmd, Editor, EventHandler, KeyCode, KeyEvent, Modifiers};
use rustyline_derive::{Completer, Helper, Highlighter, Hinter, Validator};

// TODO: save some meta info in the ast
// #[derive(Debug)]
// struct ParseInfo {pos: i64}
// Dec(ParseInfo) ...

#[derive(Debug, Clone)]
enum Ast {
    Dec,
    Inc,
    Reset,
    Square,
    JumpStart,
    Print,
    Out,
    BlockDef {
        identifier: String,
        body: Box<Vec<Ast>>,
    },
    BlockCall {
        identifier: String,
    },
}

#[derive(Default, Debug)]
struct Deadfish {
    stack: Box<Vec<i64>>,
    table: Box<HashMap<String, Box<Vec<Ast>> >>,
    ast: Box<Vec<Ast>>,
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

    fn error_with_program(program: &str, other: char, pos: usize) {
        eprintln!(
            "Error: {:?} is not supported at ({:?}, pos: {:?} )",
            program, other, pos
        );
    }

    fn try_parse_comment(program_at: &str) -> usize {
        let mut advance = 0;
        while advance < program_at.len() {
            let tok = program_at.chars().nth(advance);
            match tok {
                Some('\n') => return advance,
                None | Some(_) => {}
            }
            advance += 1;
        }
        advance
    }

    fn try_parse_blockdef(program_at: &str, ast: &mut Vec<crate::Ast>) -> usize {
        let mut advance = 0;
        let mut body: Vec<crate::Ast> = Vec::new();
        let mut identifier = String::new();

        let try_parse_identifier = |program_at: &str| -> String {
            let mut advance = 0;
            let mut ident: String = String::new();
            while advance < program_at.len() {
                let tok = program_at.chars().nth(advance);
                match tok {
                    Some(';') => return ident,
                    None | Some(' ') => {}
                    Some(other) => ident += &other.to_string()
                }
                advance += 1;
            }
            ident
        };

        while advance < program_at.len() {
            let current_program_at = &mut program_at.get(advance..program_at.len()).unwrap();
            let tok = program_at.chars().nth(advance);
            match tok {
                Some('(') => {
                    advance += 1;
                    let current_program_at = &mut program_at.get(advance..program_at.len()).unwrap();
                    identifier = try_parse_identifier(current_program_at);
                    continue;
                }
                Some(';') => {
                    advance += 1;
                    body = Self::try_parse(&current_program_at);
                    continue;
                }
                Some(')') => {
                    advance += 1;
                    let blockdef = Ast::BlockDef { identifier, body: Box::new(body) };
                    ast.push(blockdef);
                    break;
                }
                None | Some(_) => {}
            }
            advance += 1;
        }
        advance
    }

    fn try_parse_blockcall(program_at: &str, ast: &mut Vec<crate::Ast>) -> usize {
        let mut advance = 0;
        let mut identifier = String::new();

        let try_parse_identifier = |program_at: &str| -> String {
            let mut advance = 0;
            let mut ident: String = String::new();
            while advance < program_at.len() {
                let tok = program_at.chars().nth(advance);
                match tok {
                    Some('.') => return ident,
                    None | Some('@' | ' ') => {}
                    Some(other) => ident += &other.to_string()
                }
                advance += 1;
            }
            ident
        };
        while advance < program_at.len() {
            let tok = program_at.chars().nth(advance);
            match tok {
                Some('@') => {
                    advance += 1;
                    let current_program_at = &mut program_at.get(advance..program_at.len()).unwrap();
                    identifier = try_parse_identifier(current_program_at);
                    continue;
                }
                Some('.') => {
                    advance += 1;
                    let blockcall = Ast::BlockCall { identifier };
                    ast.push(blockcall);
                    break;
                }
                None | Some(_) => {}
            }
            advance += 1;
        }
        advance
    }

    // TODO: better error handling
    fn try_parse(program: &str) -> Vec<crate::Ast> {
        let mut ast: Vec<Ast> = Vec::with_capacity(program.len());
        let mut advanced = 0;
        while advanced < program.len() {
            let program_at = program.get(advanced..program.len()).unwrap();
            let tok = program.to_ascii_lowercase().chars().nth(advanced);
            match tok {
                Some('i') => ast.push(Ast::Inc),
                Some('d') => ast.push(Ast::Dec),
                Some('s') => ast.push(Ast::Square),
                Some('o') => ast.push(Ast::Out),
                Some('p') => ast.push(Ast::Print),
                Some('r') => ast.push(Ast::Reset),
                Some('j') => ast.push(Ast::JumpStart),
                Some('#') => {
                    advanced += Self::try_parse_comment(&program_at);
                    continue;
                },
                Some('(') => {
                    advanced += Self::try_parse_blockdef(&program_at, &mut ast);
                    continue;
                },
                Some('@') => {
                    advanced += Self::try_parse_blockcall(&program_at, &mut ast);
                    continue;
                }
                None | Some(_) => {}
            }
            advanced += 1
        }
        ast
    }

    fn build_ast(&mut self, program: String) {
        let ast = Self::try_parse(&program.to_ascii_lowercase());
        self.ast = Box::new(ast);
    }

    fn output_peaked(&self) {
        print!("{}", self.peak())
    }

    fn output_peaked_ascii(&self) {
        if self.peak() > u8::MAX.into() || self.peak() < u8::MIN.into() {
            self.output_peaked();
            return;
        }
        print!("{}", char::from(self.peak() as u8));
    }

    fn run(&mut self) {
        let mut next = 0usize;
        while next < self.ast.len() {
            // let tok = self.ast[next];
            let tok = &mut self.ast.get(next).unwrap();
            match tok {
                Ast::Reset => self.stack.push(0),
                Ast::Dec => self.stack.push(self.peak() - 1),
                Ast::Inc => self.stack.push(self.peak() + 1),
                Ast::Square => self.stack.push(self.peak() * self.peak()),
                Ast::JumpStart => next = 0,
                Ast::Out => self.output_peaked(),
                Ast::Print => self.output_peaked_ascii(),
                Ast::BlockDef { identifier, body } => {
                    let cloned_body = body.clone();
                    println!("cloned : {:?}", cloned_body);
                    self.table.insert(identifier.to_string(), cloned_body);
                    println!("table : {:#?}", self.table);
                }
                Ast::BlockCall { identifier } => {
                    let (ident, ast) = self.table.get_key_value(identifier).unwrap();
                    println!("ast to apply : ident: {:?} -> {:?}", ident, ast);
                    // TODO: compile indent_ast program stack return addr & appy to main self.stack
                }
            }

            // checking for the deadfish intrinsics
            if self.peak() < 0 || self.peak() == 256 {
                self.stack.push(0);
            }
            next += 1;
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
                match line.as_str().trim_end() {
                    "help" => {
                        println!("type i to increase");
                        println!("type d to decrease");
                        println!("type s to square");
                        println!("type r to reset");
                        println!("type j jump to the start again");
                        println!("type o to ouput raw value");
                        println!("type p to output value utf-8 decoded (fallback to raw, when output is not in the range {:?}-{:?} is automatic)", u8::MIN, u8::MAX);
                        println!("type # to comment something");
                        println!("type help to print this help");
                        println!("type ast to print currently-parsed AST");
                        println!("type CTRL-s to go into multi line mode");

                    }
                    "ast" => {
                        println!("{:?}", fish.ast);
                    }
                    _ => {
                        // TODO: wrap these in Results to handle errors better
                        fish.build_ast(line.to_string());
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum EmitOpts {
    Ast,
}

#[derive(Subcommand)]
enum Cmds {
    #[command(about = "Execute a source file", long_about = None)]
    Exec {
        #[arg(short, long, value_name = "SOURCE_FILE", required = true)]
        file: Option<PathBuf>,

        #[arg(long, value_name = "EMIT_ONLY", value_enum)]
        emit: Option<EmitOpts>,
    },
}

#[derive(Parser)]
#[command(author, version)]
#[command(about = "Doumi's Interpreter and REPL Mode", long_about = None )]
struct Args {
    #[command(subcommand)]
    exec: Option<Cmds>,
}

fn main() {
    let args = Args::parse();
    match args.exec {
        Some(Cmds::Exec { file, emit }) => {
            if let Some(file) = file.as_deref() {
                let mut fish = Deadfish::new();
                let program = fs::read_to_string(file).expect("could not read sourcefile provided");
                fish.build_ast(program);
                match emit {
                    Some(EmitOpts::Ast) => {
                        println!("{:?}", fish.ast);
                        ()
                    }
                    None => {
                        fish.run();
                        print!("\n");
                    }
                }
            }
        }
        None => match repl() {
            Ok(_) => {}
            Err(_) => {}
        },
    }
}
