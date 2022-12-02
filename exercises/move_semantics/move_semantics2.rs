// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.

// i know solution 2 is correct, solution 1 and 3 unclear

// fn main() {
//     let vec0 = Vec::new();

//     let mut vec1 = fill_vec(vec0);

//     // Do not change the following line!
//     println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

//     vec1.push(88);

//     println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
// }

// fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
//     let mut vec = vec;

//     vec.push(22);
//     vec.push(44);
//     vec.push(66);
//     vec
// }

// SOLUTION 1 - then fill vec doesn't work as desired, but apparently it's OK cause it compiles
// fn main() {
//     let vec0 = Vec::new();

//     let mut vec1 = fill_vec(vec0.clone());

//     // Do not change the following line!
//     println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

//     vec1.push(88);

//     println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
// }

// fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
//     let mut vec = vec;

//     vec.push(22);
//     vec.push(44);
//     vec.push(66);
//     vec
// }

// SOLUTION 2

fn main() {
    let mut vec0 = Vec::new();

    let mut vec1 = fill_vec(&mut vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: &mut Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);
    vec.clone()
}

// SOLUTION 3
// i didn't do it but i found solutions, to try this and review later https://github.com/attilahorvath/rustlings-solutions/tree/master/move_semantics

// So, `vec0` is passed into the `fill_vec` function as an argument. In Rust,
// when an argument is passed to a function and it's not explicitly returned,
// you can't use the original variable anymore. We call this "moving" a variable.
// Variables that are moved into a function (or block scope) and aren't explicitly
// returned get "dropped" at the end of that function. This is also what happens here.
// There's a few ways to fix this, try them all if you want:
// 1. Make another, separate version of the data that's in `vec0` and pass that
//    to `fill_vec` instead.
// 2. Make `fill_vec` borrow its argument instead of taking ownership of it,
//    and then copy the data within the function in order to return an owned
//    `Vec<i32>`
// 3. Make `fill_vec` *mutably* borrow a reference to its argument (which will need to be
//    mutable), modify it directly, then not return anything. Then you can get rid
//    of `vec1` entirely -- note that this will change what gets printed by the
//    first `println!`
