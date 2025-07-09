use super::context::CodegenCtx;
// use super::llvm_utils::*; // Descomentar si se usan utilidades LLVM
// use crate::ast_nodes::program::{Program, Statement}; // Descomentar si se usan nodos AST
// use crate::visitor::accept::Accept; // Descomentar si se usa el trait Accept

pub struct Generator {
    pub ctx: CodegenCtx, // context (renombrado)
    pub code: Vec<String>, // Nuevo: almacena el código generado
}

impl Generator {
    pub fn new() -> Self {
        Self {
            ctx: CodegenCtx::new(),
            code: Vec::new(),
        }
    }

 
}
