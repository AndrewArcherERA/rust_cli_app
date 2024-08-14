mod fahrenheit_celcius;
mod nth_fib;
mod twelve_days_of_xmas;

use dialoguer::Select;

fn main() {
    println!("");
    loop {
        let options = vec![
            "Convert tempeture to Fahrenheit or Celcius",
            "Get the nth number in the Fibonocci Sequence",
            "Print The Twelve Days Of Christmas Lyrics",
            "Quit"
        ];
        let options_selection = Select::new()
            .with_prompt("What mini program would you like to run?")
            .items(&options)
            .interact()
            .unwrap();

        if options_selection == 0 {
            fahrenheit_celcius::run_cli();
            println!("");
        }
        else if options_selection == 1 {
            nth_fib::run_cli();
            println!("");
        }
        else if options_selection == 2 {
            twelve_days_of_xmas::run_cli();
            println!("");
        }
        else {
            println!("Thank you for checking this out!");
            println!("Goodbye!");
            break
        }
    }
}
