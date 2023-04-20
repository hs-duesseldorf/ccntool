// Import the `InputError` type from the `inputtypes` module.
mod inputtypes;
use inputtypes::InputError;

// Import the `connectdb` and `myquery` functions from the `ccntool_core` crate.
use ccntool_core::{connectdb, myquery};

// Import the `io` module from the Rust standard library.
use std::io::{self, Write};

// Define a function for getting user input.
fn userinput() -> std::result::Result<String, InputError> {
    let mut notes = String::new();
    print!("Enter wallsocket description: ");
    io::stdout().flush().unwrap();

    // Read a line of input from the user.
    io::stdin().read_line(&mut notes).unwrap();

    // Trim whitespace from the end of the input string.
    notes = notes.trim_end().to_string();

    // If the input is empty, return an error.
    if notes.is_empty() {
        Err(InputError::new("Some error occured:"))
    } else {
        // Otherwise, return the input as a `String`.
        Ok(notes)
    }
}

// Define the main function.
fn main() {
    // Print a welcome message.
    println!("Welcome to the TDQU-cli:");

    // Connect to the database.
    let conn = connectdb(None, None, None).expect("Can't connect to database");

    // Get user input.
    let notes = userinput();

    // Match on the user input.
    match notes {
        // If the input is valid, run a query on the database.
        Ok(n) => {
            let results = match myquery(conn, &n) {
                // If the query is successful, store the results.
                Ok(rows) => rows,
                // If there's an error, print it and exit.
                Err(error) => {
                    eprintln!("Error: {error}");
                    return;
                }
            };

            // Print out the results of the query.
            println!("\n\t\tHere is what I know:\n");
            println!("Switchname: {}", results[0]);
            println!("IP: {}", results[3]);
            println!("Switchport: {}", results[2]);
            println!("Description: {}", results[1]);
            let baseurl = dotenvy::var("DCIMHOST").unwrap();
            let url: String = format!("https://{}/devices.php?DeviceID={}", baseurl, results[4]);
            println!("{url}");
        }
        // If there's an error with the user input, print it and exit.
        Err(e) => {
            eprintln!("Something went wrong: {e}");
        }
    }
}
