/**
 * match 分支可以使用守卫进行额外的筛选。
 */

 #[allow(dead_code)]
enum Temperature {
    Celsius(i32),
    Fahrenheit(i32),
}

fn main() {
    let temperature = Temperature::Celsius(35);
    // ^ TODO：尝试为 `temperature` 赋予不同的值

    match temperature {
        Temperature::Celsius(t) if t > 30 => println!("{}°C 高于 30°C", t),
        // `if condition` 部分 ^ 就是守卫
        Temperature::Celsius(t) => println!("{}°C 不高于 30°C", t),

        Temperature::Fahrenheit(t) if t > 86 => println!("{}°F 高于 86°F", t),
        Temperature::Fahrenheit(t) => println!("{}°F 不高于 86°F", t),
    }
}