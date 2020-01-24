// declare with fn, conventional style: 'snake' = 'another_function();'
// function can be globally detected(function can be declared before and after main function)

fn main() {
    println!("Hello, world!");
    
    another_function(5);
}

// function can have parameters.
// concrete values for parameters are called arguments.
// fn function(x:i 32) {~~}
// function(5)
// x: parameter, 5: argument.

// must declare types of parameters.
fn another_function(x: i32) {
    println!("x의 값: {}", x);
}

// statements vs expression
// statements performs an action, expression returns value.
// let y = 6; >>> statements
// fn main() >>> statements
// let x = (let y = 6); >>> error since (let y = 6) has no return value.
// function call & macro call & numbers(values) & operations(which returns a value) >> expression.
// *** no ; at the end of expression.
// example)
// let y = { let x = 3; x + 1 };

// function returning value: fn five() -> i32 { 5 } (must anotate type).
// function returning value: fn add_one(x:i32) -> i32 { x + 1 }
// fn add_one(x: i32) -> i32 { x + 1; } >>>> error case. x+1; is a statement returning (), while x+1 is an expression returning i32.