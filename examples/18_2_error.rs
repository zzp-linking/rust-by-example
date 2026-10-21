fn drink(beverage: &str) {
    // 你不应该喝太多含糖饮料。
    if beverage == "柠檬水" {
        // if cfg!(panic = "中止") {
        if true {
            println!("这不是你的派对。快跑！！！！");
        } else {
            println!("快吐出来！！！！");
        }
    } else {
        println!("来点清爽的{}就是我现在需要的。", beverage);
    }
}

fn main() {
    drink("水");
    drink("柠檬水");
}