fn main() {
    // 引用规则

    // 1. 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
    // 2. 引用必须总是有效的。

    // immutable borrow
    let s = String::from("immutable val");

    let len = calculate_length(&s);

    println!("s is {} and len is {}", s, len);

    // mutable borrow
    let mut s2 = String::from("mutable val");

    change(&mut s2);

    println!("s2 is {}", s2);

    // let r1 = &mut s2;
    // 在特定作用域中的特定数据只能有一个可变引用
    // let r2 = &mut s2;
    // println!("{}, {}", r1, r2);

    // 不能在拥有不可变引用的同时拥有可变引用，然而，多个不可变引用是可以的
    // let r1 = &s2; // 没问题
    // let r2 = &s2; // 没问题
    // let r3 = &mut s2; // 大问题

    // println!("{}, {}, and {}", r1, r2, r3);

    // 以下可以，一个引用的作用域从声明的地方开始一直持续到最后一次使用为止
    let r1 = &s2; // 没问题
    let r2 = &s2; // 没问题
    println!("r1 {} and r2 {}", r1, r2);
    // 此位置之后 r1 和 r2 不再使用

    let r3 = &mut s2; // 没问题
    println!("r3 {}", r3);
}

fn calculate_length(input: &String) -> usize {
    input.len()
}

fn change(input: &mut String) {
    input.push_str(" change val");
}
