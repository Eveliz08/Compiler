use super::return_types::{FunctionInfo, SemanticContext};
use super::semantic_errors::SemanticError;
use crate::ast_nodes::binary_op::BinaryOpNode;
use crate::ast_nodes::block::BlockNode;
use crate::ast_nodes::destructive_assign::DestructiveAssignNode;
use crate::ast_nodes::expression::Expression;
use crate::ast_nodes::for_loop::ForNode;
use crate::ast_nodes::function_call::FunctionCallNode;
use crate::ast_nodes::function_def::FunctionDefNode;
use crate::ast_nodes::if_else::IfElseNode;
use crate::ast_nodes::let_in::LetInNode;
use crate::ast_nodes::literals::{
    BooleanLiteralNode, IdentifierNode, NumberLiteralNode, StringLiteralNode,
};
use crate::ast_nodes::print::PrintNode;
use crate::ast_nodes::program::{Program, Statement};
use crate::ast_nodes::type_def::{TypeDefNode, TypeMember};
use crate::ast_nodes::type_instance::TypeInstanceNode;
use crate::ast_nodes::type_member_access::{TypeFunctionAccessNode, TypePropAccessNode};
use crate::ast_nodes::unary_op::UnaryOpNode;
use crate::ast_nodes::while_loop::WhileNode;
use crate::tokens::{OperatorToken, Span};
use crate::types_tree::tree_node::TypeNode;
use crate::types_tree::types_tree::{BuiltInTypes, TypeTree};
use crate::visitor::accept::Accept;
use crate::visitor::visitor_trait::Visitor;
use std::collections::HashMap;

/// SemanticAnalyzer performs semantic analysis of the AST,
/// validating types, functions, variables, and type hierarchy.
/// It maintains semantic context, scope stack, errors, and the type tree.
pub struct SemanticAnalyzer {
    /// Current context containing declared symbols, functions, and types.
    context: SemanticContext,
    /// Stack of contexts to handle nested scopes.
    scopes: Vec<SemanticContext>,
    /// List of semantic errors found during analysis.
    errors: Vec<SemanticError>,
    /// Tree representing the hierarchy and relations of types.
    types_tree: TypeTree,
}

impl SemanticAnalyzer {
 /// Creates a new SemanticAnalyzer with predefined basic types and constants (PI, E).
    pub fn new() -> Self {
        let mut s_a = Self {
            context: SemanticContext {
                symbols: HashMap::new(),
                declared_functions: HashMap::new(),
                declared_types: HashMap::new(),
                current_type: None,
                current_function: None,
            },
            scopes: Vec::new(),
            errors: Vec::new(),
            types_tree: TypeTree::new(),
        };
        s_a.context
            .symbols
            .insert("PI".to_string(), "Number".to_string());
        s_a.context
            .symbols
            .insert("E".to_string(), "Number".to_string());
        s_a
    }
     /// Enters a new scope by pushing the current context onto the stack.
    fn enter_scope(&mut self) {
        self.scopes.push(self.context.clone());
    }
    /// Exits the current scope, restoring the previous context from the stack.
    fn exit_scope(&mut self) {
        self.context = self.scopes.pop().unwrap();
    }
    /// Adds a new semantic error to the list of errors.
    fn new_error(&mut self, error: SemanticError) {
        self.errors.push(error);
    }
    /// Analyzes a complete program by collecting type and function definitions,
    /// building the type tree, and analyzing each statement.
    ///
    /// Returns Ok(()) if no errors found, or Err with the list of semantic errors.
    pub fn analyze(&mut self, node: &mut Program) -> Result<(), Vec<SemanticError>> {
        self.get_types_definitions(node);
        self.build_types();
        self.get_functions_names_and_signatures(node);
        for statement in &mut node.statements {
            statement.accept(self);
        }
        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(self.errors.clone())
        }
    }

    /// Retrieves a built-in type node based on the BuiltInTypes enum.
    pub fn get_built_in_types(&self, built_in: &BuiltInTypes) -> TypeNode {
        self.types_tree.get_type(built_in.as_str()).unwrap()
    }
    
    /// Collects function names and signatures declared in the program,
    /// validating parameter types and reporting redefinitions or undefined types.
    pub fn get_functions_names_and_signatures(&mut self, node: &mut Program) {
        for statement in &node.statements {
            match statement {
                Statement::StatementFunctionDef(node) => {
                    let func_return = node.return_type.clone();
                    let mut arg_types = Vec::new();
                    for param in &node.params {
                        if let Some(param_type) = self.types_tree.get_type(&param.signature) {
                            if let Some(_variable) =
                                arg_types.iter().find(|(name, _)| *name == param.name)
                            {
                                self.new_error(SemanticError::ParamNameAlreadyExist(
                                    param.name.clone(),
                                    node.name.clone(),
                                    "function".to_string(),
                                    param.span.clone(),
                                ));
                            } else {
                                arg_types.push((param.name.clone(), param_type.type_name.clone()));
                            }
                        } else {
                            self.new_error(SemanticError::UndefinedType(
                                param.signature.clone(),
                                param.span.clone(),
                            ));
                            arg_types.push((
                                param.name.clone(),
                                self.get_built_in_types(&BuiltInTypes::Unknown).type_name,
                            ));
                        }
                    }
                    if self.context.declared_functions.contains_key(&node.name) {
                        self.new_error(SemanticError::RedefinitionOfFunction(
                            node.name.clone(),
                            node.span.clone(),
                        ));
                    } else {
                        if let Some(func_type) = self.types_tree.get_type(&func_return) {
                            self.context.declared_functions.insert(
                                node.name.clone(),
                                FunctionInfo::new(
                                    node.name.clone(),
                                    arg_types.clone(),
                                    func_type.type_name,
                                ),
                            );
                        } else {
                            self.context.declared_functions.insert(
                                node.name.clone(),
                                FunctionInfo::new(
                                    node.name.clone(),
                                    arg_types.clone(),
                                    self.get_built_in_types(&BuiltInTypes::Unknown).type_name,
                                ),
                            );
                        }
                    }
                }
                _ => continue,
            }
        }
    }

    /// Collects type definitions in the program,
    /// ensuring no redefinitions or direct inheritance cycles.
    pub fn get_types_definitions(&mut self, node: &mut Program) {
        for statement in &node.statements {
            match statement {
                Statement::StatementTypeDef(type_def) => {
                    if self.types_tree.get_type(&type_def.identifier).is_some()
                        || self
                            .context
                            .declared_types
                            .contains_key(&type_def.identifier)
                    {
                        self.new_error(SemanticError::RedefinitionOfType(
                            type_def.identifier.clone(),
                            type_def.span.clone(),
                        ));
                    } else if let Some(parent_type) = &type_def.parent {
                        if type_def.identifier == *parent_type {
                            self.new_error(SemanticError::UnknownError(
                                "Type cannot inherit from itself".to_string(),
                                type_def.span.clone(),
                            ));
                        }
                        self.context
                            .declared_types
                            .insert(type_def.identifier.clone(), *type_def.clone());
                    } else {
                        self.context
                            .declared_types
                            .insert(type_def.identifier.clone(), *type_def.clone());
                    }
                }
                _ => continue,
            }
        }
    }

    /// Builds the type tree from collected definitions,
    /// assigning methods, variables, parents, and children,
    /// and checking for inheritance cycles and related errors.
    pub fn build_types(&mut self) {
        for (type_name, type_def) in self.context.declared_types.clone() {
            let mut methods = HashMap::new();
            let mut variables = HashMap::new();
            
            // Collect methods and variables
            for m in &type_def.members {
                match m {
                    TypeMember::Method(method) => {
                        methods.insert(method.name.clone(), Box::new(method.clone()));
                    }
                    TypeMember::Property(prop) => {
                        variables.insert(
                            prop.identifier.clone(),
                            Box::new(
                                prop.node_type
                                    .as_ref()
                                    .map(|t| t.type_name.clone())
                                    .unwrap_or_else(|| "Unknown".to_string())
                            ),
                        );
                    }
                }
            }
            
            // Add type to type tree
            self.types_tree.add_type(
                type_name.clone(),
                type_def.params.clone(),
                None,
                variables,
                methods,
            );
        }
        
        for (type_name, type_def) in self.context.declared_types.clone() {
            if let Some(parent_name) = &type_def.parent {
                let parent_type_name = parent_name.clone();
                
                if !self.types_tree.nodes.contains_key(&parent_type_name) {
                    self.new_error(SemanticError::UndefinedType(
                        parent_type_name,
                        type_def.span,
                    ));
                } else {
                    // Add child relationship
                    {
                        let parent_node = self.types_tree.nodes.get_mut(&parent_type_name).unwrap();
                        parent_node.add_child(type_name.clone());
                    }
                    
                    // Set parent for child
                    {
                        let child_node = self.types_tree.nodes.get_mut(&type_name).unwrap();
                        child_node.set_parent(parent_type_name.clone());
                    }
                    
                    // Check argument count match
                    let parent_node_params_len = {
                        let parent_node = self.types_tree.nodes.get(&parent_type_name).unwrap();
                        parent_node.params.len()
                    };
                    if type_def.parent_args.len() != parent_node_params_len {
                        self.new_error(SemanticError::InvalidTypeArgumentCount(
                            type_def.parent_args.len(),
                            parent_node_params_len,
                            parent_type_name,
                            type_def.span,
                        ));
                    }
                }
            }
        }
        
        // Check for cycles
        if let Some(cycle_node) = self.types_tree.check_cicle() {
            // Get span from type definition
            let span = self.context.declared_types
                .get(&cycle_node)
                .map(|td| td.span)
                .unwrap_or_else(|| Span::new(0, 0));
            
            self.new_error(SemanticError::CycleDetected(
                cycle_node,
                span,
            ));
        }
    }
}

impl Visitor<TypeNode> for SemanticAnalyzer {
    /// Visits a `for` loop node, checking types of loop variables and range expressions,
    /// and analyzes the loop body within a new scope.
    fn visit_for_loop(&mut self, node: &mut ForNode) -> TypeNode {
        self.enter_scope();
        self.context
            .symbols
            .insert(node.variable.clone(), "Number".to_string());
        let start_type = node.start.accept(self);
        let end_type = node.end.accept(self);
        if start_type.type_name != "Number" {
            self.new_error(SemanticError::InvalidTypeArgument(
                "for loop".to_string(),
                start_type.type_name.clone(),
                "Number".to_string(),
                0,
                "range".to_string(),
                node.span.clone(),
            ));
        }
        if end_type.type_name != "Number" {
            self.new_error(SemanticError::InvalidTypeArgument(
                "for loop".to_string(),
                end_type.type_name.clone(),
                "Number".to_string(),
                1,
                "range".to_string(),
                node.span.clone(),
            ));
        }
        let return_type = node.body.accept(self);
        self.exit_scope();
        node.set_type(return_type.clone());
        return_type
    }

    /// Visits a destructive assignment node, validating that assignment
    /// targets are identifiers or type property accesses and updating context accordingly.
    fn visit_destructive_assign(&mut self, node: &mut DestructiveAssignNode) -> TypeNode {
        match *node.identifier.clone() {
            Expression::Identifier(ref id) => {
                if self.context.symbols.contains_key(&id.value) {
                    let new_type = node.expression.accept(self);
                    self.context
                        .symbols
                        .insert(id.value.clone(), new_type.type_name.clone());
                    node.set_type(new_type.clone());
                    new_type
                } else {
                    self.new_error(SemanticError::UndefinedIdentifier(
                        id.value.clone(),
                        id.span.clone(),
                    ));
                    self.get_built_in_types(&BuiltInTypes::Unknown)
                }
            }
            Expression::TypePropAccess(ref mut access_node) => {
                let mut object_type = access_node.object.accept(self);
                if let Some(_property_type) =
                    object_type.variables.get_mut(access_node.member.as_ref())
                {
                    let new_type = node.expression.accept(self);
                    object_type.variables.insert(
                        access_node.member.as_ref().clone(),
                        Box::new(new_type.type_name.clone()),
                    );
                    node.set_type(new_type.clone());
                    new_type
                } else {
                    self.new_error(SemanticError::InvalidTypePropertyAccess(
                        object_type.type_name.clone(),
                        access_node.member.as_ref().clone(),
                        access_node.span.clone(),
                    ));
                    self.get_built_in_types(&BuiltInTypes::Unknown)
                }
            }
            _ => {
                self.new_error(SemanticError::UnknownError("Destructive assignment can only be done to an identifier or type property access".to_string(), node.span.clone()));
                self.get_built_in_types(&BuiltInTypes::Unknown)
            }
        }
    }

    /// Visits a function definition node, entering a new scope and
    /// populating symbols with function parameters. Checks for
    /// undeclared functions and verifies the return type compatibility.
    /// Returns the function's return type node.
    fn visit_function_def(&mut self, node: &mut FunctionDefNode) -> TypeNode {
        self.enter_scope();
        self.context.current_function = Some(node.name.clone());
        if let Some(function) = self.context.declared_functions.get(&node.name) {
            for param in &function.arguments_types {
                self.context
                    .symbols
                    .insert(param.0.clone(), param.1.clone());
            }
        } else if let Some(current_type) = self.context.current_type.clone() {
            if let Some(type_node) = self.types_tree.get_type(&current_type) {
                if let Some(function) = type_node.methods.get(&node.name) {
                    for param in &function.params.clone() {
                        self.context
                            .symbols
                            .insert(param.name.clone(), param.signature.clone());
                    }
                } else {
                    self.new_error(SemanticError::UndeclaredFunction(
                        node.name.clone(),
                        node.span.clone(),
                    ));
                }
            } else {
                self.new_error(SemanticError::UndefinedType(
                    current_type,
                    node.span.clone(),
                ));
            }
        } else {
            self.new_error(SemanticError::UndeclaredFunction(
                node.name.clone(),
                node.span.clone(),
            ));
        }
        let body_type = node.body.accept(self);
        let mut return_type_node = self.get_built_in_types(&BuiltInTypes::Unknown);
        if let Some(func_type) = self.types_tree.get_type(&node.return_type.clone()) {
            if !self.types_tree.is_ancestor(&func_type, &body_type) {
                self.new_error(SemanticError::InvalidFunctionReturn(
                    body_type,
                    func_type.clone(),
                    node.name.clone(),
                    node.span.clone(),
                ));
            }
            return_type_node = func_type;
        } else {
            self.new_error(SemanticError::UndefinedType(
                node.return_type.clone(),
                node.span.clone(),
            ));
        }
        self.exit_scope();
        node.set_type(return_type_node.clone());
        return_type_node
    }

    /// Visits a numeric literal node, setting and returning the built-in Number type.
    fn visit_literal_number(&mut self, node: &mut NumberLiteralNode) -> TypeNode {
        node.set_type(self.get_built_in_types(&BuiltInTypes::Number));
        self.get_built_in_types(&BuiltInTypes::Number)
    }

    /// Visits a boolean literal node, setting and returning the built-in Boolean type.
    fn visit_literal_boolean(&mut self, node: &mut BooleanLiteralNode) -> TypeNode {
        node.set_type(self.get_built_in_types(&BuiltInTypes::Boolean));
        self.get_built_in_types(&BuiltInTypes::Boolean)
    }

    /// Visits a string literal node, setting and returning the built-in String type.
    fn visit_literal_string(&mut self, node: &mut StringLiteralNode) -> TypeNode {
        node.set_type(self.get_built_in_types(&BuiltInTypes::String));
        self.get_built_in_types(&BuiltInTypes::String)
    }

    /// Visits an identifier node, resolving its type from symbols or current type context.
    /// Reports errors if the identifier or its type is undefined.
    /// Returns the resolved type node or Unknown type if unresolved.
    fn visit_identifier(&mut self, node: &mut IdentifierNode) -> TypeNode {
        if let Some(return_type) = self.context.symbols.get(&node.value) {
            if let Some(node_type) = self.types_tree.get_type(&return_type) {
                node.set_type(node_type.clone());
                node_type.clone()
            } else {
                self.new_error(SemanticError::UndefinedType(
                    return_type.clone(),
                    node.span.clone(),
                ));
                self.get_built_in_types(&BuiltInTypes::Unknown)
            }
        } else if node.value == "self" {
            if let Some(current_type) = &self.context.current_type {
                if let Some(type_node) = self.types_tree.get_type(current_type) {
                    node.set_type(type_node.clone());
                    type_node.clone()
                } else {
                    self.new_error(SemanticError::UndefinedType(
                        current_type.clone(),
                        node.span.clone(),
                    ));
                    self.get_built_in_types(&BuiltInTypes::Unknown)
                }
            } else {
                self.new_error(SemanticError::UndefinedIdentifier(
                    node.value.clone(),
                    node.span.clone(),
                ));
                self.get_built_in_types(&BuiltInTypes::Unknown)
            }
        } else {
            self.new_error(SemanticError::UndefinedIdentifier(
                node.value.clone(),
                node.span.clone(),
            ));
            self.get_built_in_types(&BuiltInTypes::Unknown)
        }
    }

    /// Visits a function call node, checking argument types against declared function signatures.
    /// Supports special handling for `base` calls in type hierarchies.
    /// Reports errors for undeclared functions, invalid argument count, or mismatched types.
    /// Returns the function's return type node.
    fn visit_function_call(&mut self, node: &mut FunctionCallNode) -> TypeNode {
        let mut arg_types: Vec<TypeNode> = Vec::new();
        for arg in node.arguments.iter_mut() {
            arg_types.push(arg.accept(self));
        }
        if self.context.current_type.is_some() && node.function_name == "base" {
            if let Some(current_type) = self.context.current_type.clone() {
                if let Some(type_node) = self.types_tree.get_type(&current_type) {
                    if let Some(parent) = type_node.parent {
                        if let Some(current_function) = self.context.current_function.clone() {
                            if let Some(func) = self
                                .types_tree
                                .find_method(parent, current_function.clone())
                            {
                                if node.arguments.len() != func.params.len() {
                                    self.new_error(SemanticError::InvalidArgumentsCount(
                                        node.arguments.len(),
                                        func.params.len(),
                                        current_function.clone(),
                                        node.span.clone(),
                                    ));
                                } else {
                                    for (index, arg) in arg_types.iter_mut().enumerate() {
                                        if arg.type_name != func.params[index].signature {
                                            self.new_error(SemanticError::InvalidTypeArgument(
                                                "function".to_string(),
                                                arg.type_name.clone(),
                                                func.params[index].signature.clone(),
                                                index,
                                                func.name.clone(),
                                                node.span.clone(),
                                            ));
                                        }
                                    }
                                }
                                if let Some(func_type_node) =
                                    self.types_tree.get_type(&func.return_type)
                                {
                                    node.set_type(func_type_node.clone());
                                    return func_type_node.clone();
                                } else {
                                    self.new_error(SemanticError::UndefinedType(
                                        func.return_type.clone(),
                                        node.span.clone(),
                                    ));
                                    return self.get_built_in_types(&BuiltInTypes::Unknown);
                                }
                            }
                        }
                    }
                }
            }
        }
        if let Some(func_info) = self.context.declared_functions.get(&node.function_name) {
            let arguments_types = func_info.arguments_types.clone();
            let func_name = func_info.name.clone();
            let func_type = func_info.return_type.clone();
            if node.arguments.len() != arguments_types.len() {
                self.new_error(SemanticError::InvalidArgumentsCount(
                    node.arguments.len(),
                    arguments_types.len(),
                    node.function_name.clone(),
                    node.span.clone(),
                ));
            } else {
                for (index, arg) in arg_types.iter_mut().enumerate() {
                    let func_arg_type_node = self.types_tree.get_type(&arguments_types[index].1);
                    let arg_type_node = self.types_tree.get_type(&arg.type_name);
                    if !(func_arg_type_node.is_some() && arg_type_node.is_some()
                        && self.types_tree.is_ancestor(func_arg_type_node.as_ref().unwrap(), arg_type_node.as_ref().unwrap()))
                    {
                        self.new_error(SemanticError::InvalidTypeArgument(
                            "function".to_string(),
                            arg.type_name.clone(),
                            arguments_types[index].1.clone(),
                            index,
                            func_name.clone(),
                            node.span.clone(),
                        ));
                    }
                }
            }
            if let Some(func_type_node) = self.types_tree.get_type(&func_type) {
                node.set_type(func_type_node.clone());
                func_type_node
            } else {
                self.new_error(SemanticError::UndefinedType(
                    func_type.clone(),
                    node.span.clone(),
                ));
                self.get_built_in_types(&BuiltInTypes::Unknown)
            }
        } else {
            self.new_error(SemanticError::UndeclaredFunction(
                node.function_name.clone(),
                node.span.clone(),
            ));
            self.get_built_in_types(&BuiltInTypes::Unknown)
        }
    }

    /// Visits a while loop node, checking that the condition type is Boolean,
    /// and returns the type of the loop body.
    fn visit_while_loop(&mut self, node: &mut WhileNode) -> TypeNode {
        let condition_type = node.condition.accept(self);
        if condition_type != self.get_built_in_types(&BuiltInTypes::Boolean) {
            self.new_error(SemanticError::InvalidConditionType(
                condition_type,
                node.span.clone(),
            ));
        }
        let body_type = node.body.accept(self);
        node.set_type(body_type.clone());
        return body_type;
    }

     /// Visits a code block node, entering a new scope for the block,
    /// visiting each expression in sequence and returning the type of the last expression.
    fn visit_code_block(&mut self, node: &mut BlockNode) -> TypeNode {
        self.enter_scope();
        let mut last_type = self.get_built_in_types(&BuiltInTypes::Unknown);
        for expr in node.expression_list.expressions.iter_mut() {
            last_type = expr.accept(self);
        }
        self.exit_scope();
        node.set_type(last_type.clone());
        last_type
    }

    /// Visits a binary operation node, checking operand types and operator validity,
    /// setting and returning the resulting type or reporting errors.
    fn visit_binary_op(&mut self, node: &mut BinaryOpNode) -> TypeNode {
        let left_type = node.left.accept(self);
        let right_type = node.right.accept(self);

        match node.operator {
            OperatorToken::PLUS
            | OperatorToken::MINUS
            | OperatorToken::MUL
            | OperatorToken::DIV
            | OperatorToken::MOD
            | OperatorToken::POW => {
                if left_type == self.get_built_in_types(&BuiltInTypes::Number)
                    && right_type == self.get_built_in_types(&BuiltInTypes::Number)
                {
                    node.set_type(self.get_built_in_types(&BuiltInTypes::Number));
                    self.get_built_in_types(&BuiltInTypes::Number)
                } else {
                    self.new_error(SemanticError::InvalidBinaryOperation(
                        left_type,
                        right_type,
                        node.operator.clone(),
                        node.span.clone(),
                    ));
                    self.get_built_in_types(&BuiltInTypes::Unknown)
                }
            }

            OperatorToken::GT | OperatorToken::GTE | OperatorToken::LT | OperatorToken::LTE => {
                if left_type == self.get_built_in_types(&BuiltInTypes::Number)
                    && right_type == self.get_built_in_types(&BuiltInTypes::Number)
                {
                    node.set_type(self.get_built_in_types(&BuiltInTypes::Boolean));
                    self.get_built_in_types(&BuiltInTypes::Boolean)
                } else {
                    self.new_error(SemanticError::InvalidBinaryOperation(
                        left_type,
                        right_type,
                        node.operator.clone(),
                        node.span.clone(),
                    ));
                    self.get_built_in_types(&BuiltInTypes::Unknown)
                }
            }
            OperatorToken::NEQ | OperatorToken::EQ => {
                if left_type == self.get_built_in_types(&BuiltInTypes::String)
                    && right_type == self.get_built_in_types(&BuiltInTypes::String)
                    || left_type == self.get_built_in_types(&BuiltInTypes::Boolean)
                        && right_type == self.get_built_in_types(&BuiltInTypes::Boolean)
                    || left_type == self.get_built_in_types(&BuiltInTypes::Number)
                        && right_type == self.get_built_in_types(&BuiltInTypes::Number)
                {
                    node.set_type(self.get_built_in_types(&BuiltInTypes::Boolean));
                    self.get_built_in_types(&BuiltInTypes::Boolean)
                } else {
                    self.new_error(SemanticError::InvalidBinaryOperation(
                        left_type,
                        right_type,
                        node.operator.clone(),
                        node.span.clone(),
                    ));
                    self.get_built_in_types(&BuiltInTypes::Unknown)
                }
            }

            OperatorToken::CONCAT => {
                let string_type = self.get_built_in_types(&BuiltInTypes::String);
                if left_type == string_type && right_type == string_type {
                    node.set_type(string_type.clone());
                    string_type
                } else {
                    self.new_error(SemanticError::InvalidBinaryOperation(
                        left_type,
                        right_type,
                        node.operator.clone(),
                        node.span.clone(),
                    ));
                    self.get_built_in_types(&BuiltInTypes::Unknown)
                }
            }
            OperatorToken::AND | OperatorToken::OR => {
                if left_type == self.get_built_in_types(&BuiltInTypes::Boolean)
                    && right_type == self.get_built_in_types(&BuiltInTypes::Boolean)
                {
                    node.set_type(self.get_built_in_types(&BuiltInTypes::Boolean));
                    self.get_built_in_types(&BuiltInTypes::Boolean)
                } else {
                    self.new_error(SemanticError::InvalidBinaryOperation(
                        left_type,
                        right_type,
                        node.operator.clone(),
                        node.span.clone(),
                    ));
                    self.get_built_in_types(&BuiltInTypes::Unknown)
                }
            }
            _ => {
                self.new_error(SemanticError::UnknownError(
                    format!(
                        "Operator ( {} ) not supported in binary operation",
                        node.operator
                    ),
                    node.span.clone(),
                ));
                self.get_built_in_types(&BuiltInTypes::Unknown)
            }
        }
    }

     /// Visits a unary operation node, validating operand type against the operator,
    /// returning the resulting type or Unknown on error.
    fn visit_unary_op(&mut self, node: &mut UnaryOpNode) -> TypeNode {
        let operand_type = node.operand.accept(self);

        match node.operator {
            OperatorToken::NEG => {
                if operand_type == self.get_built_in_types(&BuiltInTypes::Number) {
                    node.set_type(self.get_built_in_types(&BuiltInTypes::Number));
                    self.get_built_in_types(&BuiltInTypes::Number)
                } else {
                    self.new_error(SemanticError::InvalidUnaryOperation(
                        operand_type,
                        node.operator.clone(),
                        node.span.clone(),
                    ));
                    self.get_built_in_types(&BuiltInTypes::Unknown)
                }
            }
            OperatorToken::NOT => {
                if operand_type == self.get_built_in_types(&BuiltInTypes::Boolean) {
                    node.set_type(self.get_built_in_types(&BuiltInTypes::Boolean));
                    self.get_built_in_types(&BuiltInTypes::Boolean)
                } else {
                    self.new_error(SemanticError::InvalidUnaryOperation(
                        operand_type,
                        node.operator.clone(),
                        node.span.clone(),
                    ));
                    self.get_built_in_types(&BuiltInTypes::Unknown)
                }
            }
            _ => {
                self.new_error(SemanticError::UnknownError(
                    format!(
                        "Operator ( {} ) not supported in unary operation",
                        node.operator.clone()
                    ),
                    node.span.clone(),
                ));
                self.get_built_in_types(&BuiltInTypes::Unknown)
            }
        }
    }

     /// Visits an if-else node, ensuring conditions are Boolean,
    /// and checks type consistency across branches. Returns the type of the if-expression.
    fn visit_if_else(&mut self, node: &mut IfElseNode) -> TypeNode {
        let if_condition_type = node.condition.accept(self);
        if if_condition_type != self.get_built_in_types(&BuiltInTypes::Boolean) {
            self.new_error(SemanticError::InvalidConditionType(
                if_condition_type,
                node.condition.span(),
            ));
        }
        let if_expr_type = node.if_expression.accept(self);
        let mut result = if_expr_type.clone();
        for (condition , body_expr) in node.elifs.iter_mut() {

            let expr_type = body_expr.accept(self);
            if let Some(cond) = condition {
                let cond_type = cond.accept(self);
                if cond_type != self.get_built_in_types(&BuiltInTypes::Boolean) {
                    self.new_error(SemanticError::InvalidConditionType(
                        cond_type,
                        node.span.clone(),
                    ));
                }
            }
            if result != expr_type {
                let lca = self.types_tree.find_lca(&result, &expr_type);
                if lca.type_name == "Unknown" || lca.type_name == "Object" {
                    // TODO añadir error más específico para este error
                    self.new_error(SemanticError::UnknownError(
                        "Incompatible types in if-else branches".to_string(),
                        node.span.clone(),
                    ));
                }
                result = lca ;
            }
        }
        node.set_type(result.clone());
        result
    }

    /// Visits a let-in node, entering a new scope, visiting assignments,
    /// registering variable types, and returning the type of the body expression.
    fn visit_let_in(&mut self, node: &mut LetInNode) -> TypeNode {
        self.enter_scope();
        for assig in node.assignments.iter_mut() {
            let expr_type = assig.expression.accept(self);
            assig.set_type(expr_type.clone());
            self.context
                .symbols
                .insert(assig.identifier.clone(), expr_type.type_name);
        }
        let return_type = node.body.accept(self);
        self.exit_scope();
        node.set_type(return_type.clone());
        return_type
    }

     /// Visits a type definition node, entering a new scope and setting current type context.
    /// Validates parameter names and types, verifies parent type and arguments,
    /// visits members (properties and methods), and returns the type node of the defined type.
    fn visit_type_def(&mut self, node: &mut TypeDefNode) -> TypeNode {
        self.enter_scope();
        self.context.current_type = Some(node.identifier.clone());

        for param in &node.params {
            if self.context.symbols.contains_key(&param.name) {
                self.new_error(SemanticError::ParamNameAlreadyExist(
                    param.name.clone(),
                    node.identifier.clone(),
                    "type".to_string(),
                    param.span,
                ));
            }

            if let Some(type_node) = self.types_tree.get_type(&param.signature) {
                self.context
                    .symbols
                    .insert(param.name.clone(), type_node.type_name.clone());
            } else {
                self.new_error(SemanticError::UndefinedType(
                    param.signature.clone(),
                    param.span,
                ));
                self.context.symbols.insert(
                    param.name.clone(),
                    self.get_built_in_types(&BuiltInTypes::Unknown)
                        .type_name
                        .clone(),
                );
            }
        }

        if let Some(parent_name) = &node.parent {
            if let Some(parent_node) = self.types_tree.get_type(&parent_name) {
                if parent_node.params.len() != node.parent_args.len() {
                    self.new_error(SemanticError::InvalidTypeArgumentCount(
                        node.parent_args.len(),
                        parent_node.params.len(),
                        parent_node.type_name.clone(),
                        node.span,
                    ));
                } else {
                    for (index, arg) in node.parent_args.iter_mut().enumerate() {
                        let arg_type = arg.accept(self);
                        if arg_type.type_name != parent_node.params[index].signature {
                            self.new_error(SemanticError::InvalidTypeArgument(
                                "types".to_string(),
                                arg_type.type_name,
                                parent_node.params[index].signature.clone(),
                                index,
                                node.identifier.clone(),
                                arg.span(),
                            ));
                        }
                    }
                }
            } else {
                self.new_error(SemanticError::UndefinedType(parent_name.clone(), node.span));
            }
        }

        for member in node.members.iter_mut() {
            match member {
                TypeMember::Property(prop) => {
                    let prop_type = prop.expression.accept(self);
                    prop.set_type(prop_type.clone());
                    if let Some(type_node) = self.types_tree.nodes.get_mut(&node.identifier) {
                        type_node
                            .add_variable(prop.identifier.clone(), Box::new(prop_type.type_name));
                    }
                }
                _ => continue,
            }
        }

        for member in node.members.iter_mut() {
            match member {
                TypeMember::Method(method) => {
                    self.visit_function_def(method);
                }
                _ => continue,
            }
        }

        self.exit_scope();
        let return_type = self.types_tree.get_type(&node.identifier).unwrap();
        node.set_type(return_type.clone());
        return_type
    }

     /// Visits a type instance node, checking the number and types of type arguments.
    /// Returns the corresponding type node or Unknown if invalid.
    fn visit_type_instance(&mut self, node: &mut TypeInstanceNode) -> TypeNode {
        if let Some(type_node) = self.types_tree.get_type(&node.type_name) {
            if type_node.params.len() != node.arguments.len() {
                self.new_error(SemanticError::InvalidTypeArgumentCount(
                    node.arguments.len(),
                    type_node.params.len(),
                    node.type_name.clone(),
                    node.span.clone(),
                ));
                self.get_built_in_types(&BuiltInTypes::Unknown)
            } else {
                for (index, arg) in node.arguments.iter_mut().enumerate() {
                    let arg_type = arg.accept(self);
                    let arg_type_node = self.types_tree.get_type(&arg_type.type_name);
                    let param_type_node = self.types_tree.get_type(&type_node.params[index].signature);
                    if !(arg_type_node.is_some() && param_type_node.is_some() 
                        && self.types_tree.is_ancestor(param_type_node.as_ref().unwrap(),arg_type_node.as_ref().unwrap()))
                    {
                        self.new_error(SemanticError::InvalidTypeArgument(
                            "types".to_string(),
                            arg_type.type_name,
                            type_node.params[index].signature.clone(),
                            index,
                            node.type_name.clone(),
                            node.span.clone(),
                        ));
                    }
                }
                node.set_type(type_node.clone());
                type_node
            }
        } else {
            self.new_error(SemanticError::UndefinedType(
                node.type_name.clone(),
                node.span.clone(),
            ));
            self.get_built_in_types(&BuiltInTypes::Unknown)
        }
    }

    /// Visits a type function access node, resolving the method in the object's type,
    /// verifying argument count and types, and returning the function's return type node.
    /// Reports errors if method is not found or arguments mismatch.
    fn visit_type_function_access(&mut self, node: &mut TypeFunctionAccessNode) -> TypeNode {
        let object = node.object.accept(self);
        let member_function = self
            .types_tree
            .find_method(object.type_name.clone(), node.member.function_name.clone());
        if let Some(func) = member_function {
            if func.params.len() != node.member.arguments.len() {
                self.new_error(SemanticError::InvalidArgumentsCount(
                    node.member.arguments.len(),
                    func.params.len(),
                    node.member.function_name.clone(),
                    node.member.span.clone(),
                ));
                self.get_built_in_types(&BuiltInTypes::Unknown)
            } else {
                for (index, arg) in node.member.arguments.iter_mut().enumerate() {
                    let arg_type = arg.accept(self);
                    let arg_type_node = self.types_tree.get_type(&arg_type.type_name);
                    let param_type_node = self.types_tree.get_type(&func.params[index].signature.clone());
                    if ! (arg_type_node.is_some() && param_type_node.is_some() 
                        && self.types_tree.is_ancestor(param_type_node.as_ref().unwrap(),arg_type_node.as_ref().unwrap()))
                    {
                        self.new_error(SemanticError::InvalidTypeArgument(
                            "function".to_string(),
                            arg_type.type_name,
                            func.params[index].signature.clone(),
                            index,
                            node.member.function_name.clone(),
                            node.member.span.clone(),
                        ));
                    }
                }
                if let Some(function_return_type) = self.types_tree.get_type(&func.return_type) {
                    node.set_type(function_return_type.clone());
                    node.member.set_type(function_return_type.clone());
                    function_return_type
                } else {
                    self.new_error(SemanticError::UndefinedType(
                        func.return_type.clone(),
                        node.member.span.clone(),
                    ));
                    self.get_built_in_types(&BuiltInTypes::Unknown)
                }
            }
        } else {
            self.new_error(SemanticError::InvalidTypeFunctionAccess(
                object.type_name.clone(),
                node.member.function_name.clone(),
                node.member.span.clone(),
            ));
            self.get_built_in_types(&BuiltInTypes::Unknown)
        }
    }

     /// Visits a type property access node, resolving the property type in the current type context,
    /// and returning the property type node or Unknown if invalid.
    fn visit_type_prop_access(&mut self, node: &mut TypePropAccessNode) -> TypeNode {
        let object = node.object.accept(self);
        if let Some(current_type) = self.context.current_type.clone() {
            if let Some(type_node) = self.types_tree.nodes.get_mut(&current_type) {
                if let Some(property_type) = type_node.variables.get_mut(node.member.as_ref()) {
                    let property_type_cloned = property_type.clone();
                    let return_type = self.types_tree.get_type(&property_type_cloned).unwrap();
                    node.set_type(return_type.clone());
                    return_type.clone()
                } else {
                    self.new_error(SemanticError::InvalidTypeProperty(
                        object.type_name.clone(),
                        node.member.as_ref().clone(),
                        node.span.clone(),
                    ));
                    self.get_built_in_types(&BuiltInTypes::Unknown)
                }
            } else {
                self.new_error(SemanticError::UndefinedType(
                    current_type.clone(),
                    node.span.clone(),
                ));
                self.get_built_in_types(&BuiltInTypes::Unknown)
            }
        } else {
            self.new_error(SemanticError::InvalidTypePropertyAccess(
                object.type_name.clone(),
                node.member.as_ref().clone(),
                node.span.clone(),
            ));
            self.get_built_in_types(&BuiltInTypes::Unknown)
        }
    }

    /// Visits a print statement node, ensuring the expression is a valid printable type
    /// (Number, String, or Boolean), and returns the expression's type.
    fn visit_print(&mut self, node: &mut PrintNode) -> TypeNode {
        let expr_type = node.expression.accept(self);
        if expr_type.type_name != "Number"
            && expr_type.type_name != "String"
            && expr_type.type_name != "Boolean"
        {
            self.new_error(SemanticError::InvalidPrint(
                expr_type.type_name.clone(),
                node.span.clone(),
            ));
        }
        node.set_type(expr_type.clone());
        expr_type
    }
}
