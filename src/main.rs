// TODO: write the simple tokenizer and interpreter
//  - TODO: Comment Support / Specials
//  - TODO: Function  Support through Function Composition
//  - TODO: Variables ?
// TODO: write REPL
// TODO: compile to small bytecode
// TODO: write runtime for the bytecode
// TODO: compile to miri and compiole to binary through the rust toolchain
//
#![allow(dead_code)]

#[derive(Debug)]
enum Ops {
    Square,
    Dec,
    Inc,
    Out,
    Comment,
}

#[derive(Default, Debug)]
struct Deadfish {
    program: String,
    stack: Vec<i32>,
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
        for op in program.chars() {
            match op {
                's' => tokens.push(Ops::Square),
                'd' => tokens.push(Ops::Dec),
                'i' => tokens.push(Ops::Inc),
                'o' => tokens.push(Ops::Out),
                // TODO: Comment and Specials Support
                /* '#' => {}, ' '  => {}, '\n' => {}, etc.. */
                _ => {
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
            let current = self.peak();
            match tok {
                Ops::Square => self.stack.push(current * current),
                Ops::Dec => self.stack.push(current - 1),
                Ops::Inc => self.stack.push(current + 1),
                Ops::Out  => println!("{:#?}", current),
                _  => unimplemented!("interpretation: token {:#?} is not supported", tok)
            }
            token_count += 1;
        }

    }
}

fn main() {
    let program = "iiodoio";
    let mut fish = Deadfish::new();
    fish.tokenize(program.to_string());
    fish.run();
}
    
