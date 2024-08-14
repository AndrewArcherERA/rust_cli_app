use dialoguer::Select;
use std::io;
use std::io::Write;

pub fn run_cli() {
    loop {
        let options = vec!["All", "Single day", "Range of days"];
        let options_selection = Select::new()
            .with_prompt("What days would you like hear?")
            .items(&options)
            .interact()
            .unwrap();

        if options_selection == 0 {
            print_all();
        } else if options_selection == 1 {
            print_single_day();
        } else {
            print_range();
        }
        
        let options = vec!["See more days", "Quit"];
        let options_selection = Select::new()
            .with_prompt("What days would you to do?")
            .items(&options)
            .interact()
            .unwrap();

        if options_selection == 1 {
            break;
        }
    }
}

fn print_all() {
    let mut counter: usize = 0;

    println!("");

    while counter < 12 {
        handle_days(&counter);
        handle_presents(&counter);
        println!("");
        counter += 1;
    }
}

fn print_single_day() {
    print!("Enter day 1-12: ");
    let _ = io::stdout().flush();

    let mut counter = String::new();
    io::stdin()
        .read_line(&mut counter)
        .expect("Failed to read line");

    let counter: usize = counter.trim().parse().expect("Please type a number");
    let mut start = counter - 1;

    println!("");

    while start < counter {
        handle_days(&start);
        handle_presents(&start);
        println!("");
        start += 1;
    }
}

fn print_range() {
    print!("Enter start day 1-12: ");
    let _ = io::stdout().flush();

    let mut start = String::new();
    io::stdin()
        .read_line(&mut start)
        .expect("Failed to read line");

    let mut start: usize = start.trim().parse().expect("Please type a number");
    start -= 1;

    print!("Enter end day start-12: ");
    let _ = io::stdout().flush();

    let mut end = String::new();
    io::stdin()
        .read_line(&mut end)
        .expect("Failed to read line");

    let end: usize = end.trim().parse().expect("Please type a number");

    println!("");

    while start < end {
        handle_days(&start);
        handle_presents(&start);
        println!("");
        start += 1;
    }
}

fn handle_days(i: &usize) {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "nineth",
        "tenth", "eleventh", "twelfth",
    ];
    let day = days[*i];
    println!("On the {day} day of Christmas,");
    println!("my true love gave to me")
}

fn handle_presents(i: &usize) {
    let presents = [
        "Twelve drummers drumming,",
        "Eleven pipers piping,",
        "Ten lords a-leaping,",
        "Nine ladies dancing,",
        "Eight maids a-milking,",
        "Seven swans a-swimming,",
        "Six geese a-laying,",
        "Five golden rings,",
        "Four calling birds,",
        "Three French hens,",
        "Two turtle doves,",
        "And a partridge in a pear tree!",
    ];

    if *i == 0 {
        let first_day = "A partridge in a pear tree.";
        println!("{first_day}");
        return;
    }

    let mut index: usize = presents.len() - i - 1;

    while index < presents.len() {
        let present = presents[index];
        println!("{present}");

        index += 1;
    }
}
