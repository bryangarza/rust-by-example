fn main() {
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
}
