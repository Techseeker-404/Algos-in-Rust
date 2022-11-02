/// Quick sort in Rust.
/// Not for float data types as PartialOrd not implemented
pub fn quick_sort<T>(arr: &mut [T]) 
where T: Copy + PartialOrd + PartialEq,
{
    if arr.len() > 1 {
        let prt_index = partition(arr);
        quick_sort(&mut arr[..prt_index]);
        quick_sort(&mut arr[prt_index+1..]);
    }
}

///Partition Function to partition the iterator
///data-type eg: Array or Vector type.
///Not for float data types as PartialOrd not implemented
fn partition<T: Copy+ PartialOrd + PartialEq>(arr: &mut [T]) -> usize {
    let lst_idx = arr.len() - 1;
    let pivot = arr[lst_idx];
    let mut p_index = 0;
    for idx  in p_index..lst_idx {
        if arr[idx] <= pivot {
            arr.swap(idx, p_index);
            p_index += 1;
        }
    }
    arr.swap(p_index, lst_idx);
    return p_index;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition_function() {
        let mut array: [usize; 8] =  [7, 2, 1, 6, 8, 5, 3, 4];
        let res = partition(&mut array);
        assert_eq!(3, res);
    }
}
