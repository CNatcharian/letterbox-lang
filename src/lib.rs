// letterbox-lang
// An experimental esolang by Chris Natcharian

mod storage;
mod lb_lexer;
mod program;

pub mod prelude {
    pub use logos::{Logos, Lexer};
    pub use crate::program::Program as LbProgram;
    pub use crate::storage::Storage as LbStorage;
    pub use crate::lb_lexer::LBT as LbToken;
}

#[cfg(test)]
mod lb_tests;
