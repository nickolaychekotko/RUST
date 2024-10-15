#![warn(clippy::all, clippy::pedantic)]
use std::cmp::Ordering;

fn main() {
    let arr = [-1, 2, 4, 42, 77];
    let result = bin_search(&arr, 23);
    match result {
        Some((found_value, found_index)) => println!("{}", found_value),
        None => println!("NONE"),
    }
}

fn bin_search(arr: &[i32], desired_value: i32) -> Option<(i32, usize)> {
    let mut low_bound = 0;
    let mut up_bound = arr.len() - 1;
    let mut i = 0;

    while low_bound <= up_bound {
        i += 1;

        let mid = (up_bound + low_bound) / 2;
        let mid_value = arr[mid];
        /*if mid_value == desired_value {
            return Some((mid_value, mid));
        } else if desired_value > mid_value {
            low_bound = mid + 1;
        } else {
            up_bound = mid - 1;
        }*/

        match mid_value.cmp(&desired_value){
            Ordering::Equal=> return Some((mid_value,mid)),
            Ordering::Greater=>up_bound = mid - 1,
            Ordering::Less=>low_bound = mid + 1,
        }
        println!("step {i}")
    }
    None
}

#[cfg(test)]

mod tests {
    use super::*;
    const ARR: [i32; 5] = [-1, 2, 4, 42, 77];

    #[test]
    fn element_found() {
        assert_eq!((-1, 0), bin_search(&ARR, -1).unwrap());
    }
}
