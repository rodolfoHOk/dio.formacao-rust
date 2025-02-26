// fn main() {
//     loop { // repetição
//         println!("again!");
//         // break; // sair o loop
//     }
// }

// fn main() {
//     let mut x: i16 = 1;

//     while x <= 20 {
//         // if x == 10 || x == 5 { 
//         if x >= 10 && x <= 15 { 
//             x += 1;
//             continue; 
//         }

//         println!("{x}");

//         // if x > 10 { break; }

//         x += 1;
//     }
// }

fn main() {
    for number in 1..4 { // até o 3
    // for number in 1..=4 { // até o 4
        println!("Número: {}", number);
    }
}
