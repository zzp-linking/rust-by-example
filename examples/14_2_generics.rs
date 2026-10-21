/**
 * 与函数类似，实现（impl）在涉及泛型时也需要谨慎处理。
 */

 struct Val {
    val: f64,
}

struct GenVal<T> {
    gen_val: T,
}

// Val 的实现
impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

// 为泛型类型 `T` 实现 GenVal
impl<T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}

fn main() {
    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3i32 };

    println!("{}, {}", x.value(), y.value());
}