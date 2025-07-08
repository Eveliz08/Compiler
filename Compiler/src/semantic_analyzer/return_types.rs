use std::collections::HashMap;
use crate::ast_nodes::type_def::TypeDefNode;

/// Holds information about a function signature, including its name,
/// the list of argument names and their types, and the return type.
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionInfo {
    /// The function's name.
    pub name: String,

    /// A vector of tuples where each tuple contains an argument name and its type as strings.
    pub arguments_types: Vec<(String, String)>,

    /// The function's return type as a string.
    pub return_type: String,
}

impl FunctionInfo {
    /// Creates a new `FunctionInfo` instance with the given name, argument types, and return type.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the function.
    /// * `arguments_types` - A vector of `(argument_name, argument_type)` pairs.
    /// * `return_type` - The return type of the function.
    ///
    /// # Returns
    ///
    /// A new `FunctionInfo` struct.
    pub fn new(name: String, arguments_types: Vec<(String, String)>, return_type: String) -> Self {
        FunctionInfo {
            name,
            arguments_types,
            return_type,
        }
    }
}

/// Context that stores semantic information during analysis,
/// such as symbol table, declared functions and types,
/// and the currently analyzed type and function.
#[derive(Debug, Clone, PartialEq)]
pub struct SemanticContext {
    /// Maps variable names to their respective type names.
    pub symbols: HashMap<String, String>,

    /// Maps function names to their `FunctionInfo`.
    pub declared_functions: HashMap<String, FunctionInfo>,

    /// Maps type names to their corresponding type definition nodes.
    pub declared_types: HashMap<String, TypeDefNode>,

    /// Optionally holds the name of the current type being analyzed.
    pub current_type: Option<String>,

    /// Optionally holds the name of the current function being analyzed.
    pub current_function: Option<String>,
}
