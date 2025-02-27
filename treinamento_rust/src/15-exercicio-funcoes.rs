use std::io;

fn main() {
    menu();
}

fn menu() {
    loop {
        println!("Digite uma das opções abaixo:");
        println!(r#"
            1) Soma entre valores
            2) Subtração entre valores
            3) Criar a tabuada de um número
            0) Encerrar o programa
        "#);

        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Falha ao ler a linha");
        let option: i8 = option.trim().parse().expect("Por favor, digite um número");

        match option {
            1 => request_the_operation_between_values(true),
            2 => request_the_operation_between_values(false),
            3 => request_the_multiplication_table(),
            0 => {
                finish_the_program();
                break;
            }
            _ => println!("\nA opção que você escolheu é inválida\n")
        }
    }
}

fn request_the_operation_between_values(is_sum: bool) {
    let n1 = request_a_number(String::from("Digite o primeiro valor"));
    let n2 = request_a_number(String::from("Digite o segundo valor"));
    if is_sum {
        let result = sum(n1, n2);
        println!("A soma é: {}", result);
    }
    let result = subtraction(n1, n2);
    println!("A subtração é: {}", result);
}

fn request_the_multiplication_table() {
    let table_value = request_a_number(String::from("Digite o valor da tabuada"));
    multiplier_table(table_value);
}

fn finish_the_program() {
    println!("\nSaindo do programa\n")
}

fn request_a_number(question: String) -> i16 {
    println!("{}", question);
    let mut value = String::new();
    io::stdin().read_line(&mut value).expect("Falha ao ler a linha");
    let value: i16 = value.trim().parse().expect("Por favor, digite um número");
    value
}

fn sum(x: i16, y: i16) -> i16 {
    x + y
}

fn subtraction(x: i16, y: i16) -> i16 {
    x - y
}

fn multiplier_table(table_value: i16) {
    for multiplier in 1..=10 {
        println!("{} x {} = {}", table_value, multiplier, multiplier * table_value)
    }
}
