fn main() {
    // Type annotated
    let a_float: f64 = 1.0;
    // This variable is an `int`
    let mut an_integer = 5i;
    // Error! The type of a variable can't be changed
    // an_integer = true;

    let decimal = 65.4321_f32;
    // Equivalent:
    // let decimal: f32 = 65.4321;

    // Error! No implicit conversion
    // let integer: u8 = decimal;

    // Explicit conversion
    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);


    // Suffixed literals, their types are known at initialization
    let x = 1u8;
    let y = 2u;
    let z = 3f32;

    // Unsuffixed literal, their types depend on how they are used
    let i = 1;
    let f = 1.0;

    // `size_of_val` returns the size of a variable in bytes
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));

    // Constraints (summands must have the same type) for `i` and `f`
    let _constraint_i = x + i;
    let _constraint_f = z + f;
    // ^ Try commenting out these two lines


    //Using local inference, the compiler knows that `elem` has type u8
    let elem = 5u8;

    // Create an empty vector (a growable array)
    let mut vec = Vec::new();
    // At this point the compiler doesn't know the exact type of `vec`, it
    // just knows that it's a vector of something(`Vec<_>`)

    // Insert `elem` in the vector
    vec.push(elem);
    // Aha! Now the compiler knows that `vec` is a vector of `u8`s (`Vec<u8>`)
    // ^ Try commenting out the `vec.push(elem)` line

    println!("{}", vec);


    // `NanoSecond` is a new name for `u64`
    type NanoSecond = u64;
    type Inch = u64;

    // Use an attribute to silence warning
    #[allow(non_camel_case_types)]
    type uint64_t = u64;

    // `Nanosecond` = `Inch` = `uint64_t` = `u64`
    let nanoseconds: NanoSecond = 5 as uint64_t;
    let inches: Inch = 2 as uint64_t;

    // Note that aliases *don't* provide any extra type safety, because
    // aliases are *not* new types
    println!("{} nanoseconds + {} inches = {} unit?",
             nanoseconds,
             inches,
             nanoseconds + inches);

    // Only primitives types (ie `uint`, `f32`, etc) can have non-camelcase
    // names without a warning.

    // The main use of aliases is to reduce typing, for example the
    // `IoResult<T>` type is an alias for `Result<T, IoError>` type.
}
