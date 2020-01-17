// follow tags with 1. to the end of code and comeback to first line and follow .2 tags, .3 and so on.

// 2. extern implies that we have external crate dependency. 'use rand' does same thing.
// 2. we can use rand:: to call everything in rand.
extern crate rand;

// 1. get io(input/output) library from std(standard library) into scope with 'use' method.
use std::io;

// 3. get an enum type 'Ordering' in cmp(comparing) from std(standard library).
// 3. variants of Ordering are Less, Greater or Equal.
// 3. cmp method compares
use std::cmp::Ordering;

// 2. call Rng of rand which is trait of methods in rand.
use rand::Rng;

// 1. rust use 4-spaces instead of tab. every line ends with ;
// 1. use (cargo fmt) to rearrange in an official form.

// 1. main function runs primary in every .rs file. It has no parameter.
fn main() {
    // 1. println! is a bit different with println. It is rust macro while other is function.
    println!("Guess the number!");

    // 2. rand::thread_rng gives particular local random number generator.
    // 2. gen_range method is defined in trait 'Rng'. '.gen_range(i, j+1) will return arbitrary number from i to j.
    // 2. every crait has it's own document. 'cargo doc --open' gives document of every crait with dependency and open in browser.
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // 7. finalize the code. hide password!!!
    // 7. println!("The secret number is: {}", secret_number);

    // 5. loop without break gives endless loop.
    // 5. program will end if user puts weird input which is not an integer(e.g. 'quit') or ctrl+c.
    loop {
        println!("Please input your guess.");

        // 1. use let to declare new variables. mut stand for mutable.
        // 1. variables in rust is basically immutable.
        // 1. '::' in '::new' implies that new is a method of type 'String'.
        // 1. we call this kind of method as 'associated function' or in other languages we often call it as 'static method'.
        let mut guess = String::new();

        // 1. std::io + io::stdin => std::io::stdin (without 'use std::io' we need std::io::stdin).
        // 1. '.read_line(&mut guess)' passes '&mut guess' into '.read_line' method of associated function 'stdin'.
        // 1. '.read_line' method need mutable string as an 'argument'.
        // 1. & indicates that this argument is a reference.
        // 1. one of rust's major advantages is how safe and easy it is to use references.
        // 1. in standard rust code, we separate methods into different lines (just run cargo fmt).

        // 1. '.read_line' method not only saves user's input but also return io::Result.
        // 1. rust has several 'Result' types in std library, such as generic Result or io:Result.
        // 1. Result types are enums(enumerations). enums is a type that can have a fixed set of values, which are called "the enum's 'variants'".
        // 1. variants of 'Result' are 'Ok' and 'Err'. 'Ok' implies that the process was successful and it contains result value. 'Err' implies it was not successful and also it includes the reason of failure.
        // 1. '.expect' is a method of 'io::Result'. If instance of io::Result is 'Err', .expect ceases program and prints message of 'Err'(reason of failure).
        // 1. if io::Result is 'Ok', '.expect' returns value of 'Ok'.
        // 1. without using '.expect', it will be compiled but will show warning message.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // 4. shadows variable 'guess' which is a mutable string(rust allows shadowing).
        // 4. guess in expression follows an input value string.

        // 4. 'trim' method on a string instance will eliminate any whitespace at the beggining and end.
        // 4. 'read_line' method forces user to press enter key, so input will have \n, and 'trim' will eliminate \n.

        // 4. 'parse' method parses a string into an integer. we need to declare exact type since parse can give various number types.
        // 4. guess':' u32 implies that we declared a type of the variable.
        // 4. 'parse' method cannot change a string into an integer if it includes other characters. so we need '.expect' similar in '.read_line.expect(~~~)'.
        // 4. let guess: u32 = guess.trim().parse().expect("Please type a number!");

        // 6. let program continues even if user put weird inputs.
        // 6. Ok(num) => num returns num if .parse is Ok.
        // 6. '_' matches to anything. Err('_') matches every Err values.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // 1. '{}' is a 'placeholder' indicates a location of the value.
        // 1. 'println!("x = {} and y = {} and z = {}", x, y, z); => correct placing.
        // 1. 'println!("x = {} and z = {} and y = {}", x, y, z); => incorrect placing.
        println!("You guessed: {}", guess);

        // 3. cmp method compares two values and can be called on anything that can be compared.
        // 3. it takes a reference to whatever you want to compare with.
        // 3. match expression is made up of arms(similar with branches of a tree). an arm consists of a pattern and the code that should be run if the vlue given to the beginning of the 'match' expression fits that arm's pattern.
        // 3. the code will not be compiled since rust has a static type system.
        // 3. guess is a String, while secret_number is an integer(without mentioning, i32).
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            // 3. Ordering::Equal => println!("You win!"),
            Ordering::Equal => {
                println!("You win!");
                // 5. breaks when user gives an answer.
                break;
            }
        }
    }
}

// 1. 'Crate' is a collection of Rust source code files.
// 1. our project is a 'binary crate'.
// 1. 'rand crate' is a library crate for other programs.
// 1. to use crates, add dependency in Cargo.toml, then just compile it. rust will automatically download everything needed for compiling process.

// 1. Cargo.lock ensures reproducible builds.
// 1. if we don't change anything in cargo.toml, even if crate is updated after, our project will use current version in cargo.toml.
// 1. to update crates, 'cargo update'. make sure to change version in cargo.toml.
