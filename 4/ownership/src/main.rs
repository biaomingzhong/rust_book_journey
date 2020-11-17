fn main() {
    // 变量与数据的交互，克隆
    let mut s1 = String::from("hello");
    s1.push_str(", world!");
    let s2 = s1.clone();
    println!("s1 is: {}", s1);
    println!("s2 is: {}", s2);

    // 只在栈上的数据：拷贝，Copy trait
    let x = 5;
    let y = x;
    println!("x is {} y is {}", x, y);

    // 所有权与函数
    ownership_with_fn();

    // 所有权与返回值
    ownership_with_fn_return();

    // 元组来返回多个值
    ownership_with_fn_return_tup();
}

fn ownership_with_fn() {
    let s = String::from("hello owner");
    take_ownership(s);

    // 这里不能再使用 s，s 的值已被移动到函数里...
    // println!("use s after function {} ", s);

    let x = 4;
    makes_copy(x);

    // 这里 x 还能继续使用，x 的值是 i32 是 Copy 的，所以在后面可继续使用 x
    println!("use x after function {} ", x);
}// 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，所以不会有特殊操作

fn take_ownership(input: String) {
    println!("take_ownership input {}", input);
}

fn makes_copy(input: i32) {
    println!("makes_copy input {}", input);
}

fn ownership_with_fn_return() {
    // gives_ownership 将返回值移给 s1
    let s1 = given_ownership();

    println!("s1 is {}", s1);

    let s2 = String::from("s2 val");
    // s2 被移动到 takes_and_gives_back 中,
    // 它也将返回值移给 s3
    let s3 = takes_and_gives_back(s2);

    println!("s3 is {}", s3);
} // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，所以什么也不会发生。s1 移出作用域并被丢弃

fn given_ownership() -> String {
    let res = String::from("return");
    res
}

fn takes_and_gives_back(input: String) -> String {
    input
}

fn ownership_with_fn_return_tup() {
    let s1 = String::from("hello tup");

    let (s2, length) = calculate_length(s1);

    println!("s2 is {} length is {}", s2, length);
}

fn calculate_length(input: String) -> (String, usize) {
    let length = input.len();
    (input, length)
}