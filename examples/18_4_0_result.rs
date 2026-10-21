/**
 * Result 是 Option 类型的增强版，它描述可能的错误而非可能的缺失。

 * 也就是说，Result<T, E> 可能有两种结果之一：

 * Ok(T)：找到了一个 T 类型的元素
 * Err(E)：发生了一个 E 类型的错误
 */
use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let number_str = "10";
    let number = match number_str.parse::<i32>() {
        Ok(number)  => number,
        Err(e) => return Err(e),
    };
    println!("{}", number);
    Ok(())
}