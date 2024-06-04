#[allow(unused_variables)]
#[allow(unused_assignments)]

fn main() {
    
    let name:&str = "Shallom";
    let mut age:i32 = 32;

    age = 23;

    let color = "blue";
    let color = 86;

    println!("{}", color);

    let (a, b , c) = (4, 85, "red");

    let pi: f32 = 4.0;
    let million = 1_000_000;

    let is_day = true;
    let is_night = false;

    let char1 = 'A';

    let cat = "Fluffy";
    let cat: &'static str = "Fluffy";

    
    //string objects
    let dog = String::new();
    let mut dog = String::from("Max");

    let owner = format!("Hi I'm {} the owner of {}", "Mark", dog);

    println!("{}", dog.len());

    dog.push(' ');
    dog.push_str("the dog");
    println!("{}",dog);

    const URL: &str = "google.com";
    println!("{} URL", URL);

    let a = 2 + 3;
    let b = 10 / 2;
    let c = 10 % 5 ;
    println!("a={}, b={}, c={}", a,b, c);
    println!("{}", a >= b && c <= b);

    // say_hi()
    say_alotof_hi();

    let mut name = "Shallom";
    say_hello(name);
    say_hello1(&mut name);

}

fn say_hi() {
    println!("Hello There");
}

fn say_alotof_hi(){
    for i in 1..6{
        say_hi();
    }
}

fn say_hello(name: &str){
    println!("Hello {}", name);
}

//passing by reference with return values
fn say_hello1(name: &mut &str) -> String{
    println!("Hello {}", name);
    let greeting = format!("Hello {}", name);

    *name = "Micah ";
    print!("Hello {}", name);

    return greeting;
}