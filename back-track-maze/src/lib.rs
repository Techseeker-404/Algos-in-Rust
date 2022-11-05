#![allow(dead_code)]
use std::fmt;
use std::result::Result;

struct ZeroBitFieldVector;

impl ZeroBitFieldVector {
    fn new(capacity: usize) -> Vec<Vec<u8>> {
        let mut vec_arr = Vec::with_capacity(capacity);
        let mut cnt: usize = 0;
        'push: loop {
            vec_arr.push(vec![0u8; capacity]);
            cnt += 1;
            if cnt == capacity {
                break 'push;
            };
        }
        vec_arr
    }
}

#[derive(Debug)]
enum VecDispError {
    UnInitialized(&'static str),
}

fn display_2d_vec<T: fmt::Debug>(vec: &Vec<T>) -> Result<(), VecDispError> {
    if vec.len() == 0 {
        return Err(VecDispError::UnInitialized("Un-Initialized vector"));
    }
    for i in 0..vec.len() {
        println!("{:?}\n", vec[i]);
    }
    Ok(())
}

fn good_to_proceed(vector: &Vec<Vec<u8>>, x: usize, y: usize) -> bool {
    if x < vector.len() && y < vector.len() && vector[x][y] == 1u8 {
        return true;
    } else {
        return false;
    }
}

fn back_track(vector: &Vec<Vec<u8>>, x: usize, y: usize, soln_vect: &mut Vec<Vec<u8>>) -> bool {
    if x == vector.len() - 1 && y == vector.len() - 1 {
        soln_vect[x][y] = 1u8;
        return true;
    }
    if good_to_proceed(&vector, x, y) {
        soln_vect[x][y] = 1u8;
        if back_track(&vector, x + 1usize, y, soln_vect) {
            return true;
        } else if back_track(&vector, x, y + 1usize, soln_vect) {
            return true;
        } else {
            soln_vect[x][y] = 0u8; // back-tracking
            return false;
        }
    }

    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rat_in_maze_logic() {
        //Maze that Rat is stucked in.
        let maze_vec: Vec<Vec<u8>> = Vec::from([
            vec![1, 0, 1, 0, 1],
            vec![1, 1, 1, 1, 1],
            vec![0, 1, 0, 1, 0],
            vec![1, 0, 0, 1, 1],
            vec![1, 1, 1, 0, 1],
        ]);
        //Actual Path that Rat will traverse;
        let path_vec: Vec<Vec<u8>> = Vec::from([
            vec![1, 0, 0, 0, 0],
            vec![1, 1, 1, 1, 0],
            vec![0, 0, 0, 1, 0],
            vec![0, 0, 0, 1, 1],
            vec![0, 0, 0, 0, 1],
        ]);
        let mut vec_bit_map = ZeroBitFieldVector::new(5);
        //if back_track(&maze_vec, 0, 0,&mut vec_bit_map) {
        //    display_2d_vec(&vec_bit_map).unwrap_or_else(|err| println!("Error: {err:?}"));
        //}
        back_track(&maze_vec, 0, 0, &mut vec_bit_map);
        assert_eq!(path_vec, vec_bit_map);
    }
}
