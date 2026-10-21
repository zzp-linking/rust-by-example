#![allow(dead_code)]
/**
 * 你可以使用 match 语句来解包 Option，但使用 ? 运算符通常更简便。
 * 如果 x 是一个 Option，那么求值 x? 将在 x 是 Some 时返回其内部值，
 * 否则它将终止当前执行的函数并返回 None。
 */

struct Person {
    job: Option<Job>,
}

#[derive(Clone, Copy)]
struct Job {
    phone_number: Option<PhoneNumber>,
}

#[derive(Clone, Copy)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: u32,
}

impl Person {

    // 如果存在，获取此人工作电话号码的区号。
    fn work_phone_area_code(&self) -> Option<u8> {
        // 如果没有 `?` 运算符，这将需要许多嵌套的 `match` 语句。
        // 这将需要更多的代码 - 试着自己写一下，看看哪种方式
        // 更容易。
        self.job?.phone_number?.area_code
    }
}

fn main() {
    let p = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(61),
                number: 439222222,
            }),
        }),
    };

    assert_eq!(p.work_phone_area_code(), Some(61));
}