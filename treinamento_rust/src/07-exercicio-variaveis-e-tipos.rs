fn main() {
    /*
    Dado que eu tenha um ano de nascimento, e faço a subtração pelo ano atual,
    então devo ter o valor da idade da pessoa.
    */

    let name = "Marianne";

    let year_of_birth: u16 = 1993;
    let current_year: u16 = 2025;

    let age: u16 = current_year - year_of_birth;
    println!("A idade da pessoa ({}) calculada para o ano de {} é de {} anos", name, year_of_birth, age);

}
