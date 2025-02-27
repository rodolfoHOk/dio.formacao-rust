fn main() {
    // variáveis na memória heap
    let s1 = String::from("Olá"); // não é uma variável by copy // s1 possui a propriedade da string
    println!("s1 valor: {} - referência: {:p}", s1, &s1);
    let s2 = s1; // a propriedade é transferida de s1 para s2 (Borrowing)

    // println!("s1 valor: {} - referência: {:p}", s1, &s1); // erro pois s1 não é mais válido após a transferência (morreu)
    println!("s2 valor: {} - referência: {:p}", s2, &s2); // s2 é válido

    let s3 = String::from("Olá"); // s3 possui a propriedade da string
    let s4 = s3.clone(); // s4 recebe um clone de s3 (faz uma cópia de verdade)

    println!("s3 valor: {} - referência: {:p}", s3, &s3);
    println!("s4 valor: {} - referência: {:p}", s4, &s4);
}
