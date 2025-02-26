const CONSTANT: i8 = 2;
// const mut CONSTANT: i8 = 2; // não permitido constante mutável
static STATIC_VAR: i8 = 3;
static mut STATIC_MUTABLE_VAR: i8 = 3;

fn main() {
    println!("Constante: {}", CONSTANT);
    imprime();

    const ANOTHER_CONSTANT: i8 = 4;
    println!("Outra constante: {}", ANOTHER_CONSTANT);

    println!("Variável estática: {}", STATIC_VAR);

    // STATIC_MUTABLE_VAR += 3; // use of mutable static is unsafe and requires an unsafe function or block // data race problem 
    unsafe {
        STATIC_MUTABLE_VAR += 3;
        // println!("Variável estática mutável: {}", STATIC_MUTABLE_VAR); // creating a shared reference to mutable static is discouraged
    }
}

fn imprime() {
    println!("Constante: {}", CONSTANT);
    // println!("Outra constante: {}", ANOTHER_CONSTANT); // não reconhece
}

