fn main() {
    let a: my_enum = my_enum::A;
    let b: my_enum = my_enum::B(5);
    let c: my_enum = my_enum::C{x:10, y:20};
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
}

#[derive(Debug)] // This here is needed to print info inside structs and enums for debugging
enum my_enum {
    A,
    B(u32),
    C {x: u32, y: u32}
}
