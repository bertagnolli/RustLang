fn main() {
    let arr = [0, 1, 2, 3]; // Known length
    let slice = &arr[1 .. 3]; // A slice is a sement of an array. Syntax: let sliceName = &slicedArray[index .. <indexN-1>]; Last index isn't inclusive
    // array above will return arr[1 to 2], 3 is not inclusive
    borrowing_slice(arr, slice);
}

fn borrowing_slice(arr: [u8; 4], slice: &[u8]) {
    println!("{:?}", arr);
    println!("{:?}", slice);
    println!("length: {}", slice.len());
    println!("{} {}", slice[0], slice[1]);
}
