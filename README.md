# Letterbox

An experimental esolang by Chris Natcharian.

This rust library defines a modular lexer and interpreter for the Letterbox language, which can be used together or separately in your Rust programs.

## How to write a Letterbox program

See the language documentation on [its Esolang Wiki page](https://esolangs.org/wiki/Letterbox).

## How to run a Letterbox program

The purpose of this crate is to define a simple, frontend-agnostic API with which more complex applications can run Letterbox programs in contained environments. Programs are interpreted from strings and create strings as output.

```rust
// import all required types
use letterbox_lang::prelude::*;

// get string representation of program
let program_string = "P'Hello world'".to_string();

// Create a lexer to consume the string
let lex: Lexer<LbToken> = LbToken::lexer(program_string);

// Create a new data storage struct on which the program will operate
let mut data: LbStorage = LbStorage::new();

// Get a string of whitespace-separated program arguments
let input_vec = "".to_string();

// Get an empty string with which to collect program output
let mut output_buffer = String::new();

// Define how many loop iterations are allowed before the program halts to prevent infinite loops
let loop_limit: usize = 100;

// Create a program struct which consumes the previous components
let mut program = LbProgram::new(
    lex,
    &mut data,
    &input_vec,
    &mut output_buffer,
    loop_limit
).expect("Error initializing program");

// Run the program. This can be done only once.
let program_result: Result<(), String> = program.run();

// If the program results in a string, an error has occurred.
// Otherwise, the program succeeded.
if let Err(msg) = program_result {
    println!("Error occurred: {}", msg);
}
else {
    println!("{}", output_buffer); // prints Hello world!
}
```

Programs exist separately from the data they operate upon. Storage objects can be reused by multiple Programs.