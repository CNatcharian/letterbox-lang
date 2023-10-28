use logos::{Logos, Lexer};

/// A Logos-derived enum that can split a Letterbox program
/// into individual tokens AND parse out their arguments.
#[derive(Logos, Debug, PartialEq, Clone)]
pub enum LbToken {
    /// Save a value into a variable
    /// 
    /// Usage: `Sa4`
    #[regex(r"S[a-z]\-?[0-9]+(\.[0-9]+)?", save_number)]
    SaveNumber((char, f64)),

    /// Save a value into a variable
    /// 
    /// Usage: `S'hello'`
    #[regex(r"S[a-z]'[^']*'", save_str)]
    SaveStr((char, String)),

    /// Copy the value of a variable into another.
    /// 
    /// Usage: `Cab`
    #[regex(r"C[a-z][a-z]", copy)]
    Copy((char, char)),

    /// Append the value of variable b to variable a.
    /// If a is not a string, it becomes a string.
    /// The type of the appended variable is not affected.
    /// 
    /// Usage: `Aab`
    #[regex(r"A[a-z][a-z]", copy)]
    Append((char, char)),

    /// Print the value of the given variable
    /// 
    /// Usage: `Pa`
    #[regex(r"P[a-z]", single_var_arg)]
    PrintVar(char),

    /// Print the given string directly. Doesn't save it into storage.
    /// 
    /// Usage: `P'hello world'`
    #[regex(r"P'[^']*'", print_str)]
    PrintStr(String),

    /// Performs a mathematical operation.
    /// 
    /// Usage: `MAabc`
    #[regex(r"M[A-Z][a-z][a-z][a-z]", math_op)]
    MathOp((char, char, char, char)),

    /// Performs a boolean operation.
    /// 
    /// Usage: `BXabc`
    #[regex(r"B[A-Z][a-z][a-z][a-z]", bool_op)]
    BoolOp((char, char, char, char)),

    /// Performs command X, a times
    /// 
    /// Usage: `LaX`
    #[regex(r"L[a-z][A-Za-z]+", base_loop)]
    Loop((char, Box<LbToken>)),

    /// If a is nonzero, perform command X
    /// 
    /// Usage: `IaX`
    #[regex(r"I[a-z][A-Za-z]+", base_loop)]
    IfStatement((char, Box<LbToken>)),

    /// If a IS EQUAL TO ZERO, perform command X
    /// 
    /// Usage: `UaX`
    #[regex(r"U[a-z][A-Za-z]+", base_loop)]
    Unless((char, Box<LbToken>)),

    /// While a is nonzero, repeat command X
    /// 
    /// Usage: `WaX`
    #[regex(r"W[a-z][A-Za-z]+", base_loop)]
    WhileLoop((char, Box<LbToken>)),

    /// Reset variable a to 0.
    /// 
    /// Usage: `Ra`
    #[regex(r"R[a-z]", single_var_arg)]
    ResetVar(char),

    /// Reset all variables.
    /// 
    /// Usage: `RA`
    #[regex(r"RA")]
    ResetAll,

    /// Gets nth input and stores it in variable a as type X (N or S)
    /// 
    /// Usage: `GXa1`
    #[regex(r"G[A-Z][a-z][0-9]+", get_input)]
    GetInput((char, char, f64)),

    /// If a is nonzero, set it to 0, else set it to 1.
    /// 
    /// Usage: `Na`
    #[regex(r"N[a-z]", single_var_arg)]
    Negate(char),

    /// Finishes the program immediately.
    /// 
    /// Usage: `F`
    #[regex(r"F")]
    Finish,

    /// Executes a string value as a Letterbox program.
    /// Replaces any number of parameters with different variables.
    /// 
    /// Usage: `Xzacbd`
    #[regex(r"X[a-z]([a-z][a-z])*", execute_var)]
    Execute((char, String)),

    /// Unrecognized character(s)
    #[error]
    // skip comments
    #[regex(r"![^\n\r]*", logos::skip)]
    // skip whitespace
    #[regex(r"[ \t\n\f\r]+", logos::skip)]
    Error,
}

// Parser methods!

fn save_number(lex: &mut Lexer<LbToken>) -> Option<(char, f64)> {
    let token = lex.slice();
    let var_name = token.chars().nth(1);
    let num = token[2..].parse::<f64>().ok();
    if let None = var_name {
        return None;
    }
    if let None = num {
        return None;
    }
    Some((var_name.unwrap(), num.unwrap()))
}

fn save_str(lex: &mut Lexer<LbToken>) -> Option<(char, String)> {
    let token = lex.slice();
    let var_name = token.chars().nth(1);
    let my_str = String::from(token[2..].trim_matches('\''));
    
    match var_name {
        Some(var) => Some((var, my_str)),
        None => None,
    }
}

fn copy(lex: &mut Lexer<LbToken>) -> Option<(char, char)> {
    let token = lex.slice();
    let var_name_1 = token.chars().nth(1);
    let var_name_2 = token.chars().nth(2);
    if let None = var_name_1 {
        return None;
    }
    if let None = var_name_2 {
        return None;
    }
    Some((var_name_1.unwrap(), var_name_2.unwrap()))
}

fn single_var_arg(lex: &mut Lexer<LbToken>) -> Option<char> {
    let token = lex.slice();
    return token.chars().nth(1);
}

fn print_str(lex: &mut Lexer<LbToken>) -> Option<String> {
    let token = lex.slice();
    let my_str = String::from(token[1..].trim_matches('\''));
    Some(my_str)
}

fn math_op(lex: &mut Lexer<LbToken>) -> Option<(char, char, char, char)> {
    let token = lex.slice();
    let valid_ops = "ASMDEGLR";
    let args: Vec<char> = token[1..].chars().collect();
    // must have exactly one op and three vars
    if args.len() != 4 {
        return None;
    }
    // op must be valid
    if !valid_ops.contains(args[0]) {
        return None;
    }
    Some((args[0], args[1], args[2], args[3]))
}

fn bool_op(lex: &mut Lexer<LbToken>) -> Option<(char, char, char, char)> {
    let token = lex.slice();
    let valid_ops = "EAOX";
    let args: Vec<char> = token[1..].chars().collect();
    // must have exactly one op and three vars
    if args.len() != 4 {
        return None;
    }
    // op must be valid
    if !valid_ops.contains(args[0]) {
        return None;
    }
    Some((args[0], args[1], args[2], args[3]))
}

fn base_loop(lex: &mut Lexer<LbToken>) -> Option<(char, Box<LbToken>)> {
    let token = lex.slice();
    if let Some(condition) = token.chars().nth(1) {
        let cmd_string: String = token[2..].chars().collect();
        // must provide SOME subcommand
        if cmd_string.len() <= 0 {
            return None;
        }
        let cmd = lex_sub(cmd_string);
        return match cmd {
            Some(subcommand) => Some((condition, Box::new(subcommand))),
            None => None,
        };
    }
    None
}

fn execute_var(lex: &mut Lexer<LbToken>) -> Option<(char, String)> {
    let token = lex.slice();
    if let Some(fn_var) = token.chars().nth(1) {
        let args: String = token[2..].chars().collect();
        return Some((fn_var, args));
    }
    None
}

fn get_input(lex: &mut Lexer<LbToken>) -> Option<(char, char, f64)> {
    let token = lex.slice();
    let valid_ops = "NS";
    let op = token.chars().nth(1).unwrap();
    let var = token.chars().nth(2).unwrap();
    // op must be valid
    if !valid_ops.contains(op) {
        return None;
    }
    let num = token[3..].parse::<f64>().ok();
    if let None = num {
        return None;
    }
    Some((op, var, num.unwrap()))
}

// Utilities

/// Opens a new lexer to lex a subcommand.
/// The subcommand comes in as a string.
fn lex_sub(sub: String) -> Option<LbToken> {
    let mut lex = LbToken::lexer(&sub);
    return lex.next();
}

#[test]
fn tokens_parse_correctly() {
    let mut lex = LbToken::lexer("Sa4.4 Cab P'hello world' Pa i ! This is a comment".trim());
    assert_eq!(lex.next(), Some(LbToken::SaveNumber(('a', 4.4))));
    assert_eq!(lex.slice(), "Sa4.4");
    assert_eq!(lex.next(), Some(LbToken::Copy(('a', 'b'))));
    assert_eq!(lex.slice(), "Cab");
    assert_eq!(lex.next(), Some(LbToken::PrintStr(String::from("hello world"))));
    assert_eq!(lex.slice(), "P'hello world'");
    assert_eq!(lex.next(), Some(LbToken::PrintVar('a')));
    assert_eq!(lex.slice(), "Pa");
    assert_eq!(lex.next(), Some(LbToken::Error)); 
    assert_eq!(lex.slice(), "i");
    assert_eq!(lex.next(), None);
}

#[test]
fn advanced_tokens() {
    let mut lex = LbToken::lexer("MAbcd RA WaIcXzabcd !comment here".trim());
    assert_eq!(lex.next(), Some(LbToken::MathOp(('A', 'b', 'c', 'd'))));
    assert_eq!(lex.slice(), "MAbcd");
    assert_eq!(lex.next(), Some(LbToken::ResetAll));
    assert_eq!(lex.slice(), "RA");
    assert_eq!(lex.next(), Some(
        LbToken::WhileLoop(('a', Box::new(
            LbToken::IfStatement(('c', Box::new(
                LbToken::Execute(('z', String::from("abcd")))
            )))
        )))
    ));
    assert_eq!(lex.slice(), "WaIcXzabcd");
    assert_eq!(lex.next(), None);
}

#[test]
fn multi_line_comments() {
    let mut lex = LbToken::lexer("! This program prints out n fibonacci numbers.
! Works for any number n = 0 or greater.
! Input
Sn0 ! GNn0

! variables
Sa0 Sb1".trim());
    assert_eq!(lex.next(), Some(LbToken::SaveNumber(('n', 0.0))));
    assert_eq!(lex.slice(), "Sn0");
    assert_eq!(lex.next(), Some(LbToken::SaveNumber(('a', 0.0))));
    assert_eq!(lex.slice(), "Sa0");
    assert_eq!(lex.next(), Some(LbToken::SaveNumber(('b', 1.0))));
    assert_eq!(lex.slice(), "Sb1");
    assert_eq!(lex.next(), None);
}


