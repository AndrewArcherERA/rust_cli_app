use std::io;
use std::io::Write;
use dialoguer::Select;

pub fn run_cli() {
    loop {
        println!("Which number of the Fibonacci Sequence would you like to see?");
        print!("Type a number: ");
        let _ = io::stdout().flush();

        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("Failed to read line");

        let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number");
                continue;
            }
        };

        let result = fib(n);
        println!("Output: {result}");

        let options = vec!["See another number", "Quit"];
        let options_selection = Select::new()
            .with_prompt("What would you like to do?")
            .items(&options)
            .interact()
            .unwrap();

        if options_selection == 1 {
            break
        }
    }
}

fn fib(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }

    fib(n - 1) + fib(n - 2)
}
