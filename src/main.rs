fn main() {
    let mut age: isize = 0x0A10;
    //let s1: &str = "My";
    let sym: char = 'A';
    let b: bool = true;
    let num: f64 = 2.34;
    let name = "Rust";

    println!("{} age = {}!", name, age);

    for i in 1..10
    {
        age += i;
    }

    print!("{} age = {}! {} {} {}", name, age, num, b, sym);
}
