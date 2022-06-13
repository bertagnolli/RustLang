fn main() {
    // Tuples can hold different types of data
    let tupleEx: (u8, bool, f32) = (5, true, 2.1);

    // Print each element in a tuple, syntax is tupleName.index
    println!("first {}, second {}, third {}", tupleEx.0, tupleEx.1, tupleEx.2);

    // Print all elements inside a tuple, same as array {:?:}    
    println!("{:?}", tupleEx);

    // Descontruct a tuple
    let (a, b, c) = tupleEx;
    println!("Disconstructed tuple {} {} {}", a, b, c);

}

