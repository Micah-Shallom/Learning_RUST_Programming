#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(unused_macros)]

static mut R:i32 = 0;
macro_rules! build_fn {
    ($fn_name: ident) => {
        fn $fn_name(){
            println!("{} was called", stringify!($fn_name))
        }
    }
}

fn main() {
    {
        let a = 3;
        println!("{}", a);
    }

    unsafe {
        R = 4;
        println!("R = {}", R);
    }

    /*  
        CLOSURES 
        a function within a function
        an anonymous function, lambda expression
        a function can be assigned to a variable and be called with the name of the variable
        a closure can be generic
    */

    let a = |a:i32| a+1;
    println!("{}", a(6));

    let b = |b:i32| {
        let c:i32 = b + 1;
        c
    };
    println!("{}", b(12));

    //generic closures
    let _gn = |x: i32| println!("{}", x);

    /* 
        Higher Order Functions (HOF)
            functions that take another function as a parameter
    */

    let square = |a:i32| a * a;
    apply(square, 6);

    //calculate the sum of all even squares less than 500
    let limit = 500;
    let mut sum = 0;

    for i in 0.. {
        let isq = i  * i;
        if isq > limit {
            break
        }else {
            if is_even(isq) {
                sum += isq
            }
        }
    }
    println!("{}", sum);

    //using HOFs

    let _sum2 = 
        (0..).map(|x| x * x)
        .take_while(|&x| x <= limit)
        .filter(|x| is_even(*x))
        .fold(0, |sum, x| sum + x);

    /* 
        MACROS
            write code that writes code - meta programming
            match an expression and perform some opertaion
            code is expanded and compiled

            eg. println! and format!
            macro_rules! my_macro {
                (match) => (code to run)
            }
    */

    macro_rules! my_macro {
        () => {
            println!("First Macro")
        };
    }
    my_macro!();

    // macro_rules! name {
    //     ($name: expr) => {
    //         println!("{}", $name)
    //     };
    // }
    // name!("Shallom");

    macro_rules! names {
        ($($name:expr), *) => ( 
            $(println!("Hey {}", $name);)* 
        )
    }
    names!("Alex", "Mary", "Carol");

    macro_rules! xy {
        (x => $e:expr) => (
            println!("X is {}", $e);
        );

        (y => $e:expr) => (
            println!("Y is {}", $e);
        )
    }
    xy!(x => 5);
    xy!(y => 6*7);

    build_fn!(hey);
    hey();


}


fn is_even(x:i32) -> bool {
    x & 2 == 0
}


fn apply(f: fn(i32) -> i32, a:i32) {
    println!("{}", f(a))
}

