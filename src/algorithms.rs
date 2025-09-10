pub fn binary_search(array: &[usize], element: usize) {
    let mut left_pointer: usize = 0;
    let mut right_pointer: usize = (array.len() - 1) as usize;
    let mut result: i32 = -1;

    while left_pointer <= right_pointer {
        let mid: usize = (left_pointer + right_pointer) / 2;
        if array[mid as usize] < element {
            left_pointer = mid + 1;
        } else if array[mid as usize] > element {
            right_pointer = mid - 1;
        } else {
            result = mid as i32;
            break;
        }
    }
    println!("Index: {:?}", result);
}
