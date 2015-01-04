fn function() {
    println!("called `function()`");
}

// A module name `my`
mod my {
    // A module contain items likes functions
    #[allow(dead_code)]
    fn function() {
        println!("called `my::function()`");
    }

    // Modules can be nested
    mod nested {
        #[allow(dead_code)]
        fn function() {
            println!("called `my::nested::function()`");
        }
    }
}

fn main() {
    function();

    // Items inside a module can be called using their full path
    // THe `println` function lives in the `stdio` module
    // The `stdio` module lives in the `io` module
    // And the `io` module lives in the `std` crate
    std::io::stdio::println("Hello World!");

    // Error! `my::function` is private
    // my::function();
    // ^ Comment out this line
}
