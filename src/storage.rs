use std::collections::HashMap;
use crate::program::Val;

const VALID_VARS: &str = "abcdefghijklmnopqrstuvwxyz";

/// Returns true if and only if the given character is a valid
/// name for a [LbStorage] variable.
pub fn is_var(c: &char) -> bool {
    VALID_VARS.contains(*c)
}


/// A data storage struct that can be operated upon by a [crate::program::LbProgram].
/// Represents a bank of 26 variables, one for each lowercase letter
/// of the alphabet. Each variable can store a single number (an f64) or String.
pub struct LbStorage {
    data: HashMap<char, Val>,
}

impl LbStorage {
    /// Returns a new [LbStorage] instance. It can be used by one or more LbPrograms.
    pub fn new() -> LbStorage {
        let store = LbStorage {
            data: HashMap::new(),
        };
        return store;
    }

    /// Returns a new [LbStorage] instance that takes ownership of the given HashMap
    /// and uses it as its own data.
    pub fn from_hashmap(existing_data: HashMap<char, Val>) -> LbStorage {
        let store = LbStorage {
            data: existing_data.to_owned(),
        };
        return store;
    }

    /// Gets the value stored under the given variable name.
    /// If the name is invalid, returns `None`.
    /// If nothing has been stored under the valid name, returns the default value of `0`.
    pub fn get_var(&mut self, var_name: char) -> Option<&Val> {
        if !is_var(&var_name) {
            return None;
        }
        let val = self.data.entry(var_name)
            .or_insert(Val::zero());
        Some(val)
    }

    /// Store a value under the given variable name.
    /// Returns `Ok(())` if the value has been stored.
    pub fn set_var(&mut self, var_name: char, new_value: &Val) -> Result<(), String> {
        self.data.insert(var_name, (*new_value).clone());
        Ok(())
    }

    /// Resets the value under the given name to the default value of `0`.
    pub fn reset_var(&mut self, var_name: char) -> Result<(), String> {
        self.data.remove(&var_name);
        Ok(())
    }

    /// Resets ALL variables to `0`. Thw resulting storage is equivalent to `LbStorage::new()`.
    pub fn reset_all(&mut self) -> Result<(), String> {
        self.data.clear();
        Ok(())
    }

    /// Copies a value from one variable to another.
    /// Does not affect the original value.
    /// Returns `Ok(())` if the value was cloned successfully.
    pub fn copy(&mut self, from_var: char, to_var: char) -> Result<(), String> {
        let x = self.get_var(from_var).expect("Couldn't find variable");
        let y = (*x).clone();
        self.set_var(to_var, &y)
    }

    /// Returns `Some(false)` if and only if the value under the given name is 0.
    /// Otherwise, returns `Some(true)`.
    /// If the given variable name is invalid, returns `None`.
    pub fn var_as_bool(&mut self, var_name: char) -> Option<bool> {
        let x = self.get_var(var_name).expect("Couldn't find variable");
        return match x {
            Val::Number(n) => Some(*n != 0.0),
            Val::Text(_) => Some(true),
        };
    }

    pub fn to_hashmap(&self) -> HashMap<char, Val> {
        self.data.clone()
    }
}