/* 
    Traits
        are very similar to an interface or an abstract class
        add a definition to a structure

    Generics
    dyn
    Operator Overlaoding
    Static dispatch
    dynamic dispatch
*/

use std::fmt::format;

struct RustDev {
    awesome: bool
}

struct PythonDev {
    awesome: bool
}

//implmenting traits

trait Developer {
    fn new(awesome: bool) -> Self;
    fn language(&self) -> &str;
    fn say_hello(&self) {println!("Hello World!")} 
}


impl Developer for RustDev {
    fn new(awesome: bool) -> Self {
        return RustDev{awesome: awesome};
    }

    fn language(&self) -> &str {
        return "Rust";
    }

    fn say_hello(&self) {
        return println!("println!(\"Hello World\")")
    }
}

impl Developer for PythonDev {
    fn language(&self) -> &str {
        return "Python";
    }
    fn new(awesome: bool) -> Self {
        return PythonDev{awesome: awesome};
    }
    fn say_hello(&self) {
        return println!("print(\"Hello world\")")
    }
}

//=========================================================
/* 
    trait generics
*/

trait Bark {
    fn bark(&self) -> String;
}

struct Dog {
    species : &'static str
}

struct Cat {
    color: &'static str
}

impl Bark for Dog {
    fn bark(&self) -> String {
        return format!("{} barking", self.species)
    }
}

//takes in all struct objects 
fn bark_it<T: Bark>(b: T){
    println!("{}", b.bark())
}

//================================================
/*
    RETURNING TRAITS
        the compiler needs to know the space required for a function return type
        a workaround is to return a box with a dyn traits
*/

trait Animal {
    fn make_noise(&self) -> &'static str;
}

struct Cow {}
struct Goat {}

impl Animal for Cow{
    fn make_noise(&self) -> &'static str {
        "moo"
    }
}

impl Animal for Goat{
    fn make_noise(&self) -> &'static str {
        "mehhhhhh"
    }
}

fn get_animal(val: f64) -> Box<dyn Animal> {
    if val < 1.0 {
        Box::new(Cow{})
    }else {
        Box::new(Goat{})
    }
}


fn main() {
    let r = RustDev::new(true);
    let j = RustDev::new(false);
    println!("{}", r.language());
    r.say_hello();

    let dog = Dog{species:"bingo"};
    let cat = Cat{color: "white"};
    bark_it(dog);
    // bark_it(cat);

    println!("The animal says {}", get_animal(2.5).make_noise());
    println!("The animal says {}", get_animal(0.5).make_noise());
}
