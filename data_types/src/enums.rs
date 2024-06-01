/*
    ENUMS
    a collection of values
*/
// use crate::Colors::Green; 

#[derive(Debug)]
pub enum Colors {
    Red,
    Green,
    Blue
}

#[derive(Debug)]
enum Person {
    Name(String),
    Surname(String),
    Age(u32)
}

fn main(){
    let my_color = Colors::Red;
    println!("{:?}", my_color);
    
    // let my_color = Green;

    let person = Person::Name(String::from("Shallom"));
    println!("{:?}", person)

}


