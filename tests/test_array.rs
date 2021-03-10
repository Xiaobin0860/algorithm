fn find_missing_number(arr: &[i32]) -> i32 {
    let len = arr.len() as i32;
    for i in 0..len {
        if i != arr[i as usize] {
            return i;
        }
    }
    len
}

#[test]
fn test_missing_number() {
    let input: [i32; 3] = [0, 1, 3];
    assert_eq!(2, find_missing_number(&input));
    let input: [i32; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 9];
    assert_eq!(8, find_missing_number(&input));
    let input: [i32; 4] = [0, 1, 2, 3];
    assert_eq!(4, find_missing_number(&input));
}
