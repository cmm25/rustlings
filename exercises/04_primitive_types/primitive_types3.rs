// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.

// I AM NOT DONE

fn main() {
    let a = [2,3,2,1,3,45,5,2,2,1,4,6,2,4,13,235,2,412,314,4];
    //when initializing an array with ist required length eg let c :[u32:3] =[]
    //the length must be met
    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed")
    }
}
