// `F` 必须实现 `Fn` 用于一个不接受输入且不返回任何内容的闭包
// - 这正是 `print` 所需要的
fn apply<F>(f: F) where
    F: Fn() {
    f();
}

fn main() {
    let x = 7;

    // 将 `x` 捕获到一个匿名类型中并为其实现 `Fn`
    // 将其存储在 `print` 中
    let print = || println!("{}", x);

    apply(print);
}