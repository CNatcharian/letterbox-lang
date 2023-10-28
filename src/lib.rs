// letterbox-lang
// An experimental esolang by Chris Natcharian

mod storage;
mod lb_lexer;
mod program;

pub mod prelude {
    pub use logos::{Logos, Lexer};
    pub use crate::program::LbProgram;
    pub use crate::storage::LbStorage;
    pub use crate::lb_lexer::LbToken;
    pub use crate::program::Val;
}

#[cfg(test)]
mod lb_tests;
