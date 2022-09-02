const MAX_POINT: u32 = 100_000;

fn main() {
    let x = 5;
    println!("Значение x равно {}", x);
    let x = 6;
    println!("Значение x равно {}", x);
    let x = x * 2;
    println!("Значение x равно {}", x);

    let space = "   ";
    let space = space.len();
    println!("Длина строки space = {}", space);

    let guess: u32 = "42".parse().expect("НЕ являетя числом");

    let x = 2.0;
    let y: f32 = 3.0;

    let t = true;
    let f: bool = false;
    println!("Згачение f = {}", f);

    let z = 'z';
    let c = 'c';
    println!("z = {}", z);
}
