use dialoguer::Select;
mod functions;
fn main() {
    println!("");
    loop {
        let options = vec![
            "Convert tempeture to Fahrenheit or Celcius",
            "Get the nth number in the Fibonocci Sequence",
            "Print The Twelve Days Of Christmas Lyrics",
            "Calculate the area of a rectangle",
            "Quit"
        ];
        let options_selection = Select::new()
            .with_prompt("What mini program would you like to run?")
            .items(&options)
            .interact()
            .unwrap();

        if options_selection == 0 {
            functions::fahrenheit_celcius_cli();
            println!("");
        }
        else if options_selection == 1 {
            functions::nth_fib_cli();
            println!("");
        }
        else if options_selection == 2 {
            functions::twelve_days_of_xmas_cli();
            println!("");
        }
        else if options_selection == 3 {
            functions::get_area_cli();
            println!("");
        }
        else {
            println!("Thank you for checking this out!");
            println!("Goodbye!");
            break
        }
    }
}
