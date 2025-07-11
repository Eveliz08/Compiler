use crate::ast_nodes::program;
use crate::visitor::accept::Accept;
use crate::{ast_nodes::program::Program, codegen::generator::Generator};
use std::collections::HashMap;
use crate::{Statement, TypeDefNode, TypeMember};
use crate::codegen::utils::to_llvm_type;

impl Generator {
   

    pub fn init_all_type_methods_and_props(&mut self, program: &mut Program) {
        let mut type_def_list = Vec::new();
        for statement in &mut program.statements {
            match statement {
                Statement::StatementTypeDef(type_def) => {
                   type_def_list.push(type_def);
                }
                _ => continue
            }
        }
        let mut visited:  HashMap<String, bool> = HashMap::new(); 
        let node_list: Vec<&Box<TypeDefNode>> = type_def_list.iter().map(|r| &**r).collect();
        for node in &type_def_list {
            if !visited.contains_key(&node.identifier) {
                self.aux_init_types(node, node_list.clone(), &mut visited);
            }
        }
    }

    

    fn aux_init_types(
        &mut self,
        node: &Box<TypeDefNode>,
        node_list: Vec<&Box<TypeDefNode>>,
        visited: &mut HashMap<String, bool>,
    ) {
        if let Some(parent_name) = node.parent.clone()
         {
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
        let type_name = node.identifier.clone();
        let mut methods: Vec<(String, String)> = Vec::new();
        let mut count_functions = 0;

        self.ctx.type_ids.insert(node.identifier.clone(), self.ctx.defined_types_count.clone());
        self.ctx.vtables_per_type.push(format!("@{}_vtable", node.identifier.clone()));
        self.ctx.defined_types_count += 1;
    
        let mut params_types_list = Vec::new();
        for param in node.params.iter() {
            let param_type = param.signature.clone();
            params_types_list.push(param_type);
        }
        self.ctx.ctor_arg_types.insert(type_name.clone(), params_types_list);
        if let Some(parent_type) = &node.parent {
            self.ctx.type_inheritance.insert(type_name.clone(), parent_type.clone());
        }

        let mut props_list = Vec::new();
        let mut member_index: i32 = 2 ;

        if let Some(parent_name) = &node.parent {
            if let Some(parent_members) = self.ctx.type_members_map.get(parent_name) {
                for (index,( member_name,member_type)) in parent_members.iter().enumerate() {
                    self.ctx.member_types.insert((type_name.clone(), member_name.clone()), member_type.clone());
                    self.ctx.member_indices.insert((type_name.clone(), member_name.clone()), index.clone() as i32);
                    member_index += 1;
                }
                props_list = parent_members.clone();
            }
        }

        for member in node.members.iter() {
            match member { 
                TypeMember::Property(assignment) => {
                    let member_name = assignment.identifier.clone();
                    self.ctx.member_indices.insert((type_name.clone(), member_name.clone()), member_index);
                    self.ctx.member_types.insert((type_name.clone(), member_name.clone()), assignment.node_type.clone().unwrap().type_name);
                    props_list.push((member_name.clone(), assignment.node_type.clone().unwrap().type_name));
                    member_index += 1;
                }
                TypeMember::Method(method) => {
                    count_functions += 1;
                    let method_name = method.name.clone();
                    self.ctx.member_func_llvm_names.insert((type_name.clone(), method_name.clone()), format!("@{}_{}", type_name, method_name.clone()));
                    let mut method_args_types = Vec::new(); 
                    for param in &method.params {
                        method_args_types.push(param.signature.clone());
                    }
                    self.ctx.member_func_args.insert((type_name.clone(), method_name.clone(), member_index), method_args_types);
                    self.ctx.member_types.insert((type_name.clone(),method_name.clone()), method.node_type.clone().unwrap().type_name);
                }
            }
        }
        self.ctx.type_members_map.insert(type_name.clone(), props_list);
        self.ctx.max_vtable_funcs += count_functions;

        let mut props_types = Vec::new();
        if let Some(props_list) = self.ctx.type_members_map.get(&type_name) {
            for (_prop_name, prop_type) in props_list {
                props_types.push(to_llvm_type(prop_type.clone()));
            }
        } 

        let list_props_str = props_types
            .iter()
            .map(|llvm_name| format!("{}", llvm_name))
            .collect::<Vec<_>>()
            .join(", ");
        // type (vtable , parent , props...)
        if props_types.len() > 0 {
            self.ctx.append_line(format!("%{}_type = type {{ i32, ptr, {} }}", type_name.clone(), list_props_str)); 
        } else {
            self.ctx.append_line(format!("%{}_type = type {{ i32, ptr }}", type_name.clone())); 
        }
        
        // lll
        if let Some(parent_name) = &node.parent {
            if let Some(parent_methods) = self.ctx.type_functions_map.get(parent_name) {
                methods = parent_methods.clone();
            }
        }
        
        for member in node.members.iter() {
            match member {
                TypeMember::Method(method) => {
                    if let Some(llvm_name) = self.ctx.member_func_llvm_names.get(&(type_name.clone(), method.name.clone())) {
                        if let Some(idx) = methods.iter().position(|(name, _)| name == &method.name.clone()) {
                            methods[idx] = (method.name.clone(),llvm_name.clone());
                        } else {
                            methods.push((method.name.clone(), llvm_name.clone()));
                        }
                    }
                }
                _ => continue 
            }
        }
        for (index, (name, _)) in methods.iter().enumerate() {
            self.ctx.func_indices.insert((type_name.clone(),name.clone()),index as i32);
        }
        self.ctx.type_functions_map.insert(type_name.clone(), methods);
    }

    pub fn generate_type_constructor(&mut self, node: &mut TypeDefNode) {
        let type_name = node.identifier.clone();
        let type_reg = format!("%{}_type",type_name);
        let mut params_list = Vec::new();
        self.ctx.push_scope();
        for param in node.params.iter() {
            let param_name  = format!("%{}.{}",param.name.clone(),self.ctx.current_scope_id());
            params_list.push(format!("ptr {}",param_name.clone()));
            self.ctx.add_var(param_name.clone(), to_llvm_type(param.signature.clone()));
        }
        let params_str = params_list.join(", ");

    
        // Crea una lista del tamaño de max_functions, inicializada con "ptr null"
        let mut method_list = vec!["ptr null".to_string(); self.ctx.max_vtable_funcs as usize];

        // Llena la lista con el nombre de la función en el índice correspondiente
        if let Some(functions) = self.ctx.type_functions_map.get(&type_name) {
            for (index,(_, llvm_name))in functions.iter().enumerate() {
                if index < self.ctx.max_vtable_funcs as usize {
                    method_list[index] = format!("ptr {}", llvm_name);
                }
            }
        }

        // generate vtable instance 
        let type_table_instance = format!("@{}_vtable", type_name);

        // Crea la instancia de la vtable usando method_list
        self.ctx.append_line(format!("{} = constant %VTableType [ {} ]", type_table_instance, method_list.join(", ")));
        
        // build constructor
        self.ctx.append_line(format!("define ptr @{}_new( {} ) {{",type_name.clone(),params_str.clone())); 

        let size_temp = self.ctx.fresh_temp_var("Number".to_string());
        self.ctx.append_line(format!("{} = ptrtoint ptr getelementptr({}, ptr null, i32 1) to i64", size_temp, type_reg));
        let mem_temp = self.ctx.fresh_temp_var(type_name.clone());
        self.ctx.append_line(format!("{} = call ptr @malloc(i64 {})" , mem_temp , size_temp));

        // set type index on super_vtable
        self.ctx.append_line(format!("%index_ptr = getelementptr {}, ptr {}, i32 0, i32 0", type_reg, mem_temp));
        self.ctx.append_line(format!("store i32 {}, ptr %index_ptr", self.ctx.type_ids.get(&type_name).expect("Type ID not found for type").clone()));

        if let Some(parent_name) = node.parent.clone() {
            let mut parent_args_values = Vec::new();
            for arg in node.parent_args.iter_mut() {
                let arg_result = arg.accept(self);
                let arg_reg = self.ctx.fresh_temp_var(arg_result.ast_type);
                self.ctx.append_line(format!("{} = alloca {}", arg_reg.clone() , arg_result.llvm_type.clone()));
                self.ctx.append_line(format!(
                    "store {} {}, ptr {}",
                    arg_result.llvm_type, arg_result.register, arg_reg.clone()
                ));
                parent_args_values.push(format!("ptr {}",arg_reg.clone()));
            }
            let args_regs_str = parent_args_values.join(", ");
            let parent_ptr = self.ctx.fresh_temp_var(parent_name.clone());
            let parent_constructor_name = format!("@{}_new" , parent_name.clone()); 
            self.ctx.append_line(format!(
                "{} = call ptr {}({})",
                parent_ptr.clone(), parent_constructor_name, args_regs_str
            ));
            self.ctx.append_line(format!("%parent_ptr = getelementptr {}, ptr {}, i32 0, i32 1", type_reg, mem_temp));
            self.ctx.append_line(format!("store ptr {}, ptr %parent_ptr", parent_ptr.clone()));
            if let Some(parent_members) = self.ctx.type_members_map.get(&parent_name) {
                let parent_members_cloned = parent_members.clone();
                for (index, (_member_name, member_type)) in parent_members_cloned.iter().enumerate() {
                    let llvm_type = to_llvm_type(member_type.clone());
                    let parent_type = format!("%{}_type", parent_name.clone());
                    self.ctx.append_line(format!(
                        "%src_{} = getelementptr {}, ptr {}, i32 0, i32 {}",
                        index.clone(), parent_type.clone(), parent_ptr.clone(), index.clone() + 2
                    ));
                    self.ctx.append_line(format!(
                        "%val_{} = load {}, ptr %src_{}",
                        index.clone(), llvm_type, index.clone()
                    ));
                    self.ctx.append_line(format!(
                        "%dst_{} = getelementptr {}, ptr {}, i32 0, i32 {}",
                        index.clone(), type_reg, mem_temp, index.clone() + 2
                    ));
                    self.ctx.append_line(format!(
                        "store {} %val_{}, ptr %dst_{}",
                        llvm_type, index.clone(), index.clone()
                    ));
                }
            }
        }
        
        // set properties values 

        for member in node.members.iter() {
            match member {
                TypeMember::Property(assign) => {
                    let prop_reg = assign.expression.clone().accept(self);
                    let result_reg = self.ctx.fresh_temp_var(prop_reg.ast_type.clone());
                    let member_key = (type_name.clone(), assign.identifier.clone());
                    let member_index = self.ctx.member_indices.get(&member_key)
                        .expect("Member index not found for type and param name");
                    self.ctx.append_line(format!(
                        "{} = getelementptr {}, ptr {}, i32 0, i32 {}",
                        result_reg, type_reg, mem_temp, member_index
                    ));
                    self.ctx.append_line(format!("store {} {}, ptr {}", prop_reg.llvm_type, prop_reg.register, result_reg));
                }
                _ => continue 
            }
        }

        self.ctx.append_line(format!("ret ptr {}", mem_temp));
        self.ctx.append_line("}".to_string());

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