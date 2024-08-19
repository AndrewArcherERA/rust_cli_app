use dialoguer::Select;
use std::io;
use std::io::Write;

struct Rectangle {
    width: u32,
    height: u32,
}

pub fn run_cli() {
    loop {
        print!("Enter Width: ");
        let _ = io::stdout().flush();

        let mut width = String::new();
        io::stdin()
            .read_line(&mut width)
            .expect("Failed to read line");

        print!("Enter Height: ");
        let _ = io::stdout().flush();

        let mut height = String::new();
        io::stdin()
            .read_line(&mut height)
            .expect("Failed to read line");

        let width: u32 = match width.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number for width");
                continue;
            }
        };

        let height: u32 = match height.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number for height");
                continue;
            }
        };

        let rect1 = Rectangle {
            width: width,
            height: height,
        };

        println!("The area of the rectangle is: {}", area(&rect1));

        let options = vec!["Calculcate another area", "Quit"];
        let options_selection = Select::new()
            .with_prompt("What would you like to do?")
            .items(&options)
            .interact()
            .unwrap();

        if options_selection == 1 {
            break;
        }
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
