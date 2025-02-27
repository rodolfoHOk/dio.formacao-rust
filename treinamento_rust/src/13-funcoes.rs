fn main() {
    println!("Hello, world!");

    another_function();
    another_function();
    another_function();
    another_function();

    sum_without_return(1, 2);

    let r = sum(2, 3);
    println!("O valor de r: {}", r);

    let n1 = 99;
    let n2 = 99;
    let r = sum(n1, n2);
    println!("O valor de r: {}", r);

    let r = sum_ref(&n1, &n2);
    println!("O valor de r: {}", r);

    unnamed_function_example();

    let _x = another_function(); // no Rust não retorna void, retorna um tipo de unidade

    let s = string_return();
    println!("O valor de s: {}", s);

    let result = function_with_return(10);
    println!("{}", result);

    function_without_return(10);
    function_without_return(11);
}

fn another_function() {
    println!("Another function.");
}

fn sum_without_return(x: i8, y: i8) {
    let r = x + y;
    println!("x + y = {}", r);
}

fn sum(x: i16, y: i16) -> i16 {
    x + y
}

fn sum_ref(x: &i16, y: &i16) -> i16 {
    x + y
}

fn unnamed_function_example() {
    let y = { // unnamed function
        let x = 3;
        x + 1 // retorna a última linha
    };

    println!("O valor de y é: {}", y);
}

fn string_return() -> String {
    String::from("Teste")
}

fn function_with_return(param: i32) -> String {
    if param > 10 {
        return String::from("Maior que dez");
    }
    String::from("Menor ou igual a dez")
}

fn function_without_return(param: i32) {
    if param > 10 {
        return;
    }
    println!("Menor ou igual a dez")
}
