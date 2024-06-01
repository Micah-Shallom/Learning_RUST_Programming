use crate::Colors::Red;

#[allow(unused_variables)]
#[allow(unused_assignments)]

mod structure;
mod enums;

fn main() {
    let primes:[i32;5] = [2, 3, 5, 7, 11];
    let doubles:[f64; 4] = [2.0, 4.0, 6.0, 8.0];
    println!("{:?}", primes);
    println!("{:?}", doubles);


    let numbers = [0; 15];
    println!("{:?}", numbers);

    const DEFAULT:i32 = 3;
    let mut numbers = [DEFAULT; 15];
    println!("{:?}", numbers);
    
    numbers[3] = 5;
    println!("{:?}", numbers);

    for number in numbers.iter(){
        println!("{}", number);
    }

    // Vectors
    // This are arrays of variable sizes
    let primes: Vec<i32> = Vec::new(); //either via standard method
    let mut primes = vec![2,3,5]; //or micro method
    println!("{:?}", primes);
    
    //adding elements (note the array has to be mutable)
    primes.push(7);
    println!("{:?}", primes);
    
    //removing an element(by index)
    primes.remove( 2);
    println!("{:?}", primes);
    
    let numbers = vec![2; 10];
    println!("{:?}", numbers);

    const DEFBOOL:bool = true;
    let values = vec![DEFBOOL; 5];
    println!("{:?}", primes);

    for number in numbers.iter() {
        println!("{:?}", number * number)
    }

    /*
        SLICES
        THIS IS A POINTER TO A BLOCK OF MEMORY
        can be used on arrays, vectors and strings
    */

    let numbers = [1,2,3,4,5];
    let slice = &numbers[1..4];
    println!("{:?}", slice);

    let mut colors = vec!["red", "green", "blue", "pink"];
    update_colors(&mut colors[2..4]);
    println!("{:?}", colors);

    /*
        TUPLES
        a collection of values of various types
        static- cannot be resized
        element values can be updated
        indexed
        liimited to 12 elements
     */

    let mut person = ("john", 27, true);
    println!("{:?}", person);

    //accessing elements
    println!("{}", person.0);
    
    //updating elements
    person.0 = "Shallom";
    println!("{}", person.0);

    //destructuring tuple
    let (name, age, employed) = person;
    println!("name={}, age={}, employed={}", name, age, employed);

    /*
        STRUCTURES
        a collection of key-valaue pairs
        synonymous to a class in python or structs in golang
    */
    structure::structure();

    
    /*
        GENERICS
        allows us to have variable data types

     */
    #[derive(Debug)]
    struct Point<T> {
        X: T,
        Y: T
    }

    let p1 = Point{X:6, Y:8};
    let p2 = Point{X:3.25, Y:8.63};
    println!("{:?}", p1);
    println!("{:?}", p2);

    
    let c1 = Red("#f000");
    let c2 = Red(255);

    let p3 = Point2{X:34, Y:3.0};

}
    


#[derive(Debug)]
enum Colors<T>{
    Red(T),
    Blue(T),
    Green(T)
}

//using multiple generics
#[derive(Debug)]
struct Point2<T, V> {
    X: T,
    Y: V
}


fn update_colors(color_slice: &mut [&str]) {
    color_slice[0] = "yellow";
    color_slice[1] = "orange";
}
