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
}
