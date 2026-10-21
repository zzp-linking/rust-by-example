
/**
 * impl Trait 可以在两个位置使用：

 * 作为参数类型
 * 作为返回类型
 */
#[allow(dead_code)]
 fn parse_csv_document<R: std::io::BufRead>(src: R) -> std::io::Result<Vec<Vec<String>>> {
    src.lines()
        .map(|line| {
            // 遍历数据源中的每一行
            line.map(|line| {
                // 如果成功读取行，则处理它；否则，返回错误
                line.split(',') // 按逗号分割行
                    .map(|entry| String::from(entry.trim())) // 去除首尾空白
                    .collect() // 将该行的所有字符串收集到 Vec<String> 中
            })
        })
        .collect() // 将所有行收集到 Vec<Vec<String>> 中
}
/**
 * parse_csv_document 是泛型函数，
 * 可以接受任何实现了 BufRead 的类型，如 BufReader<File> 或 [u8]。
 * 但具体的 R 类型并不重要，R 仅用于声明 src 的类型。因此，这个函数也可以写成：
 */
#[allow(dead_code)]
fn parse_csv_document2(src: impl std::io::BufRead) -> std::io::Result<Vec<Vec<String>>> {
    src.lines()
        .map(|line| {
            // 遍历数据源中的每一行
            line.map(|line| {
                // 如果成功读取行，则处理它；否则，返回错误
                line.split(',') // 按逗号分割行
                    .map(|entry| String::from(entry.trim())) // 去除首尾空白
                    .collect() // 将该行的所有字符串收集到 Vec<String> 中
            })
        })
        .collect() // 将所有行收集到 Vec<Vec<String>> 中
}

/**
 * 如果函数返回一个实现了 MyTrait 的类型，
 * 你可以将其返回类型写为 -> impl MyTrait。这可以大大简化类型签名！
 */

use std::iter;
use std::vec::IntoIter;

// 这个函数合并两个 `Vec<i32>` 并返回一个迭代器。
// 看看它的返回类型有多复杂！
#[allow(dead_code)]
fn combine_vecs_explicit_return_type(
    v: Vec<i32>,
    u: Vec<i32>,
) -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
    v.into_iter().chain(u.into_iter()).cycle()
}

// 这是完全相同的函数，但它的返回类型使用了 `impl Trait`。
// 看看它变得多么简单！
fn combine_vecs(
    v: Vec<i32>,
    u: Vec<i32>,
) -> impl Iterator<Item=i32> {
    v.into_iter().chain(u.into_iter()).cycle()
}

fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5];
    let mut v3 = combine_vecs(v1, v2);
    assert_eq!(Some(1), v3.next());
    assert_eq!(Some(2), v3.next());
    assert_eq!(Some(3), v3.next());
    assert_eq!(Some(4), v3.next());
    assert_eq!(Some(5), v3.next());
    println!("全部完成");
}