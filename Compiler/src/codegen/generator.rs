use super::context::CodegenCtx;
// use super::llvm_utils::*; // Descomentar si se usan utilidades LLVM
use crate::ast_nodes::program::{Program, Statement};
use crate::codegen::utils::{declare_global, generate_printf, generate_runtime_declarations};
use crate::visitor::accept::Accept;

pub struct Generator {
    pub ctx: CodegenCtx, // context (renombrado)
    pub code: String, // Nuevo: almacena el código generado
}

impl Generator {
    pub fn new() -> Self {
        Self {
            ctx: CodegenCtx::new(),
            code: String::new(),
        }
    }

    /// Genera el código y actualiza la propiedad `code` con el resultado generado como string.
    pub fn generate(&mut self, program: &mut Program) {
        let mut module_code: Vec<String> = vec![];
        self.generate_header(&mut module_code);
        declare_global(&mut module_code, &mut self.ctx);
        generate_runtime_declarations(&mut module_code);
        module_code.push("".into());

        let mut body_ctx = CodegenCtx::new();
        std::mem::swap(&mut self.ctx, &mut body_ctx);
        let globals = self.ctx.take_globals();
        std::mem::swap(&mut self.ctx, &mut body_ctx);

        module_code.extend(globals);
        if !module_code.last().map(|s| s.is_empty()).unwrap_or(false) {
            module_code.push("".into());
        }

        self.init_all_type_methods_and_props(program);
        module_code.extend(self.get_definitions(program));
        let main_code = &self.get_main_code(program);
        generate_main_wrapper(&mut module_code, &main_code, self.ctx.string_consts.clone());
        self.code = module_code.join("\n");
    }
    fn generate_header(&mut self, module_code: &mut Vec<String>) {
        module_code.push(" ".into());
    }



    fn get_definitions(&mut self, _program: &mut Program) -> Vec<String> {
        // Implementar lógica según sea necesario
        Vec::new()
    }

    fn get_main_code(&mut self, _program: &mut Program) -> Vec<String> {
        // Implementar lógica según sea necesario
        Vec::new()
    }
}

// Carcasa de función global usada en generate
fn generate_main_wrapper(module_code: &mut Vec<String>, body_code: &[String], global_consts: Vec<String>) {
    for global_const in global_consts {
        module_code.push(global_const);
    }
    module_code.push("define i32 @main() {".into());
    module_code.push("entry:".into());
    for line in body_code {
        module_code.push("  ".to_string() + line);
    }
    module_code.push("  ret i32 0".into());
    module_code.push("}".into());
}
