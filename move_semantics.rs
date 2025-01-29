//Q1
// // TODO: Fix the compiler error in this function.
// fn fill_vec( vec: Vec<i32>) -> Vec<i32> {
//     let mut vec = vec;

//     vec.push(88);

//     vec
// }

// fn main() {

//     let vec0 = vec![65, 97, 6];
//     let vec1 = fill_vec(vec0);
//     println!("{:?}", vec1);
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn move_semantics1() {
//         let vec0 = vec![22, 44, 66];
//         let vec1 = fill_vec(vec0);
//         assert_eq!(vec1, vec![22, 44, 66, 88]);
//     }
// }


// //-----------------
// //Q2

// fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
//     let mut vec = vec;

//     vec.push(88);

//     vec
// }

// fn main() {
//     let vec0 = vec![76, 35, 65];

//     let vec1 = fill_vec(vec0.clone()); 
//     println!("{:?}", vec1);
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     // TODO: Make both vectors `vec0` and `vec1` accessible at the same time to
//     // fix the compiler error in the test.
//     #[test]
//     fn move_semantics2() {
//         let vec0 = vec![22, 44, 66];

//         let vec1 = fill_vec(vec0.clone());


//         assert_eq!(vec0, [22, 44, 66]);
//         assert_eq!(vec1, [22, 44, 66, 88]);
//     }
// }




// -----------
//Q3
// TODO: Fix the compiler error in the function without adding any new line.
fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(88);

    vec
}

fn main() {
    // You can optionally experiment here.
    let vec0 = vec![65, 97, 6];
    let vec1 = fill_vec(vec0);
    println!("{:?}", vec1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics3() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}