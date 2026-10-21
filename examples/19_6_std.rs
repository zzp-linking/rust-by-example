/**
 * panic! 宏可用于生成一个 panic 并开始展开其栈。
 * 在展开过程中，运行时会通过调用该线程所有对象的析构函数来释放线程拥有的所有资源。

 * 由于我们处理的是只有一个线程的程序，panic! 会导致程序报告 panic 消息并退出。
 */
// 重新实现整数除法（/）
fn division(dividend: i32, divisor: i32) -> i32 {
    if divisor == 0 {
        // 除以零会触发 panic
        panic!("除以零");
    } else {
        dividend / divisor
    }
}

// `main` 任务
fn main() {
    // 堆分配的整数
    let _x = Box::new(0i32);

    // 这个操作将触发任务失败
    division(3, 0);

    println!("这个点不会被执行到！");

    // `_x` 应该在此处被销毁
}