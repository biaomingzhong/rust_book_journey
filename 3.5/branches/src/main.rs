fn main() {
    let number = 6;

    if number > 5 {
        println!("number > 5 condition was true");
    } else {
        println!("number > 5 condition was false");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let number = if number % 3 == 0 {
        number/2
    } else {
        number
    };
    println!("new number is {}", number);
}
