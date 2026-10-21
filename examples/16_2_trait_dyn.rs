/**
 * 智能指针包含的特征对象 Box<dyn Animal>
 */

struct Sheep {}
struct Cow {}

trait Animal {
    // 实例方法签名
    fn noise(&self) -> &'static str;
}

// 为 `Sheep` 实现 `Animal` trait
impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "咩～！"
    }
}

// 为 `Cow` 实现 `Animal` trait
impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "哞~哞~哞~"
    }
}

// 返回某个实现了 Animal 的结构体，但在编译时我们并不知道具体是哪一个
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

fn main() {
    let random_number = 0.234;
    let animal = random_animal(random_number);
    println!("你随机选择了一个动物，它说：{}", animal.noise());
}
