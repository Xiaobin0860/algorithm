#[test]
fn test_missing_number() {
    let input = [0, 1, 3];
    assert_eq!(2, find_missing_number(&input));
    let input = [0, 1, 2, 3, 4, 5, 6, 7, 9];
    assert_eq!(8, find_missing_number(&input));
    let input = [0, 1, 2, 3];
    assert_eq!(4, find_missing_number(&input));
}

#[test]
fn test_count_number() {
    let input = [5, 7, 7, 8, 8, 10];
    assert_eq!(2, count_number(&input, 8));
    assert_eq!(0, count_number(&input, 6));
}

#[test]
fn test_max_area() {
    let input = [1, 8, 6, 2, 5, 4, 8, 3, 7];
    assert_eq!(49, max_area(&input));
    let input = [1, 1];
    assert_eq!(1, max_area(&input));
    let input = [4, 3, 2, 1, 4];
    assert_eq!(16, max_area(&input));
}

fn find_missing_number(arr: &[usize]) -> usize {
    let len = arr.len();
    for i in 0..len {
        if i != arr[i] {
            return i;
        }
    }
    len
}

fn count_number<T: PartialEq + Copy>(arr: &[T], target: T) -> usize {
    let mut count: usize = 0;
    for i in 0..arr.len() {
        if target == arr[i] {
            count += 1;
        }
    }
    count
}

use std::cmp;
fn max_area(arr: &[usize]) -> usize {
    let len = arr.len();
    if len < 2 {
        return 0;
    }
    let mut i = 0;
    let mut j = len - 1;
    let mut area = 0;
    loop {
        let w = j - i;
        if w <= 0 {
            break;
        }
        let h = cmp::min(arr[i], arr[j]);
        let tmp = w * h;
        if tmp > area {
            area = tmp
        }
        if arr[i] < arr[j] {
            i += 1;
        } else {
            j -= 1;
        }
    }
    area
}
