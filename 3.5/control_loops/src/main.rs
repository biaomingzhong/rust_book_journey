fn main() {
    let mut counter = 0;

    // loop
    let result = loop {
        counter += 1;

        println!("loop counter is {}", counter);

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);

    // while
    let mut number = 3;

    while number != 0 {
        println!("while number is {}", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");

    // for iter
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("for iter element value is: {}", element);
    }
    
    // for times
    for number in (1..4).rev() {
        println!("for iter number is: {}", number);
    }
    println!("LIFTOFF!!!");
}
