/**
 * 在某些情况下，使用 match 匹配枚举可能会显得繁琐。
 * // 创建 `Option<i32>` 类型的 `optional`
let optional = Some(7);

match optional {
    Some(i) => println!("这是一个很长的字符串，其中包含 `{:?}`", i),
    _ => {},
    // ^ 这是必需的，因为 `match` 要求穷举所有情况。
    // 是不是觉得有些浪费空间？
};

 * 对于这种情况，if let 更加简洁，而且还允许指定各种失败时的处理选项
 */
fn main() {
    // 以下都是 `Option<i32>` 类型
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // `if let` 结构的含义是：如果 `let` 能将 `number` 解构为
    // `Some(i)`，则执行代码块（`{}`）。
    if let Some(i) = number {
        println!("匹配到 {:?}！", i);
    }

    // 如果需要指定匹配失败的情况，可以使用 else：
    if let Some(i) = letter {
        println!("匹配到 {:?}！", i);
    } else {
        // 解构失败。转到失败处理的情况。
        println!("没有匹配到数字。那就用一个字母吧！");
    }

    // 提供一个修改后的失败条件。
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("匹配到 {:?}！", i);
    // 解构失败。评估 `else if` 条件，看是否应该执行替代的失败分支：
    } else if i_like_letters {
        println!("没有匹配到数字。那就用一个字母吧！");
    } else {
        // 条件判断为假。这个分支是默认情况：
        println!("我不喜欢字母。那就用个表情符号吧 :)！");
    }
}