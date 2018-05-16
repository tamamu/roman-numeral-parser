
use std::env;
use std::io;

#[derive(Debug, Clone)]
enum Token {
    One(usize),
    Five(usize)
}

#[derive(Debug)]
enum ParseError {
    UnknownCharacter,
    NoInput,
    IllegalAlignment,
    Unknown
}

struct Parser {
    src: String,
    pos: usize,
    toks: Vec<Token>,
    result: usize,
    max: usize
}

impl Parser {
    fn new(src: String) -> Self {
        Parser {
            src: src,
            pos: 0,
            toks: Vec::new(),
            result: 0,
            max: 10001
        }
    }

    fn lex(&mut self) -> Result<(), ParseError> {
        for c in self.src.chars() {
            match c {
                'I' => self.toks.push(Token::One(1)),
                'V' => self.toks.push(Token::Five(1)),
                'X' => self.toks.push(Token::One(10)),
                'L' => self.toks.push(Token::Five(10)),
                'C' => self.toks.push(Token::One(100)),
                'D' => self.toks.push(Token::Five(100)),
                'M' => self.toks.push(Token::One(1000)),
                 _  => {
                     return Err(ParseError::UnknownCharacter);
                 }
            }
        }
        Ok(())
    }

    fn parse_one(&mut self) -> Result<(usize, usize), ParseError> {
        let first = try!(self.toks.get(self.pos).ok_or(ParseError::NoInput)).clone();
        self.pos += 1;
        match first {
            Token::One(n) => {
                Ok((n, n))
            },
            _ => Err(ParseError::IllegalAlignment)
        }
    }

    fn parse_five(&mut self) -> Result<(usize, usize), ParseError> {
        let first = try!(self.toks.get(self.pos).ok_or(ParseError::NoInput)).clone();
        self.pos += 1;
        match first {
            Token::Five(n) => {
                Ok((n*5, n))
            },
            _ => Err(ParseError::IllegalAlignment)
        }
    }

    fn parse(&mut self) -> Result<usize, ParseError> {
        let len = self.toks.len();
        let head = try!(self.toks.get(self.pos).ok_or(ParseError::NoInput)).clone();
        match head {
            Token::One(n) => {
                if n >= self.max {
                    return Err(ParseError::IllegalAlignment);
                }
                self.max = n;
                let (first, first_max) = try!(self.parse_one());
                if self.pos >= len {
                    self.result += first;
                    return Ok(self.result);
                }
                let second = try!(self.toks.get(self.pos).ok_or(ParseError::NoInput)).clone();
                match second {
                    Token::One(m) => {
                        if m == n {
                            self.max = m;
                            self.result += first;
                            let (second, second_max) = try!(self.parse_one());
                            if self.max == second_max {
                                self.result += second;
                            } else {
                                self.pos -= 1;
                                return Ok(self.result);
                            }
                            if self.pos >= len { return Ok(self.result); }
                            let (third, third_max) = try!(self.parse_one());
                            if self.max == third_max {
                                self.result += third;
                            } else {
                                self.pos -= 1;
                                return Ok(self.result);
                            }
                            if self.pos >= len { return Ok(self.result); }
                        } else if m == n * 10 {
                            let (second, second_max) = try!(self.parse_one());
                            self.result += second - first;
                        } else if m < n {
                            self.result += first;
                        } else {
                            return Err(ParseError::IllegalAlignment);
                        }
                        Ok(self.result)
                    },
                    Token::Five(n) => {
                        let (second, second_max) = try!(self.parse_five());
                        if self.max == second_max {
                            self.result += second - first;
                        } else if self.max > second_max {
                            self.result += first;
                            self.pos -= 1;
                        }
                        Ok(self.result)
                    },
                    _ => Err(ParseError::IllegalAlignment)
                }
            },
            Token::Five(n) => {
                if n >= self.max {
                    return Err(ParseError::IllegalAlignment);
                }
                self.max = n;
                let (first, first_max) = try!(self.parse_five());
                self.result += first;
                if self.pos >= len { return Ok(self.result); }
                let (second, second_max) = try!(self.parse_one());
                if first_max == second_max {
                    self.result += second;
                } else {
                    return Err(ParseError::IllegalAlignment);
                }
                if self.pos >= len { return Ok(self.result); }
                let (third, third_max) = try!(self.parse_one());
                if first_max == third_max {
                    self.result += third;
                } else {
                    return Err(ParseError::IllegalAlignment);
                }
                if self.pos >= len { return Ok(self.result); }
                let (fourth, fourth_max) = try!(self.parse_one());
                if first_max == fourth_max {
                    self.result += fourth;
                } else {
                    return Err(ParseError::IllegalAlignment);
                }
                Ok(self.result)
            }
        }
    }
}

fn parse(src: &str) -> usize {
    let mut parser = Parser::new(src.to_string());
    parser.lex();
    let len = parser.toks.len();
    while parser.pos < len {
        parser.parse().expect("parsing error");
    }
    parser.result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_len = args.len();
    if args_len > 1 {
        println!("{}", parse(&args[0]));
    } else {
        let stdin = io::stdin();
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();
        println!("{}", parse(&buf));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(3999, parse("MMMCMXCIX"))
    }
    #[test]
    fn test2() {
        assert_eq!(12, parse("XII"))
    }
    #[test]
    fn test3() {
        assert_eq!(42, parse("XLII"))
    }
    #[test]
    fn test4() {
        assert_eq!(49, parse("XLIX"))
    }
    #[test]
    fn test5() {
        assert_eq!(89, parse("LXXXIX"))
    }
    #[test]
    fn test6() {
        assert_eq!(299, parse("CCXCIX"))
    }
    #[test]
    fn test7() {
        assert_eq!(493, parse("CDXCIII"))
    }
    #[test]
    fn test8() {
        assert_eq!(1960, parse("MCMLX"))
    }
    #[test]
    fn test9() {
        assert_eq!(2018, parse("MMXVIII"))
    }
}
