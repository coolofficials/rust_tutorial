// get io(input/output) library from std(standard library) into scope with 'use' method.
use std::io;

// rust use 4-spaces instead of tab. every line ends with ;
// use (cargo fmt) to rearrange in an official form.

// main function runs primary in every .rs file. It has no parameter.
fn main() {

    // println! is a bit different with println. It is rust macro while other is function.
    println!("Guess the number!");

    println!("Please input your guess.");

    // use let to declare new variables. mut stand for mutable.
    // variables in rust is basically immutable.
    // '::' in '::new' implies that new is a method of type 'String'.
    // we call this kind of method as 'associated function' or in other languages we often call it as 'static method'.
    let mut guess = String::new();

    // std::io + io::stdin => std::io::stdin (without 'use std::io' we need std::io::stdin).
    // .read_line(&mut guess) passes '&mut guess' into 'read_line' method of associated function 'stdin'.
    // .read_line method need mutable string as an 'argument'.
    // & indicates that this argument is a reference.
    // one of rust's major advantages is how safe and easy it is to use references.
    // in standard rust code, we separate methods into different lines (just run cargo fmt).

    // .read_line method not only saves user's input but also return io::Result.
    // rust has several 'Result' types in std library, such as generic Result or io:Result.
    // Result types are enums(enumerations). enums is a type that can have a fixed set of values, which are called "the enum's 'variants'".
    // variants of 'Result' are 'Ok' and 'Err'. 'Ok' implies that the process was successful and it contains result value. 'Err' implies it was not successful and also it includes the reason of failure.
    // .expect is a method of 'io::Result'. If instance of io::Result is 'Err', .expect ceases program and prints message of 'Err'(reason of failure).
    // if io::Result is 'Ok', .expect returns value of 'Ok'.
    // without using .expect, it will be compiled but will show warning message.
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // '{}' is a 'placeholder' indicates a location of the value.
    // 'println!("x = {} and y = {} and z = {}", x, y, z); => correct placing.
    // 'println!("x = {} and z = {} and y = {}", x, y, z); => incorrect placing.
    println!("You guessed: {}", guess);
}
