fn main() {
    // 世界よ、こんにちは
    println!("Hello, world!!");

    // 変数
    let age = 22;
    println!("I am {} years old.", age);
    // 再代入はできない
    // age = 21

    // 可変変数で再代入可能
    let mut name = "John";
    println!("I am {}!", name);
    name = "Jane";
    println!("She is {}!", name);
}
