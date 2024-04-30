fn main() {
    // print and formatting print statements
    println!("first print in rust");
    print!("Hello {0}, this is {1}, {1} this is {0}.", "Alice", "Bob"); //print! prints on one line while println! will add a new line

    // error print
    eprint!("error in your code");

    // primitive data types
    // boolean
    let logical: bool = true;
    //float
    let float_a: f64 = 1.0; //memory allotment
    let float_b = 3.2; //defaults to f64
                       // integer
    let int = 5i32; //this is memory allotment
    let int_a = 6; //defaults to i32
    println!("{}", int); //can't just print the variable, you have to format it into a string
                         //data type can be inferred from context
    let number = 45;

    // number = "hello"; //should error saying you cannot change type i.e. "expected integer, found `&str`"
    println!("{}", number);

    // operators (pretty standard operators)
    println!("{}", number == 6);
    // || or operator
    //  && 'AND'
    // ! 'NOt'
} /*adding block comment */
