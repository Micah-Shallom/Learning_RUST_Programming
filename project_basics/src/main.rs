use std::io;

/// Project Basics


fn main() { 
    //! # Main function

    //! fn main()
    //! 
    //! Reads user input and prints it to the console
    let mut input = String::new();

    // /Print a mesage to the user
    println!("Say something");
    /*
        Check and respond accordingly
     */
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("You said {}", input);
        },
        Err(e) => {
            println!("Something went wrong {}", e);
        }
    }

    //formatting
    println!("My name is {} and I am {} years old", "Alex", "29");

    //expressions
    println!("a + b = {}", 3+6);

    //positional arguments
    println!("{0} has a {2} and {0} has a {1}", "Alex", "Cat", "Dog");

    //named arguments
    println!("{name} {surname}", surname="Micah", name="Shallom"); 

    //printing traits
    println!("Binary {:b}, hex: {:x}, octal {:o}", 50, 50, 50);

    //Debug
    println!("array: {:?}", [1,2,3]);
}
