fn main() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", tup.2);
    println!("The value of x is: {}", tup.0);
}
