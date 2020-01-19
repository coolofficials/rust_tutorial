fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let mutability = 5;
    println!("The value of mutability is: {}", mutability);
    let mutability = "test";
    println!("The value of mutability is: {}", mutability);

    let shadow = 5;
    
    let shadow = shadow + 1;

    let shadow = shadow * 2;

    println!("The value of shadow is: {}", shadow);
}

// use let keyword to declare variables.
// choice of mutable and immutable: if data structure is large, using mutable instance maybe faster than assigning new instance to assign new variable. if data structure is small, use new instance and write down in functional programming style; this will help others to read code easily.
// but still, mutable variable's type will be fixed without shadowing(re-assigning with let).

// for constant(const), mut is not allowed.
// must annotate the type always for constant.
// const MAX_POINTS: u32 = 100_000;
// constants are valid for the entire time a program runs, within the scope they were declared in.

// shadowing is allowed.
// note that shadowing is different with mutability, we need to use let.
// shadowing is useful to change variable's type.
// let spaces = "   ";
// let spaces = spaces.len();

// in case of mutable varaible,
// let mut spaces = "   ";
// spaces = spaces.len();
// is not allowed and gives type error.