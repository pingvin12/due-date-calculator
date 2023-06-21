use std::{env, process};
use due_date_calculator::{calculator::Calculator, validator::{localdatetime::DatetimeValidator,workhours::WorkhoursValidator, turnaround::TurnaroundValidator}, misc::datetime_utils::DatetimeUtils};


fn main() {
    // Retrieve command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the required number of arguments is provided
    if args.len() != 3 {
        eprintln!("Error: wrong number of arguments specified");
        eprintln!("Usage: due-date-calculator <start_date> <turnaround_time>");
        process::exit(1);
    }

    // Parse command-line arguments
    let creation_date = match DatetimeValidator::validate(&args[1]) {
        Ok(date) => date,
        Err(error) => {
            eprintln!("Error: {}", error);
            process::exit(1);
        }
    };

    // since this validator does validation inside the method,
    //we don't need to check the format of creation_date since it is already a SystemTime variant.
    WorkhoursValidator::validation(creation_date);

    let turnaround_hours = match TurnaroundValidator::validate(args[2].parse().unwrap()) {
        Ok(_) => args[2].parse().unwrap(),
        Err(_) => {
            eprintln!("Error: invalid turnaround time specified");
            process::exit(1);
        }
    };

    // We create a new calculator object
    let mut calculator = match Calculator::new() {
        Ok(calculator) => calculator,
        Err(error) => {
            eprintln!("Error: {}", error);
            process::exit(1);
        }
    };
    // Perform the due date calculation
    let due_date = match calculator.calculate_due_date(creation_date, turnaround_hours) {
        Ok(date) => date,
        Err(error) => {
            eprintln!("Error: {}", error);
            process::exit(1);
        }
    };

    // Print the due date
    println!("{}", DatetimeUtils::format_system_time(due_date));

}
