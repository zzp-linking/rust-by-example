/**
 * 运算符重载
 */
use std::ops;

struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar;

#[derive(Debug)]
struct BarFoo;

// `std::ops::Add` trait 用于指定 `+` 的功能
// 这里我们实现 `Add<Bar>` - 这个 trait 用于与 `Bar` 类型的右操作数相加
// 下面的代码块实现了操作：Foo + Bar = FooBar
impl ops::Add<Bar> for Foo {
    type Output = FooBar;

    fn add(self, _rhs: Bar) -> FooBar {
        println!("> 调用了 Foo.add(Bar)");

        FooBar
    }
}

// 通过交换类型顺序，我们实现了非交换加法。
// 这里我们实现 `Add<Foo>` trait —— 用于处理右操作数类型为 `Foo` 的加法。
// 此代码块实现了如下操作：Bar + Foo = BarFoo
impl ops::Add<Foo> for Bar {
    type Output = BarFoo;

    fn add(self, _rhs: Foo) -> BarFoo {
        println!("> 调用了 Bar.add(Foo)");

        BarFoo
    }
}

fn main() {
    println!("Foo + Bar = {:?}", Foo + Bar);
    println!("Bar + Foo = {:?}", Bar + Foo);
}