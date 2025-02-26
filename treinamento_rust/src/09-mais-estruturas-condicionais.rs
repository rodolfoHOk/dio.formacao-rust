/*
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
        return; // termina a função
    } else {
        println!("condition was false");
    }

    println!("Oi");
}
*/

// fn main() {
//     let number = 3;

//     if number { // error: expected `bool`
//         println!("number was three");
//     }
// }

// fn main() {
//     let number = 3;

//     if number != 0 {
//         println!("number was something other than zero");
//     }
// }

// fn main() {
//     let number = 6;

//     if number % 4 == 0 {
//         println!("number is divisible by 4");
//     } else if number % 3 == 0 {
//         println!("number is divisible by 3");
//     } else if number % 2 == 0 {
//         println!("number is divisible by 2");
//     } else {
//         println!("number is not divisible by 4, 3, or 2");
//     }
// }

// fn main() {
//     let number: i8 = 4;

//     let result: i8 = if number > 3 { 10 } else { 5 };

//     println!("The value of number is: {result}");
// }

fn main() {
    let number: i8 = 3;

    match number {
        1 => println!("um"),
        2 => println!("dois"),
        3 => println!("três"),
        _ => println!("outro número"),
    }
}
