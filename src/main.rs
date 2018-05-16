
enum Token {
    One(usize),
    Five(usize)
}

struct Parser {
    src: String,
    pos: usize,
    toks: Vec<Token>
}

impl Parser {
    fn new(src: String) -> Self {
        Parser {
            src: src,
            pos: 0,
            toks: Vec::new()
        }
    }
}

fn main() {
    println!("Hello, world!");
}
