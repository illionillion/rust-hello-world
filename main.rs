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

    // letで再宣言は可能
    let age = 21;
    println!("He is {} years old.", age);

    if age >= 20 {
        println!("{}歳は成人です。", age);
    } else {
        println!("{}歳は未成年です。", age);
    }

    // ループ
    for i in 0..5 {
        println!("i= {}", i);
    }

    // 配列？
    let fruits = vec!["apple", "orange", "banana"];  
    // forで出力
    // イテレーター
    for fruit in fruits.iter() {  
        println!("{}", fruit);  
    }
    // 何が違う？
    for fruit in fruits {  
        println!("{}", fruit);  
    }  
}
