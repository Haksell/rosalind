mod problems;

use crate::problems::PROBLEMS;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        argument_error("Invalid number of arguments.")
    } else {
        match PROBLEMS.get(&args[1]) {
            Some(solve_function) => println!("{}", solve_function()),
            None => argument_error(&format!("Problem {} is not implemented yet.", args[1])),
        }
    }
}

fn argument_error(message: &str) {
    eprintln!("{}", message);
    eprintln!("Usage:");
    eprintln!("  cargo run -- <name>  # Runs the problem with the given name");
    std::process::exit(1);
}
