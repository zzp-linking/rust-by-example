/**
 * newtype 模式在编译时保证了程序接收到正确类型的值。
 */

struct Years(i64);

struct Days(i64);

impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}

impl Days {
    /// 截断不足一年的部分
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

fn is_adult(age: &Years) -> bool {
    age.0 >= 18
}

fn main() {
    let age = Years(25);
    let age_days = age.to_days();
    println!("是否成年？{}", is_adult(&age));
    println!("是否成年？{}", is_adult(&age_days.to_years()));
    // println!("是否成年？{}", is_adult(&age_days));
}