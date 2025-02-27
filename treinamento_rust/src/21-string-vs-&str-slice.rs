fn main() {
    // variáveis na memória heap
    let s1 = String::from("Olá, mundo"); // não é uma variável by copy // s1 possui a propriedade da string
    let s2 = s1; // a propriedade é transferida de s1 para s2 (Borrowing)

    // print_string(&s1); // erro pois s1 não é mais válido após a transferência (morreu)
    print_string(&s2);

    let s3 = String::from("Olá, mundo"); // s3 possui a propriedade da string
    let s4 = s3.clone(); // clonando a string de s3 para a s4
    print_string(&s3);
    print_string(&s4);

    let s5 = "Olá, mundo"; // s5 é um &str (slice de string) e é sempre imutável
    let s6 = s5; // s6 é uma referência para o mesmo &str de s5

    print_str(&s5);
    print_str(&s6);

    let mut s7 = String::from("Olá, mundo");
    s7 += " - teste";
    let s8 = s7.clone();
    print_string(&s7);
    print_string(&s8);

    let mut s9 = "Olá, mundo";
    // s9 += " - teste"; // não é permitido em &str (slice)
    let s10 = s9;

    print_str(s9);
    print_str(s10);
}

fn print_string(s: &String) {
    println!("Valor da string: {} - referência: {:p}", s, s);
}

fn print_str(s: &str) {
    println!("Valor da &str: {} - referência: {:p}", s, s);
}
