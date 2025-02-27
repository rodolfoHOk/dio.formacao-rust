use std::io;

fn main() {
    loop {
        println!("Digite uma das opções abaixo:");
        println!(r#"
            Opção 1
            Opção 2
            Opção 3
            Opção 4 - Sair
        "#);

        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Falha ao ler a linha");
        let option: i8 = option.trim().parse().expect("Por favor, digite um número");

        match option {
            1 => println!("\nVocê escolheu a opção um\n"),
            2 => println!("\nVocê escolheu a opção dois\n"),
            3 => println!("\nVocê escolheu a opção três\n"),
            4 => {
                println!("\nSaindo do programa\n");
                break;
            }
            _ => println!("\nA opção que você escolheu é inválida\n")
        }
    }
}
