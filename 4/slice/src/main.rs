fn main() {
    // 返回在该字符串中找到的第一个单词下标位置
    let words = String::from("hello world");
    let first_word_end = first_word_end(&words);

    println!("find first_word length is {}", first_word_end);

    let my_string = String::from("hello world");
    // first_word 中传入 `String` 的 slice
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word 中传入字符串字面值的 slice
    let word1 = first_word(&my_string_literal[..]);

    // 因为字符串字面值 **就是** 字符串 slice，
    // 这样写也可以，即不使用 slice 语法！
    let word2 = first_word(my_string_literal);

    println!("find first_word 0 {} 1 {} 2 {}", word, word1, word2);

    // 其他类型的 slice
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];
}

fn first_word_end(input: &String) -> usize {
    let bytes = input.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    input.len()
}

fn first_word(input: &str) -> &str {
    let bytes = input.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &input[0..i];
        }
    }

    &input[..]
}
