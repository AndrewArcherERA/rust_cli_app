use dialoguer::Select;
use std::io;
use std::io::Write;

pub fn run_cli() {
    loop {
        let options = vec!["Convert Tempurature", "Quit"];
        let options_selection = Select::new()
            .with_prompt("What would you like to do?")
            .items(&options)
            .interact()
            .unwrap();

        if options[options_selection] == "Quit" {
            break
        } else {
            print!("Enter temp: ");
        let _ = io::stdout().flush();

        let mut temp = String::new();
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        let temp: i32 = temp.trim().parse().expect("Please type a number");

        let temp_scales = vec!["Fahrenheit", "Celsius"];
        let scale_selection = Select::new()
            .with_prompt("What would you like to convert your temp to?")
            .items(&temp_scales)
            .interact()
            .unwrap();

        convert(temp, temp_scales[scale_selection]);
        println!("");
        }
    }
}

fn convert(temp: i32, convert_to: &str) {
    if convert_to == "Celsius" {
        let c = (temp - 32) * 5 / 9;
        println!("{temp} degrees Fahrenheit is {c} degrees Celsius");
    } else {
        let f = (9 / 5 * temp) + 32;
        println!("{temp} degrees Celsius is {f} degrees Fahrenheit");
    }
}