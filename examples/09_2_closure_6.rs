fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // 对 vec 使用 `iter()` 产生 `&i32`，解构为 `i32`
    println!("2 在 vec1 中：{}", vec1.iter()     .any(|&x| x == 2));
    // 对 vec 使用 `into_iter()` 产生 `i32`，无需解构
    println!("2 在 vec2 中：{}", vec2.into_iter().any(|x| x == 2));

    // `iter()` 只借用 `vec1` 及其元素，所以它们可以再次使用
    println!("vec1 长度：{}", vec1.len());
    println!("vec1 的第一个元素是：{}", vec1[0]);
    // `into_iter()` 会移动 `vec2` 及其元素，所以它们不能再次使用
    // println!("vec2 的第一个元素是：{}", vec2[0]);
    // println!("vec2 长度：{}", vec2.len());
    // TODO：取消上面两行的注释，观察编译器错误

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // 对数组使用 `iter()` 产生 `&i32`
    println!("2 在 array1 中：{}", array1.iter()     .any(|&x| x == 2));
    // 对数组使用 `into_iter()` 产生 `i32`
    println!("2 在 array2 中：{}", array2.into_iter().any(|x| x == 2));
}