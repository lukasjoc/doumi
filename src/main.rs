#![allow(dead_code)]
#![allow(unused_assignments)]

use std::fs;
use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};
use rustyline::error::ReadlineError;
use rustyline::validate::MatchingBracketValidator;
use rustyline::{Cmd, Editor, EventHandler, KeyCode, KeyEvent, Modifiers};
use rustyline_derive::{Completer, Helper, Highlighter, Hinter, Validator};

#[derive(Debug)]
enum ApplicationAst {
    Inc,
    Dec,
    Reset,
}

// TODO: save some meta info in the ast
// #[derive(Debug)]
// struct ParseInfo {pos: i64}
// Dec(ParseInfo) ...

#[derive(Debug)]
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
        application: Box<ApplicationAst>,
    },
}

#[derive(Default, Debug)]
struct Deadfish {
    stack: Box<Vec<i64>>,
    // TODO: table ...
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

    fn try_parse_comment(program: &str, at: usize) -> usize {
        let mut advance = 0usize;
        let mut parsed = false;
        let program_at = program.get(at..program.len()).unwrap();
        while (advance < program_at.len()) && !parsed {
            let tok = program_at.chars().nth(advance);
            match tok {
                Some('\n') => {
                    parsed = true;
                    break;
                }
                None | Some(_) => {}
            }
            advance += 1;
        }
        advance
    }

    fn try_parse_blockdef(program: &str, at: usize) -> (usize, crate::Ast) {
        let mut advance = 0usize;
        let mut parsed = false;
        let program_at = program.get(at..program.len()).unwrap();

        let mut identifier = String::new();

        let try_parse_identifier = |program: &str, at: usize| -> (usize, String) {
            let mut advance = 1usize;
            let mut parsed = false;
            let mut identifier = String::new();
            let program_at = program.get(at..program.len()).unwrap();
            while (advance < program_at.len()) && !parsed {
                let tok = program_at.chars().nth(advance);
                match tok {
                    Some(';') => {
                        parsed = true;
                        advance -= 1;
                        break;
                    }
                    Some(' ') => {}
                    Some(other) => {
                        identifier += &other.to_string();
                    }
                    None => {}
                }
                advance += 1;
            }
            (advance, identifier)
        };

        let mut body: Vec<crate::Ast> = Vec::new();
        while (advance < program_at.len()) && !parsed {
            let tok = program_at.chars().nth(advance);
            match tok {
                Some('(') => {
                    println!("block def parsing (");
                    // try_parse_identif
                    let (advance_ident, ident) = try_parse_identifier(&program, advance);
                    identifier = ident;
                    advance += advance_ident;
                }
                Some(';') => {
                    let ast = Self::try_parse_body(&program.to_ascii_lowercase(), advance + 1);
                    body = ast;
                }
                Some(')') => {
                    // somehow it gets advanced to early and then
                    // parse_main doesnt match to the end brace correctly
                    println!("block def parsing )",);
                    parsed = true;
                    advance += 1;
                    break;
                }
                Some(other) => {
                    println!("block def parsing {:?}", other);
                }
                None => {}
            }
            advance += 1;
        }
        (
            advance,
            Ast::BlockDef {
                identifier,
                body: Box::new(body),
            },
        )
    }

    // TODO: error handling
    fn try_parse_body(program: &str, at: usize) -> Vec<crate::Ast> {
        let mut ast: Vec<Ast> = Vec::with_capacity(program.len());
        let mut next = at;
        while next < program.len() {
            let tok = program.to_ascii_lowercase().chars().nth(next);
            match tok {
                Some('i') => ast.push(Ast::Inc),
                Some('d') => ast.push(Ast::Dec),
                Some('s') => ast.push(Ast::Square),
                Some('o') => ast.push(Ast::Out),
                Some('p') => ast.push(Ast::Print),
                Some('r') => ast.push(Ast::Reset),
                Some('j') => ast.push(Ast::JumpStart),
                Some('#') => next += Self::try_parse_comment(&program.to_ascii_lowercase(), next),
                Some('(') => {
                    let (blockdef_advance, blockdef) =
                    Self::try_parse_blockdef(&program.to_ascii_lowercase(), next);
                    ast.push(blockdef);
                    next += blockdef_advance;
                }
                None | Some(' ' | '\n' | '\t' | '\r') => {}
                Some(other) => {
                    Self::error_with_program(&program.to_ascii_lowercase(), other, next);
                    break;
                }
            }
            next += 1
        }
        ast
    }

    fn build_ast(&mut self, program: String) {
        let ast = Self::try_parse_body(&program.to_ascii_lowercase(), 0);
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
            let tok = &self.ast[next];
            match tok {
                Ast::Reset => self.stack.push(0),
                Ast::Dec => self.stack.push(self.peak() - 1),
                Ast::Inc => self.stack.push(self.peak() + 1),
                Ast::Square => self.stack.push(self.peak() * self.peak()),
                Ast::JumpStart => next = 0,
                Ast::Out => self.output_peaked(),
                Ast::Print => self.output_peaked_ascii(),
                other => {
                    // unimplemented!("this token is not yet implemented")
                    eprintln!("operation not supported yet : {:#?}", other);
                    break;
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
