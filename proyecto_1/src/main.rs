mod interpolation;

fn main() {
    let b = 6;
    let sum = interpolation::bil_interpol(3, b);

    println!("9 = {}", sum);
    println!("6 = {}", b);
    println!("Hello, world!");
}
