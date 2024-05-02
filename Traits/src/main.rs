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

// use std::fmt::format;

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

//we can create a trait for a structure we didnt create

trait Summable<T> {
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32>{
    fn sum(&self) -> i32 {
        let mut sum = 0;
        for i in self {
            sum += *i
        }
        sum
    }
}

/* 
    Operator Overloading
        we can implement standard operators for our custom structs
        NOTE: operators themselves are traits
*/

use std::{fmt::format, ops::Add};

#[derive(Debug)]
struct Point{
    x: f64,
    y:f64
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Self) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

/* 
    STATIC DISPATCH
        a generic trait will be converted to the required type at compile time

*/

trait Duplicateable {
    fn dupl(&self) -> String;
}

impl Duplicateable for String   {
    fn dupl(&self) -> String {
        format!("{0}{0}", *self)
    }
}

impl Duplicateable for i32{
    fn dupl(&self) -> String {
        format!("{}",*self*2)
    }
}

fn duplicate<T: Duplicateable>(x: T){
    println!("{}", x.dupl());
}

/* 
    DYNAMIC DISPATCH
        A generic trait will be converted to the required type at run time.
*/

fn duplicate1(x: &dyn Duplicateable){
    println!("{}", x.dupl())
}

fn main() {
    let r = RustDev::new(true);
    let _j = RustDev::new(false);
    println!("{}", r.language());
    r.say_hello();

    let dog = Dog{species:"bingo"};
    let _cat = Cat{color: "white"};
    bark_it(dog);
    // bark_it(cat);

    println!("The animal says {}", get_animal(2.5).make_noise());
    println!("The animal says {}", get_animal(0.5).make_noise());

    let a = vec![1,2,3,4,5];
    println!("sum {}", a.sum());

    let p1 = Point{x:1.3, y:4.6};
    let p2 = Point{x:3.7, y:1.4};
    let p3 = p1 + p2;
    println!("{:?}", p3);

    let a = 42;
    let b = "Hi John ".to_string();
    duplicate(a);
    duplicate(b);
    duplicate1(&a);
    duplicate1(&b);
}
