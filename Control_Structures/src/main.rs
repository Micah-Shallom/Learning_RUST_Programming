use rand::Rng;
use crate::Suit::{Club, Diamond, Heart, Spade};

fn main() {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0..11);

    if num >= 5 {
        println!("number {} is greater than or equal to 5", num);
    }else if num == 5{
        println!("{} == 5", num)
    }else{
        println!("Number {} is smaller than 5", num);
    }

    let res = if num >= 5 {true} else {false};
    println!("{}", res);


    /*
        MATCH
            similar to when or switch in other languages
            can return a result
            ranges are allowed
    */
    print_choice(Heart);
    print_choice(Club);
    print_choice(Spade);
    print_choice(Diamond);


    /* 
        PATTERN MATCHING
        multiple values 1 | 2
        Ranges 1..=5
        Conditions x if a > b
        tuple matching
    */
    for i in 0..15{
        println!("{}. I have {} oranges", i, get_oranges(i));
    }

    //tuple matching
    let point = (0,5);

    match point {
        (0, 0) => println!("origin"),
        (x, 0) => println!("x axis ({}, 0)", x),
        (0, y) => println!("y axis (0 , {})", y),
        (x, y) => println!("({} , {})", x,y),
    }

    /* 
        FOR LOOP
    */

    for i in 1..11{
        println!("{0} * {0} = {1}", i, i * i);
    }

    let pets = ["cat", "dog", "chihuahua", "hamster", "bear"];
    for pet in pets.iter(){
        if pet == &"chihuahua" {
            println!("{} barks too much", pet);
            continue
        }
        if pet == &"bear" {
            println!("{} is not a pet", pet);
            break
        }
        println!("I love my {}", pet);
    }

    for (pos, i) in (1..11).enumerate() {
           let square = i * i;
           let nb = pos + 1;
           println!("{0} * {0} = {1}", nb, square);
    }


}

enum Suit {
    Heart,
    Spade,
    Club,
    Diamond
}

fn country(code:i32) {
    let country = match code {
        44 => "UK",
        32 => "Spain",
        1..=9 => "unknown",
        _ => "invalid"
    };

    println!("Country is {}", country);
}

fn print_choice(choice: Suit) {
    match choice{
        Heart => {println!("\u{2665}")}
        Spade => {println!("\u{2660}")}
        Club => {println!("\u{2663}")}
        Diamond => {println!("\u{2666}")}
    }
}


fn get_oranges(amount: i32) -> &'static str {
    return match amount {
        0 => "no",
        1 | 2 => "one or two",
        3..=7 => "a few",
        _ if (amount % 2 == 0) => "an even amount of",
        _ => "lots of",
    }
}