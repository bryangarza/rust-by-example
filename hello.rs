fn main() {
    println!("Hello World!");
    // `{}` are placeholder arguments that will be stringified
    // `i` suffix indicates to the compiler that this literal has a type:
    // in this case a signed pointer size integer
    println!("{} days", 31i);
    // Positional arguments can be reused
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    // Name arguments
    println!("{subject} {verb} {predicate}",
             predicate="over the lazy dog",
             subject="the quick brown fox",
             verb="jumps");
    println!("{} of {:b} people know binary, the other half don't", 1i, 2i);
    println!("My name is {0}, {1} {0}", "Bond", "James");
    println!("1 + 2 = {}", 1u + 2);
    println!("1 - 2 = {}", 1i - 2);
    // Short-circuiting boolean logic
    println!("true AND flase is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);
    //Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u ^ 0b0101);
    println!("1 << 5 is {}", 1u << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u >> 2);
    // Use underscore to improve readability!
    println!("One million is written as {}", 1_000_000u);
}
