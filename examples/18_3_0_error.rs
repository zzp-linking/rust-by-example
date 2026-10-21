/**
 * std 库中的 Option<T> 枚举用于处理可能存在缺失的情况。它表现为两个“选项“之一：

 * Some(T)：找到了一个 T 类型的元素
 * None：没有找到元素
 * 这些情况可以通过 match 显式处理，也可以用 unwrap 隐式处理。隐式处理要么返回内部元素，要么触发 panic。
 */

// 成年人见多识广，可以很好地处理任何饮料。
// 所有饮料都使用 `match` 显式处理。
fn give_adult(drink: Option<&str>) {
    // 为每种情况指定一个处理方案。
    match drink {
        Some("柠檬水") => println!("呸！太甜了。"),
        Some(inner)   => println!("{}？真不错。", inner),
        None          => println!("没有饮料？好吧。"),
    }
}

// 其他人在喝含糖饮料前会触发 `panic`。
// 所有饮料都使用 `unwrap` 隐式处理。
fn drink(drink: Option<&str>) {
    // 当 `unwrap` 收到 `None` 时会触发 `panic`。
    let inside = drink.unwrap();
    if inside == "柠檬水" { panic!("啊啊啊啊啊！！！！"); }

    println!("我超爱{}！！！！！", inside);
}

fn main() {
    let water  = Some("水");
    let lemonade = Some("柠檬水");
    let void  = None;

    give_adult(water);
    give_adult(lemonade);
    give_adult(void);

    let coffee = Some("咖啡");
    let nothing = None;

    drink(coffee);
    drink(nothing);
}