pub fn bin_search<T:Ord, U: AsRef<[T]>>(array: U, num_selec: T) -> Option<usize> {
    //or add where T: Ord
    let array = array.as_ref();
    let mut left = 0;
    let mut right = array.len();

    while left < right {
        let mid = left + (right - left) / 2;
        println!("Mid Value:- {}", mid);
        if array[mid] == num_selec {
            return Some(mid);
        }
        else if array[mid] < num_selec {
            left = mid + 1;
        } else {
            right = mid;
        }
        println!("Iterating .... \n");
    }

   None   // Final Return
}

#[cfg(test)]
mod tests {
    //super::*;
    use crate::bin_search;
    #[test]
    fn test_binary_search() {

        let arr: Vec<i32> = vec![2, 5, 6, 7, 8, 12, 23, 26];
        let res = bin_search(&arr, 11);
        if let Some(num) = res {
            println!("Resultant Key is in {:?}", num);
            assert_eq!(11, num);
        } else {
            println!("Not found the Key in array");
        }
    }
}
