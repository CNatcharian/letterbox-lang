# Letterbox

An experimental esolang by Chris Natcharian.

The purpose of this crate is to define a simple, frontend-agnostic API with which more complex applications can run Letterbox programs in contained environments. Programs are interpreted from strings and create strings as output; data is stored independently and can be reused between programs.

## Parts of this crate

- `LbStorage` represents a bank of 26 variables, each stored under a lowercase letter of the alphabet ('a' to 'z'). Each can hold one String or one float.
- `LbToken` is an enum derived from [Logos](https://crates.io/crates/logos) that defines the valid tokens of the Letterbox language. An instance of `LbToken::lexer` can convert a textual Letterbox program into individual tokens AND parse out their arguments.
- `LbProgram` consumes a lexer containing zero or more tokens and executes them on some `LbStorage`. It can also accept program arguments and expose program output.

For more details, see the [crate docs](https://docs.rs/letterbox-lang/).

## How to write a Letterbox program

See the language documentation on [its Esolang Wiki page](https://esolangs.org/wiki/Letterbox).

## How to run a Letterbox program

This is a minimal example of a Rust program that executes a Letterbox program. For a more full-featured example, see [the Letterbox command line tool](https://github.com/CNatcharian/rs-letterbox).

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
