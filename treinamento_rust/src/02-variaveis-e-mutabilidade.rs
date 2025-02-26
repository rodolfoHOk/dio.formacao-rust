fn main() {
    let x = 5; // infere o tipo da variável no caso i32
    // x = 6; // cannot mutate immutable variable `x`
    // x += 6; // cannot mutate immutable variable `x`
    println!("O valor de x é {}", x);

    let mut y = 5;
    y += 6;
    println!("O valor de y é {}", y);

    let z: i64 = 5; // define o tipo da variável no caso i64
    println!("O valor de z é {}", z);
}
