/**
 * 在前面的例子中，我们使用组合器显式地处理了错误。
 * 处理这种情况分析的另一种方法是使用 match 语句和提前返回的组合。

 * 也就是说，如果发生错误，我们可以简单地停止执行函数并返回错误。
 * 对某些人来说，这种形式的代码可能更容易阅读和编写。
 * 考虑使用提前返回重写的前面示例的这个版本：
 */
use std::num::ParseIntError;

fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = match first_number_str.parse::<i32>() {
        Ok(first_number)  => first_number,
        Err(e) => return Err(e),
    };

    let second_number = match second_number_str.parse::<i32>() {
        Ok(second_number)  => second_number,
        Err(e) => return Err(e),
    };

    Ok(first_number * second_number)
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n)  => println!("n 是 {}", n),
        Err(e) => println!("错误：{}", e),
    }
}

fn main() {
    print(multiply("10", "2"));
    print(multiply("t", "2"));
}