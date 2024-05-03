/* 
    ERROR HANDLING
        working with files
        errors
        helper methods
        ? operator
*/

use std::{fs::{File, OpenOptions}, io::{self, Read, Write}};

fn main() {
    // let mut file = File::create("src/example.txt").expect("create failed");
    //write to a file
    // file.write_all("Hello World!\n".as_bytes()).expect("failed");
    //append content to file
    // let mut file = OpenOptions::new().append(true).open("src/example.txt").expect("cannot open file");
    // file.write_all("adding content to the file.\n".as_bytes()).expect("failed");
    //read from a file
    let mut file = File::open("src/example.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents); 

    /* 
        ERRORS
            2 types of error
            Recoverable  -result enum
            Unrecoverable - panic macro
    */
    // panic!("Something happend! Cannot proceed");
    let f = File::open("main.jpg");

    //using enums
    match f {
        Ok(f) => {
            println!("file found {:?}", f);
        },
        Err(e) => {
            println!("file not found \n{:?}", e);
        }
    }
    println!("Continuing on with the execution");

    divide(Some(1));
    divide(Some(10));
    divide(None);
    divide(Some(0));

    /* 
        HELPER METHODS
        unwrap
            will return the data if its available or panic! if its not
        expect
            similar to unwrap but allows us to set a custom error message
    */
    let f = File::open("main.jpg").unwrap();
    let f= File::open("main.jpg").expect("Unable to open file");

    /* 
        ? Operator
    */
    let a = read_username_from_file();
    println!("{:?}", a);



}
const ANSWER_TO_LIFE:i32 = 42;

fn divide(x: Option<i32>){
    match x {
        Some(0) => panic!("Cannot divide by 0"),
        Some(x) => println!("result os {}", ANSWER_TO_LIFE / x),
        None => println!("None received, the answer is {}", ANSWER_TO_LIFE)
    }
}

fn read_username_from_file() -> Result<String, io::Error>{
    // let f = File::open("username.txt");
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // let mut s = String::new();
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }

    //the above code can be replaced with the below
    let mut f = File::open("src/username.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
