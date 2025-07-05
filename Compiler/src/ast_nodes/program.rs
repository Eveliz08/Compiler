use crate::visitor::accept::Accept;
use super::function_def::FunctionDefNode;
use super::expression::Expression;
use crate::visitor::visitor_trait::Visitor;
use super::type_def::TypeDefNode;

#[derive(Debug, PartialEq)]
pub struct Program{
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    StatementExpression(Box<Expression>),
    StatementFunctionDef(Box<FunctionDefNode>),
    StatementTypeDef(Box<TypeDefNode>),
}

impl Statement {
    pub fn new_expression(expression: Expression) -> Self {
        Statement::StatementExpression(Box::new(expression))
    }

    pub fn new_function_def(func_def: FunctionDefNode) -> Self {
        Statement::StatementFunctionDef(Box::new(func_def))
    }

    pub fn new_type_def(type_def: TypeDefNode) -> Self {
        Statement::StatementTypeDef(Box::new(type_def))
    }
}

impl Accept for Statement {
    fn accept<V: Visitor<T>,T>(&mut self, visitor: &mut V) -> T {
        match self {
            Statement::StatementExpression(expr) => expr.accept(visitor),
            Statement::StatementFunctionDef(node) => visitor.visit_function_def(node),
            Statement::StatementTypeDef(node) => visitor.visit_type_def(node),
        }
    }
}
