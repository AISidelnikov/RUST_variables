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

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("Значение y = {}", y);
    println!("Значение tup.x = {}, tup.y = {}, tup.z = {}", tup.0, tup.1, tup.2);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Значение массива a = {} {} {} {} {}", a[0], a[1], a[2], a[3], a[4]);
}

