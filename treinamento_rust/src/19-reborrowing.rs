fn main() {
    let mut x: i32 = 4; // owner
    
    print_value(&x);

    println!("[Original] - Valor de x após as modificações: {} - referência: {:p}", x, &x);

    print_value_mutable(&mut x); // passa referência mutável

    println!("[Original] - Valor de x após as modificações: {} - referência: {:p}", x, &x);

    print_value_mutable(&mut x); // passa referência mutável

    println!("[Original] - Valor de x após as modificações: {} - referência: {:p}", x, &x);
}

fn print_value(value: &i32) {
    // value += 1; // não pode pois tenho imutabilidade nas referências
    println!("[Print] - O valor é {} - referencia: {:p}", value, value);
}

fn print_value_mutable(value: &mut i32) { // pede referência mutável
    *value += 1; // modificando o valor referenciado utilizando reborrowing.
    // move o valor da variável para uma variável temporária em uma localização diferente na memória e depois atualizar o valor de referência
    // para evitar possíveis problemas de aliasing e garantir segurança das referências mutáveis
    println!("[Reborrowing] - O valor da variável referênciada é {} - referência: {:p}", value, value);
}
