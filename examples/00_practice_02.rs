
fn main () {
    let mut arr = vec![42, 7, 19, 33, 5, 28, 14, 50, 1, 23];
    let len = arr.len();
    if len < 2 {
        print!("数组长度小于2，不需要排序");
    } else {
        let mut min_index;
        for i in 0..len-1 {
            min_index = i;
            for j in i+1..len {
                if arr[j] < arr[min_index] {
                    min_index = j
                }
            }
            arr.swap(i, min_index);
            
        }
        println!("{:?}", arr);
    }
    
}