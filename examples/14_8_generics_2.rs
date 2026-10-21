
/**
 * 让我们使用关联类型重写上一节的示例
 */
struct Container(i32, i32);

// 一个检查容器内是否存储了两个项的 trait。
// 同时可以检索第一个或最后一个值。
trait Contains {
    // 在此定义泛型类型，方法将能够使用这些类型
    type A;
    type B;

    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains for Container {
    // 指定 `A` 和 `B` 的具体类型。如果 `input` 类型
    // 是 `Container(i32, i32)`，那么 `output` 类型
    // 就被确定为 `i32` 和 `i32`
    type A = i32;
    type B = i32;

    // 在这里使用 `&Self::A` 和 `&Self::B` 也是有效的。
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }
    // 获取第一个数字
    fn first(&self) -> i32 { self.0 }

    // 获取最后一个数字
    fn last(&self) -> i32 { self.1 }
}

fn difference<C: Contains>(container: &C) -> i32 {
    container.last() - container.first()
}

fn main() {
    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!("容器是否包含 {} 和 {}：{}",
        &number_1, &number_2,
        container.contains(&number_1, &number_2));
    println!("第一个数字：{}", container.first());
    println!("最后一个数字：{}", container.last());

    println!("差值为：{}", difference(&container));
}