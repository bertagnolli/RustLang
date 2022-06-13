fn main() {
    // String below is type string Literal, stored in Stack memory and immutable
    let mut stringLiteral = "hello world";

    // this string below is of type String and will be stored in heap memory. This String type can be altered if 
    // it is set as 'mut'
    let mut string = String::from("Hello World");

    let slice = &string[.. 6];

    println!("{}", slice);
    println!("{}", &string);

    string.push('a'); // pushes character a to string
    string.push_str(" add string"); // adds " add string" to string
    println!("{}", string); 
    string = string.replace("add", "replace"); // replaces the word "add" with word "replace"
    println!("{}", string); 
}

