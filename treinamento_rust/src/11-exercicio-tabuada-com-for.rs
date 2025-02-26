use std::io;

fn main() {
    println!("Digite o valor da tabuada: ");
    let mut table_value = String::new();
    io::stdin().read_line(&mut table_value).expect("Falha ao ler a linha");
    let table_value: i32 = table_value.trim().parse().expect("Por favor, digite um número");

    for multiplier in 1..=10 {
        println!("{} x {} = {}", table_value, multiplier, multiplier * table_value)
    }
}
