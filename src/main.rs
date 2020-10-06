use std::{env, process};

use nom::{
    branch::alt,
    character::complete::{char as char1, digit1},
    IResult,
};

/// Token kind
enum TokenKind {
    /// Keywords or punctuators
    RESERVED(char),
    /// Numeric literals
    NUM(isize),
    /// End-of-file markers
    EOF,
}

/// Token type
struct Token {
    kind: TokenKind,
    pos: usize,
}

impl Token {
    pub fn new(kind: TokenKind, pos: usize) -> Self {
        Token { kind, pos }
    }
}

fn tokenize(p: &str) -> IResult<&str, Vec<Token>> {
    let s = p;
    let mut pos = 0;
    let mut p = p;
    let mut tokens = Vec::new();
    while !p.is_empty() {
        if let Ok((remaining, num)) = parse_number(p) {
            tokens.push(Token::new(TokenKind::NUM(num), pos));
            pos += p.len() - remaining.len();
            p = remaining;
        } else if let Ok((remaining, pun)) = parse_punctuators(p) {
            tokens.push(Token::new(TokenKind::RESERVED(pun), pos));
            pos += p.len() - remaining.len();
            p = remaining;
        } else {
            exit_with_stderr(s, pos, "Invalid token");
        }
    }

    tokens.push(Token::new(TokenKind::EOF, pos));
    Ok((p, tokens))
}

fn parse_number(input: &str) -> IResult<&str, isize> {
    let pair = digit1(input)?;
    let digit = pair.1.parse::<isize>().unwrap_or_default();
    Ok((pair.0, digit))
}

fn parse_punctuators(input: &str) -> IResult<&str, char> {
    alt((char1('+'), char1('-')))(input)
}

fn exit_with_stderr(s: &str, pos: usize, msg: &str) {
    eprintln!("{}", s);
    (0..pos).into_iter().for_each(|_| eprint!(" "));
    eprintln!("^ {}", msg);
    process::exit(1);
}

fn main() {
    let mut args = env::args();
    if args.len() != 2 {
        eprintln!(
            "{}: invalid number of arguments",
            args.next().expect("Fail to get binary name.")
        );
        process::exit(1);
    }

    let p = args.nth(1).unwrap();
    let tokens = tokenize(&p).expect("Fail to create tokens");
    let mut tokens = tokens.1.iter();
    println!("  .globl main");
    println!("main:");
    let first = tokens.next().expect("No token created.");
    let num = match first.kind {
        TokenKind::NUM(n) => n,
        _ => 0,
    };
    println!("  mov ${}, %rax", num);

    while let Some(Token {
        kind: TokenKind::RESERVED(op),
        pos,
    }) = tokens.next()
    {
        if *op == '+' {
            match tokens.next() {
                Some(Token {
                    kind: TokenKind::NUM(num),
                    pos: _,
                }) => println!("  add ${}, %rax", *num),
                _ => exit_with_stderr(&p, *pos + 1, "Expect a number"),
            }
        }
        if *op == '-' {
            match tokens.next() {
                Some(Token {
                    kind: TokenKind::NUM(num),
                    pos: _,
                }) => println!("  sub ${}, %rax", *num),
                _ => exit_with_stderr(&p, *pos + 1, "Expect a number"),
            }
        }
    }

    println!("  ret");
}
