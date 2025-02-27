fn main() {
    // memória stack - variáveis do tipo copy no Rust
    let x0: i32 = 4;
    let y0: i32 = x0; // cópia de dados

    println!("O valor de x0 é {} - Referência {:p}", x0, &x0);
    println!("O valor de y0 é {} - Referência {:p}", y0, &y0);

    // memória stack - variáveis do tipo copy no Rust
    let x1: i32 = 4; // owner
    let y1: &i32 = &x1; // referência de dados (y1 aponta para o mesmo local que x1, )

    println!("O valor de x1 é {} - Referência {:p}", x1, &x1);
    println!("O valor de y1 é {} - Referência {:p}", y1, y1); // y1 já é referência
}
