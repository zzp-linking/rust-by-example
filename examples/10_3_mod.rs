/**
 * use 声明可以将完整路径绑定到新名称，以便更轻松地访问。
 */

 use std::mem::drop;

fn main() {
    let name = "Ferris".to_owned();
    println!("name: {}", name);
    drop(name)
}