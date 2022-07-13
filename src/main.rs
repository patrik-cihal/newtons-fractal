mod camera;
mod real_root;
mod complex_root;
mod sketch_complex_root;
use std::io::stdin;

fn main() {
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).unwrap();
    let buffer = buffer.trim();
    if buffer == "complex" {
        complex_root::init();
    }
    else {
        real_root::init();
    }
}

pub fn permutations<T: Clone>(v: &Vec<T>, length: usize) -> Vec<Vec<T>> {
    return _permutations(v, length as i32, v.len() as i32-1, vec![]);
}

// generate permutations from list of given length
fn _permutations<T: Clone>(v: &Vec<T>, length: i32, pos: i32, result: Vec<T>) -> Vec<Vec<T>> {
    if length == 0 {
        return vec![result];
    }
    if pos==-1 {
        return vec![];
    }

    let mut v1 = result.clone();
    v1.push(v[pos as usize].clone());

    let mut answer = _permutations(v, length-1, pos-1, v1);
    if pos >= length {
        answer.append(&mut _permutations(v, length, pos-1, result));
    }
    return answer;
}