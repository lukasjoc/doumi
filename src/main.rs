use std::path::PathBuf;
use std::fs;

#[derive(Debug)]
enum Ops {
    Square,
    Dec,
    Inc,
    Out,
}

#[derive(Default, Debug)]
struct Deadfish {
    stack: Box<Vec<i32>>,
    tokens: Vec<Ops>,
}

impl Deadfish {
    fn new() -> Self {
        Self::default()
    }

    fn peak(&self) -> i32 {
        let current_size = self.stack.len();
        if current_size > 0 {
            return self.stack[current_size -1];
        }
        0
    }

    fn tokenize(&mut self, program: String) {
        let mut tokens: Vec<Ops> = Vec::with_capacity(program.len());
        let mut in_comment_scope = false; 
        for op in program.chars() {
            match op {
                's' => { 
                    if in_comment_scope { continue }
                    tokens.push(Ops::Square)
                },
                'd' => { 
                    if in_comment_scope { continue }
                    tokens.push(Ops::Dec)
                },
                'i' => {
                    if in_comment_scope { continue }
                    tokens.push(Ops::Inc)
                },
                'o' => {
                    if in_comment_scope { continue }
                    tokens.push(Ops::Out)
                },
                // Support for single line comments
                '#' => {
                    in_comment_scope = true;
                }
                ' ' => continue,
                '\n' => {
                    in_comment_scope = false;
                    continue
                },
                _ => {
                    if in_comment_scope { continue }
                    unimplemented!("tokenize: op {:#?} is not supported", op)
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
                Ops::Square => self.stack.push(i32::pow(self.peak(), 2)),
                Ops::Dec => self.stack.push(self.peak() -1),
                Ops::Inc => self.stack.push(self.peak() + 1),
                Ops::Out  => println!("{:#?}", self.peak()),
            }

            // checking for the deadfish intrinsics
            if self.peak() < 0 || self.peak() == 256 {
                self.stack.push(0);
            }
            token_count += 1;
        }

    }
}


struct DeadfishArgs {
    sourcefile: std::path::PathBuf,
}

fn main() {
    let sourcefile = std::env::args().nth(1).expect("Usage: deadfish <file>");

    let args = DeadfishArgs {
        sourcefile: PathBuf::from(&sourcefile),
    };

    let program = fs::read_to_string(args.sourcefile)
                        .expect("could not read sourcefile provided");

    let mut fish = Deadfish::new();
    fish.tokenize(program);
    fish.run();
}

