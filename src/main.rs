
enum Token {
    One(usize),
    Five(usize)
}

struct Parser {
    src: String,
    pos: usize,
    toks: Vec<Token>
}

fn main() {
    println!("Hello, world!");
}
