fn main() {
    let an_integer = 1u;
    let a_boolean = true;
    let unit = ();
    // copy `an_integer` into `copied_integer`
    let copied_integer = an_integer;
    println!("An integer: {}", copied_integer);
    println!("A boolean: {}", a_boolean);
    println!("Meet the unit value: {}\n", unit);
    let _unused_variable = 3u;
    let _noisy_unused_variable = 2u;
    //  ^ Prefixing with an underscore supresses the unused variable warning

    let _immutable_variable = 1i;
    let mut mutable_variable = 1i;
    println!("Before mutation: {}", mutable_variable);
    mutable_variable += 1;
    println!("After mutation: {}", mutable_variable);
    // Error!
    // _immutable_variable += 1;
    let long_lived_var = 1i;
    {
        let short_lived_var = 2i;
        println!("inner short: {}", short_lived_var);
        let long_lived_var = 5_f32;
        println!("inner long: {}", long_lived_var);
    }
    // Error!
    // println!("outer short: {}", short_lived_var);

    println!("\nouter long: {}", long_lived_var);
    let a_variable;
    {
        let x = 2i;
        a_variable = x * x;
    }
    println!("a variable: {}", a_variable);
    let another_variable;
    // Error! Use of uninitialized variable
    // println!("another var: {}", another_variable);
    another_variable = 1i;
    println!("another var: {}", another_variable);
}
