// move_semantics2.rs
//
// Expected output:
// vec0 has length 3, with contents `[22, 44, 66]`
// vec1 has length 4, with contents `[22, 44, 66, 88]`
//
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand
// for a hint.



// 1st Implementation
//fn main() {
//    let vec0 = Vec::new();
//
//    let mut vec1 = fill_vec(vec0.clone());
//
//    println!("{} has length {}, with contents: `{:?}`", "vec0", vec0.len(), vec0);
//    // vec0 has length 0, with contents: `[]`
//
//    vec1.push(88);
//
//    println!("{} has length {}, with contents `{:?}`", "vec1", vec1.len(), vec1);
//    // vec1 has length 4, with contents `[22, 44, 66, 88]`
//
//}
//
//fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
//    let mut vec = vec;
//
//    vec.push(22);
//    vec.push(44);
//    vec.push(66);
//
//    vec
//}

// 2nd Implementation
//fn main() {
//    let mut vec0 = Vec::new();
//
//    fill_vec(&mut vec0);
//
//    println!("{} has length {}, with contents: `{:?}`", "vec0", vec0.len(), vec0);
//    // vec0 has length 3, with contents: `[22, 44, 66]`
//
//    vec0.push(88);
//
//    println!("{} has length {}, with contents `{:?}`", "vec0", vec0.len(), vec0);
//    // vec0 has length 4, with contents `[22, 44, 66, 88]`
//}
//
//fn fill_vec(vec: &mut Vec<i32>) {
//
//    vec.push(22);
//    vec.push(44);
//    vec.push(66);
//
//}

// 3rd implementation
fn main() {
    let mut vec0 = Vec::new();

    let mut vec1 = fill_vec(&mut vec0);

    println!("{} has length {}, with contents: `{:?}`", "vec0", vec0.len(), vec0);
    // vec0 has length 3, with contents: `[22, 44, 66]`

    vec1.push(88);

    println!("{} has length {}, with contents `{:?}`", "vec1", vec1.len(), vec1);
    // vec1 has length 4, with contents `[22, 44, 66, 88]`

}

fn fill_vec(vec: &mut Vec<i32>) -> Vec<i32> {

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec.clone()
}
