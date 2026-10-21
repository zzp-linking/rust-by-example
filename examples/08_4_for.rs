fn main() {
    // `n` 在每次迭代中将取值：1, 2, ..., 100(不含)
    for n in 1..100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

     // `n` 在每次迭代中将取值：1, 2, ..., 100(含)
     for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    /*
     * for in 结构能以多种方式与 Iterator 交互。
     * 正如在 Iterator 特质一节中讨论的那样，
     * 默认情况下 for 循环会对集合应用 into_iter 函数。
     * 然而，这并不是将集合转换为迭代器的唯一方法。
     * 
     * into_iter、iter 和 iter_mut 都以不同的方式处理集合到迭代器的转换，通过提供对数据的不同视图。
     * iter - 在每次迭代中借用集合的每个元素。因此，集合保持不变，并且在循环之后可以重复使用。
     * into_iter - 这会消耗集合，使得在每次迭代中提供确切的数据。一旦集合被消耗，它就不再可用于重复使用，因为它已经在循环中被“移动“了。
     * iter_mut - 这会可变地借用集合的每个元素，允许在原地修改集合。
     */
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("我们中间有一个 Rustacean！"),
            // TODO ^ 尝试删除 & 并只匹配 "Ferris"
            _ => println!("你好 {}", name),
        }
    }

    println!("names: {:?}", names);
}