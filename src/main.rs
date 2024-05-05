// fn main() { //this was practice from docs
//     // print and formatting print statements
//     println!("first print in rust");
//     print!("Hello {0}, this is {1}, {1} this is {0}.", "Alice", "Bob"); //print! prints on one line while println! will add a new line. You can also add the variable directly in the {} rather than the index position of them

//     // error print
//     eprint!("error in your code");

//     // primitive data types
//     // boolean
//     let logical: bool = true;
//     //float
//     let float_a: f64 = 1.0; //memory allotment
//     let float_b = 3.2; //defaults to f64
//                        // integer
//     let int = 5i32; //this is memory allotment
//     let int_a = 6; //defaults to i32
//     println!("{}", int); //can't just print the variable, you have to format it into a string
//                          //data type can be inferred from context
//     let number = 45;

//     // number = "hello"; //should error saying you cannot change type i.e. "expected integer, found `&str`"
//     println!("{}", number);

//     // operators (pretty standard operators)
//     println!("{}", number == 6);
//     // || or operator
//     //  && 'AND'
//     // ! 'NOt'
// } /*adding block comment */
// Practice from youtube video "Rust Programming Introduction-Beginner Crash Course"

fn main() {
    let mut name = "Kyle"; //by default variables are immutable must add 'mut' to make them mutable
    println!("{name}");
    println!("name variable is changing");
    name = "Billy";
    println!("{name}");
    let last_name = "Reimers";
    println!("{name} {last_name}");
    println!("Hello, {name} {}", last_name.to_lowercase()); //cannot use methods withing the curly brackets

    let arr = [1, 2, 3, 4, 5];
    println!("{arr:?}"); // must have the ':?' this formats the array in debug allowing you to see it in the print, otherwise you get an error. not good to use in a terminal application though.
}
