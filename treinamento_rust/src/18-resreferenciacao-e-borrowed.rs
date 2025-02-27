fn main() {
    let x: i32 = 4; // owner
    let y: &i32 = &x; // y é uma referência para x

    println!("O valor de x é {}", x);
    println!("O valor de y é {}", y);
    println!("O endereço de memória de x é {:p}", &x);
    println!("O endereço de memória de y é {:p}", y);

    let t: &i32 = y; // cria outra referência para o owner
    println!("O endereço de memória de t é {:p}", t);

    let w: i32 = *y; // desreferência com copy para o w
    println!("O endereço de memória de w é {:p}", &w);

    println!("O valor de t é {}", t);
    println!("O valor de w é {}", w);

    let mut x1: i32 = 4;
    let y1: &i32 = &x1;

    println!("O valor de x1 é {}", x1);
    println!("O valor de y1 é {}", y1);

    x1 = 44; // cannot assign to `x1` because it is borrowed - quando tento imprimir o y1 novamente
    println!("O valor de x1 é {}", x1);
    // println!("O valor de y1 é {}", y1); // não posso mais usar a referência se mudar o valor de owner

    print_value(&x);
    print_value(y);
}

fn print_value(value: &i32) {
    println!("O valor é {}", value);
    println!("O endereço de memória é {:p}", value);
}
