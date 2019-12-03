use std::io::stdin;
use std::io::stdout;
use std::io::Result;
use std::io::Write;

fn read(input: &str) -> &str {
    input
}

fn eval(input: &str) -> &str {
    input
}

fn print(input: &str) -> &str {
    input
}

fn rep(input: &str) -> &str {
    let read_result = read(input);
    let eval_result = eval(read_result);
    print(eval_result)
}

fn main_loop() -> Result<()> {
    let mut user_input = String::new();
    loop {
        print!("user> ");
        stdout().flush()?;
        match stdin().read_line(&mut user_input) {
            Ok(bytes_read_count) => {
                if bytes_read_count == 0 {
                    println!("\nGot EOF input. Aborting");
                    break;
                }
            }
            Err(error) => return Err(error),
        }
        let rep_result = rep(&user_input);
        print!("{}", rep_result);
        user_input.clear();
    }
    Ok(())
}

fn main() {
    if let Err(error) = main_loop() {
        eprintln!("{}", error);
    }
}
