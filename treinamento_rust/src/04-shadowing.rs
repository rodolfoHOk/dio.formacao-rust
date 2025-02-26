fn main() {
    let x = 5;
    println!("O valor de x e sua memória: {}, {:p}", x, &x);    
    let x = x + 1;
    println!("O valor de x e sua memória: {}, {:p}", x, &x);    
    let x = x * 2;
    println!("O valor de x e sua memória: {}, {:p}", x, &x); 
}
