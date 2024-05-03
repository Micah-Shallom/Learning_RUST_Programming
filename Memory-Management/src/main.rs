/* 
    MEMORY-MANAGEMENT
        Ownership
        Borrowing
        Lifetimes
        Reference counted variables
*/

/* 
    OWNERSHIP
        Only one variable can own a piece of memory
        for primitive types, copying data is cheap
        for complex types, ownership is transferred
*/

/* 
    BORROWING
        only one variable can own a piece of memory
        variables can borrowownership to other pieces of memory
        the borrow has to match the mutability     
*/

/* 
    LIFETIME 
        An indication of how long an object will live
        Rust prevents parts of objects outliving the object
        Lifetimr ellision
*/

struct Person {
    name: String,
}
#[derive(Debug)]
struct Dog<'l>{
    name: String,
    owner: &'l Person
}

fn main() {
    let i = 5;
    let j = i;

    let v = vec![1,2,3,4,5];

    let foo = |v: Vec<i32>| -> Vec<i32> {
        println!("vector used in foo");
        v
    };

    let v = foo(v);
    println!("{:?}", v);

    //borrowing
    let mut a = 6;
    {
        let b = &mut a;
        println!("{}", *b);
        *b += 2;
    }
    println!("{}", a);

    let mut v = vec![1,2,3,4,5];

    for i in v {
        println!("{}", i);
        v.push(6);
    }      

    //lifetime
    let p1 = Person {name: String::from("John")};
    let d1 = Dog {name: String::from("Max"), owner: &p1};
    println!(
        "{:?}", d1
    );

}
