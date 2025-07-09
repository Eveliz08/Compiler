use crate::{ast_nodes::program::Program, codegen::generator::Generator};
use std::collections::HashMap;
use crate::{Statement, TypeDefNode, TypeMember};
use crate::codegen::utils::to_llvm_type;

impl Generator {
   

    // Carcasa de funciones auxiliares usadas en generate
    pub fn init_all_type_methods_and_props(&mut self, _program: &mut Program) {
        // Implementar lógica según sea necesario
    }

    

    fn aux_init_types(
        &mut self,
        node: &Box<TypeDefNode>,
        node_list: Vec<&Box<TypeDefNode>>,
        visited: &mut HashMap<String, bool>,
    ) {
        if let Some(parent_name) = node.parent.clone() {
            if let Some(parent_node) = node_list.iter().find(|n| n.identifier == parent_name) {
                if !visited.contains_key(&parent_name) {
                    self.aux_init_types(parent_node, node_list.clone(), visited);
                }
            }
        }
        self.generate_type_table(&mut node.clone());
        visited.insert(node.identifier.clone(), true);
    }

    pub fn generate_type_table(&mut self, node: &mut TypeDefNode) {
        // Carcasa vacía, implementar lógica según sea necesario
    }

    pub fn generate_type_constructor(&mut self, node: &mut TypeDefNode) {
        // Carcasa vacía, implementar lógica según sea necesario
    }

    
    pub fn generate_get_vtable_method(&mut self) {
        self.ctx.append_line("define ptr @get_vtable_method(i32 %type_id, i32 %method_id) {".to_string());
        self.ctx.append_line(format!("%vtable_ptr_ptr = getelementptr [ {} x ptr ], ptr @super_vtable, i32 0, i32 %type_id", self.ctx.defined_types_count));
        self.ctx.append_line(format!("%vtable_ptr = load ptr , ptr %vtable_ptr_ptr"));
        self.ctx.append_line(format!("%typed_vtable = bitcast ptr %vtable_ptr to ptr"));
        self.ctx.append_line(format!("%method_ptr = getelementptr [ {} x ptr ], ptr %typed_vtable, i32 0, i32 %method_id", self.ctx.max_vtable_funcs));
        self.ctx.append_line(format!("%method = load ptr, ptr %method_ptr"));
        self.ctx.append_line(format!("ret ptr %method"));
        self.ctx.append_line("}".to_string());
    }

}