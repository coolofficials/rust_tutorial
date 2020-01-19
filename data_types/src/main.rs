// rust has two data type subsets: scalar and compound.
// rust is a statically typed language, which means that it must know the types of all variables at compile time.
// the compiler can usually infer what type we want to use based on the value and how we use it.
// we must add a type annotation, in cases when many types are possible.

fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{}", guess);
    println!("{}", 43 % 5);
    println!("{}", 43.5 % 5.0);
//  println!("{}", 43.5 % 5); type error
    println!("{}", 43.58 % 5.1); // should return 2.78 but returning 2.780000000000001
    println!("{}", 43.58 / 5.1);
}

// **** salar types: integers, floating-point numbers, booleans, characters. ****
// a scalar type represents a single value.

// [Integer types]
// 8, 16, 32, 64 -bit signed, unsigned. (i8, u8, i16, u16, i32, u32, i64, u64).
// isize, usize follows architecture-bit(32-bit or 64-bit).
// other integers: Decimal, Hex, Octal, Binary, Byte.
// singed numbers are stored using two's complement.
// signed n-bit: -2^(n-1) ~ 2^(n-1)-1 / unsigned n-bit: 0~2^n-1.
// * Integer Overflow *
// rust compiler in debug mode checks for integer overflow that cause panics program at runtime.
// release mode compiler does not include check for integer overflow. instead, if overflow occurs, rust performs two's complement wrapping(in the case of u8, 256 becomes 0). this behavior is considered an error even if program won't panic.

// usually i32, a standart integer type of rust, is recommended.

// [Floating-point numbers type]
// f32(single precision), f64(double precision,standard in rust)
// follows IEEE-754 standard.

// * Numeric Operations *
// supports the basic mathematical operations for numbers: addition, subtraction, multiplication, division, and remainder(+, -, *, /, %).
// see Appendix B.

// [The Boolean Type]
// value 'true' or 'false'. type specified using 'bool'.

// [The Character Type]
// using '' while string type uses "".
// 'char' type is the language's most primitive alphabetic type.
// Unicode Scalar (U+0000 ~ U+D7FF, U+E000 ~ U+10FFFF)



// **** compound types: tuples, arrays. ****
// compound types can group multiple values into one type.

// [The Tuple Type]