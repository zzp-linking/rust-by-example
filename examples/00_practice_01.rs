/**
 * 冒泡排序
 */
fn main () {
    let mut arr = vec![42, 7, 19, 33, 5, 28, 14, 50, 1, 23];
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - i -1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
    println!("{:?}", arr);
}