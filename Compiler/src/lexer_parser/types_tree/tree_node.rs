//! Defines the `TypeNode` struct used to represent user-defined types in the semantic layer.
//!
//! Each `TypeNode` contains metadata such as its name, parameters, inheritance hierarchy,
//! attributes (variables), and methods.

use std::collections::HashMap;
use crate::ast_nodes::function_def::{FunctionDefNode, FunctionParams};

/// Represents a user-defined type (class-like structure) in the language.
///
/// A `TypeNode` contains all relevant metadata for a type: its name, inheritance info,
/// parameters (for generics or constructors), variables, and methods.
#[derive(Debug, Clone, PartialEq)]
pub struct TypeNode {
    /// The name of the type.
    pub type_name: String,
    /// The depth in the inheritance tree (0 = root).
    pub depth: i32,
    /// Parameters used by the type (e.g., generic parameters or constructor arguments).
    pub params: Vec<FunctionParams>,
    /// The name of the parent type (if any).
    pub parent: Option<String>,
    /// Names of all child types.
    pub children: Vec<String>,
    /// Map of variable names to their types (as strings).
    pub variables: HashMap<String, Box<String>>,
    /// Map of method names to their corresponding function definitions.
    pub methods: HashMap<String, Box<FunctionDefNode>>,
}

impl TypeNode {
    /// Constructs a new `TypeNode` instance.
    ///
    /// # Arguments
    /// * `type_name` - Name of the type.
    /// * `params` - Parameters or generic arguments of the type.
    /// * `depth` - Depth in the inheritance tree.
    /// * `parent` - Optional name of the parent type.
    /// * `children` - List of names of the child types.
    /// * `variables` - Map of variable names to type names.
    /// * `methods` - Map of method names to function definitions.
    pub fn new(
        type_name: String,
        params: Vec<FunctionParams>,
        depth: i32,
        parent: Option<String>,
        children: Vec<String>,
        variables: HashMap<String, Box<String>>,
        methods: HashMap<String, Box<FunctionDefNode>>,
    ) -> Self {
        TypeNode {
            type_name,
            params,
            depth,
            parent,
            children,
            variables,
            methods,
        }
    }

    /// Adds a new child type by name.
    pub fn add_child(&mut self, child_name: String) {
        self.children.push(child_name);
    }

    /// Sets the parent type by name.
    pub fn set_parent(&mut self, parent_name: String) {
        self.parent = Some(parent_name);
    }

    /// Adds a new variable to the type.
    ///
    /// # Arguments
    /// * `name` - The variable's name.
    /// * `variable` - The type of the variable.
    pub fn add_variable(&mut self, name: String, variable: Box<String>) {
        self.variables.insert(name, variable);
    }

    /// Adds a new method to the type.
    ///
    /// # Arguments
    /// * `name` - The method's name.
    /// * `method` - The function node representing the method.
    pub fn add_method(&mut self, name: String, method: Box<FunctionDefNode>) {
        self.methods.insert(name, method);
    }

    /// Retrieves a method by name, if it exists.
    ///
    /// # Arguments
    /// * `method_name` - The name of the method to retrieve.
    ///
    /// # Returns
    /// An `Option` containing the method if found.
    pub fn get_method(&mut self, method_name: &String) -> Option<Box<FunctionDefNode>> {
        self.methods.get(method_name).cloned()
    }
}
