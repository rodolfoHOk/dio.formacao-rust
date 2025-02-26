fn main() {
    let x: i8 = 100; // -128..=127
    println!("{}", x);

    let x: i16 = 30000; // -32768..=32767
    println!("{}", x);

    let x: i32 = 2000000000; // -2147483648..=2147483647
    println!("{}", x);

    let x: i64 = 9000000000000000000; // -9223372036854775808..=9223372036854775807
    println!("{}", x);

    let x: i128 = 100000000000000000000000000000000000000; // -170141183460469231731687303715884105728..=170141183460469231731687303715884105727
    println!("{}", x);

    let y: u8 = 200; // 0..=255
    println!("{}", y);

    let y: u16 = 60000; // 0..=65535
    println!("{}", y);

    let y: u32 = 4000000000; // 0..=4294967295
    println!("{}", y);

    let y: u64 = 10000000000000000000; // 0..=18446744073709551615
    println!("{}", y);

    let y: u128 = 300000000000000000000000000000000000000; // 0..=340282366920938463463374607431768211455
    println!("{}", y);

    let a: f32 = 1.0f32 / 5.0f32;
    println!("{}", a);

    let a: f64 = 1.0f64 / 5.0f64;
    println!("{}", a);

    let b: bool = false;
    println!("{}", b);

    let b: bool = true;
    println!("{}", b);

    let c: char = 'a';
    println!("{}", c);

    let s: &str = "rh"; // imutável somente
    println!("{}", s);

    let s: String = String::from("rh");
    println!("{}", s);

    let mut s: String = String::from("rh");
    s += "o";
    println!("{}", s);

    /*
    Outros tipos:
    - struct
    - enum
    - tuple
    - array
    - slice
    - String
    - Vec
    - Option
    - Result
    - HashMap
    */

}
