//! Contains the definition of `TypeTree`, a hierarchical representation of types in the language,
//! along with utilities for inheritance, type checking, and method resolution.

use std::collections::HashMap;
use crate::ast_nodes::function_def::{FunctionDefNode, FunctionParams};

use super::tree_node::TypeNode;

/// Represents a set of built-in types available in the language.
pub enum BuiltInTypes {
    Object,
    String,
    Number,
    Boolean,
    Unknown,
}

impl BuiltInTypes {
    /// Returns the string representation of the built-in type.
    pub fn as_str(&self) -> &str {
        match self {
            BuiltInTypes::Object => "Object",
            BuiltInTypes::String => "String",
            BuiltInTypes::Number => "Number",
            BuiltInTypes::Boolean => "Boolean",
            BuiltInTypes::Unknown => "Unknown",
        }
    }
}

/// Represents the tree structure of all types (user-defined and built-in).
/// 
/// Each node corresponds to a type and maintains its position in the inheritance hierarchy.
pub struct TypeTree {
    /// Root type node (`Object`) from which all others inherit by default.
    pub root: TypeNode,
    /// A map of all type names to their corresponding `TypeNode`.
    pub nodes: HashMap<String, TypeNode>,
}

impl TypeTree {
    /// Creates a new `TypeTree` with built-in types initialized.
    pub fn new() -> Self {
        let mut newtree = TypeTree {
            root: TypeNode::new("Object".to_string(), vec![], 0, None, Vec::new(), HashMap::new(), HashMap::new()),
            nodes: HashMap::new(),
        };
        newtree.nodes.insert("Object".to_string(), newtree.root.clone());
        newtree.add_type("String".to_string(), vec![], None, HashMap::new(), HashMap::new());
        newtree.add_type("Number".to_string(), vec![], None, HashMap::new(), HashMap::new());
        newtree.add_type("Boolean".to_string(), vec![], None, HashMap::new(), HashMap::new());
        newtree.add_type("Unknown".to_string(), vec![], None, HashMap::new(), HashMap::new());
        newtree
    }

    /// Adds a new type node to the tree.
    ///
    /// # Arguments
    /// * `type_name` - The name of the new type.
    /// * `params` - Constructor or type parameters.
    /// * `parent_name` - Optional parent type name (for inheritance).
    /// * `variables` - Instance variables for the type.
    /// * `methods` - Methods defined in the type.
    pub fn add_type(
        &mut self,
        type_name: String,
        params: Vec<FunctionParams>,
        parent_name: Option<String>,
        variables: HashMap<String, Box<String>>,
        methods: HashMap<String, Box<FunctionDefNode>>,
    ) {
        match &parent_name {
            Some(name) => {
                if let Some(parent) = self.nodes.get_mut(name) {
                    let new_node = TypeNode::new(type_name.clone(), params, 0, Some(name.clone()), Vec::new(), variables, methods);
                    parent.add_child(type_name.clone());
                    self.nodes.insert(type_name.clone(), new_node);
                }
            }
            None => {
                let new_node = TypeNode::new(type_name.clone(), params, 0, None, Vec::new(), variables, methods);
                self.root.add_child(new_node.type_name.clone());
                self.nodes.insert(type_name.clone(), new_node);
            }
        }
    }

    /// Retrieves a type node by name.
    pub fn get_type(&self, type_name: &str) -> Option<TypeNode> {
        self.nodes.get(type_name).cloned()
    }

    /// Finds the Lowest Common Ancestor (LCA) of two types in the hierarchy.
    pub fn find_lca(&self, type1: &TypeNode, type2: &TypeNode) -> TypeNode {
        if type1.type_name == type2.type_name {
            return type1.clone();
        }
        if type1.depth < type2.depth {
            if let Some(ref parent2) = type2.parent {
                if let Some(ref parent2_node) = self.nodes.get(parent2) {
                    return self.find_lca(type1, parent2_node);
                } else {
                    return self.root.clone();
                }
            } else {
                return self.root.clone();
            }
        } else if type2.depth < type1.depth {
            if let Some(ref parent1) = type1.parent {
                if let Some(ref parent1_node) = self.nodes.get(parent1) {
                    return self.find_lca(parent1_node, type2);
                } else {
                    return self.root.clone();
                }
            } else {
                return self.root.clone();
            }
        } else {
            if let (Some(parent1), Some(parent2)) = (&type1.parent, &type2.parent) {
                if let (Some(parent1_node), Some(parent2_node)) = (self.nodes.get(parent1), self.nodes.get(parent2)) {
                    return self.find_lca(parent1_node, parent2_node);
                } else {
                    return self.root.clone();
                }
            } else {
                return self.root.clone();
            }
        }
    }

    /// Checks if `ancestor` is a true ancestor (direct or indirect) of `descendant`.
    pub fn is_ancestor(&self, ancestor: &TypeNode, descendant: &TypeNode) -> bool {
        let mut current = Some(descendant);
        while let Some(node) = current {
            if node.type_name == ancestor.type_name {
                return true;
            }
            if let Some(parent_name) = &node.parent {
                current = self.nodes.get(parent_name);
            } else {
                current = None;
            }
        }
        false
    }

    /// Detects any cycle in the inheritance graph.
    ///
    /// # Returns
    /// * `Some(type_name)` if a cycle is found starting from that type.
    /// * `None` if the tree is acyclic.
    pub fn check_cicle(&mut self) -> Option<String> {
        let mut visited = HashMap::new();
        for (type_name, _) in self.nodes.clone() {
            if !visited.contains_key(&type_name) {
                if let Some(cycle_node) = self.check_cicle_helper(type_name, &mut visited) {
                    return Some(cycle_node);
                }
            }
        }
        None
    }

    /// Helper method for cycle detection using DFS traversal.
    fn check_cicle_helper(
        &mut self,
        node_name: String,
        visited: &mut HashMap<String, bool>,
    ) -> Option<String> {
        if visited.contains_key(&node_name) {
            return Some(node_name);
        }
        if let Some(node) = self.nodes.get_mut(&node_name) {
            visited.insert(node_name.clone(), true);
            let parent_depth = node.depth;
            let children = node.children.clone();
            for child in &children {
                if let Some(child_node) = self.nodes.get_mut(child) {
                    child_node.depth = parent_depth + 1;
                }
                if let Some(cycle_node) = self.check_cicle_helper(child.clone(), visited) {
                    return Some(cycle_node);
                }
            }
            visited.remove(&node_name);
        }
        None
    }

    /// Recursively searches for a method named `method_name` in `node_name` and its ancestors.
    ///
    /// # Returns
    /// * `Some(FunctionDefNode)` if found.
    /// * `None` if not found in the entire ancestry.
    pub fn find_method(&mut self, node_name: String, method_name: String) -> Option<Box<FunctionDefNode>> {
        if let Some(type_node) = self.nodes.get_mut(&node_name) {
            if let Some(method) = type_node.get_method(&method_name) {
                return Some(method);
            } else if let Some(parent) = type_node.parent.clone() {
                return self.find_method(parent, method_name);
            }
        }
        None
    }
}
