use std::{env, process};

use nom::{character::complete::digit0, IResult};

fn parse_number(input: &str) -> IResult<&str, isize> {
    let pair = digit0(input)?;
    let digit = pair.1.parse::<isize>().unwrap_or_default();
    Ok((pair.0, digit))
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
    println!("  .globl main");
    println!("main:");
    let mut p = parse_number(p.as_str()).expect("Fail to parse number");
    println!("  mov ${}, %rax", p.1);

    while !p.0.is_empty() {
        let mut chars = p.0.chars();
        match chars.next() {
            Some('+') => {
                p = parse_number(chars.as_str()).expect("Fail to parse number");
                println!("  add ${}, %rax", p.1);
            }
            Some('-') => {
                p = parse_number(chars.as_str()).expect("Fail to parse number");
                println!("  sub ${}, %rax", p.1);
            }
            c => {
                eprintln!("unexpected character: {:?}", c);
                process::exit(1);
            }
        }
    }

    println!("  ret");
}
