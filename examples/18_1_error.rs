/**
 * 我们将看到的最简单的错误处理机制是 panic。
 * 它会打印一条错误消息，开始展开栈，并通常会退出程序。
 */
fn drink(beverage: &str) {
    // 你不应该喝太多含糖饮料。
    if beverage == "柠檬水" { panic!("啊啊啊啊啊！！！！"); }

    println!("来点清爽的{}就是我现在需要的。", beverage);
}

fn main() {
    drink("水");
    drink("柠檬水");
    drink("纯净水");
}