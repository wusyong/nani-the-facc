use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("{}: invalid number of arguments", args[0]);
        process::exit(1);
    }

    print!("  .globl main\n");
    print!("main:\n");
    print!(
        "  mov ${}, %rax\n",
        args[1].parse::<i32>().unwrap_or_default()
    );
    print!("  ret\n");
}
