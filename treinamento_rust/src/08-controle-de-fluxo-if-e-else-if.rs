fn main() {
    /*
    Dado que eu tenha um ano de nascimento, e faço a subtração pelo ano atual,
    então devo ter o valor da idade da pessoa.
    */

    let name = "Rodolfo";

    let year_of_birth: u16 = 1980;
    let month_of_birth: u16 = 6;
    let day_of_birth: u16 = 18;

    let current_year: u16 = 2025;
    let current_month: u16 = 2;
    let current_day: u16 = 25;

    let mut age: u16 = current_year - year_of_birth;
    if month_of_birth > current_month {
        age -= 1;
    }
    else if month_of_birth == current_month && day_of_birth > current_day {
        age -= 1;
    }
    
    println!("A idade da pessoa ({}) calculada para o ano de {} é de {} anos", name, year_of_birth, age);
}
