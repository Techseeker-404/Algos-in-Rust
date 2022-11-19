use back_track_maze::*;
use binary_search::bin_search;
use quick_sort::quick_sort;

//Test for Array data-type.
#[test]
fn test_bin_search_array() {
    let test_array = [2, 4, 6, 8, 10, 12];
    assert_eq!(4, bin_search(&test_array, 10).unwrap());
}

//Test for Vector data-type.
#[test]
fn test_bin_search_vector() {
    let test_vector = vec![2, 4, 6, 8, 10, 12];
    assert_eq!(4, bin_search(&test_vector, 10).unwrap());
}

//Test for Array data-type with str.
#[test]
fn test_bin_search_str_array() {
    let test_array_str = ["a", "b", "c", "d", "e"];
    assert_eq!(3, bin_search(&test_array_str, "d").unwrap());
}

//Test for Quick Sort in Rust for array.
#[test]
fn test_quick_sort_array() {
    let mut array: [usize; 8] = [7, 2, 1, 6, 8, 5, 3, 4];
    let _ = quick_sort(&mut array);
    assert_eq!([1, 2, 3, 4, 5, 6, 7, 8], array);
}
//Test for Quick Sort in Rust for array.
#[test]
fn test_quick_sort_float_array() {
    let mut array: [f32; 8] = [7.0, 2.0, 1.0, 6.0, 8.0, 5.0, 3.0, 4.0];
    let _ = quick_sort(&mut array);
    assert_eq!([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0], array);
}

//Test for Quick Sort in Rust for Vector.
#[test]
fn test_quick_sort_vector() {
    let mut vector = vec![8, 5, 4, 6, 3, 1, 2];
    let _ = quick_sort(&mut vector);
    assert_eq!(vec![1, 2, 3, 4, 5, 6, 8], vector);
}
//Test for Quick Sort in Rust for Vector.
#[test]
fn test_quick_sort_alpha_array() {
    let mut alpha_array = ["e", "d", "f", "b", "c", "a"];
    let _ = quick_sort(&mut alpha_array);
    assert_eq!(["a", "b", "c", "d", "e", "f"], alpha_array);
}

#[test]
fn test_rat_in_maze_logic() {
    // Problem MAZE -> (MATRIX)
    let maze_vec: Vec<Vec<u8>> = Vec::from([
        vec![1, 0, 1, 0, 1],
        vec![1, 1, 1, 0, 1],
        vec![0, 1, 1, 0, 0],
        vec![1, 0, 1, 0, 1],
        vec![1, 1, 1, 1, 1],
    ]);
    //Actual Path that Rat will go;
    let path_vec: Vec<Vec<u8>> = Vec::from([
        vec![1, 0, 0, 0, 0],
        vec![1, 1, 0, 0, 0],
        vec![0, 1, 1, 0, 0],
        vec![0, 0, 1, 0, 0],
        vec![0, 0, 1, 1, 1],
    ]);
    let mut vec_bit_map = ZeroBitFieldVector::new(5);
    back_track(&maze_vec, 0, 0, &mut vec_bit_map);
    assert_eq!(path_vec, vec_bit_map);
}
