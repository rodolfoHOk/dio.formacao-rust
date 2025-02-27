fn main() {
    show_on_screen(1);

    let r = show_on_screen_with_return(1);
    println!("O valor somado é: {}", r);
}

fn show_on_screen(i: i32) {
    println!("O valor de i é: {}", i);
    if i >= 10 {
        return
    }
    show_on_screen(i + 1)
}

fn show_on_screen_with_return(i: i32) -> i32 {
    if i >= 10 {
        return i
    }
    println!("O valor de i é: {}", i);
    show_on_screen_with_return(i + 1)
}
