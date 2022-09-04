mod interpolation;

fn main() {
    let b = 6;
    let mut buf: [u8; 3] = [1, 2, 3];
    let sum = interpolation::bil_interpol("result.img", &mut buf, 1, 3);

    println!("9 = {}", sum);
    println!("6 = {}", b);
    println!("Hello, world!");
}
