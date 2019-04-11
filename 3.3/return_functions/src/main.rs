fn main() {
    let x = five();
    println!("Hello, world!, x is: {}", x);
    let x = x+plus_one(x);
    println!("Hello, world!, x is: {}", x);
}

fn five() -> i32 {
    5
}

fn plus_one(i: i32) -> i32 {
    i+1
}

