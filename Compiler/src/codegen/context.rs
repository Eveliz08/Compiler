use std::collections::{HashSet, HashMap};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CtxType { // Type
    Double,
    Boolean,
    String,
}

pub struct CodegenCtx { // CodeGenContext
    // Vector de strings representando el código generado (code)
    pub generated_lines: Vec<String>, // code
    // Variables globales (globals)
    pub global_vars: Vec<String>, // globals
    // Conjunto de constantes globales (global_constants)
    pub global_consts: HashSet<String>, // global_constants
    // Constantes de string usadas en el código (str_constants)
    pub string_consts: Vec<String>, // str_constants
    // Contador para constantes de string (str_counter)
    pub string_const_counter: usize, // str_counter
    // Contador para variables temporales (temp_counter)
    pub temp_var_counter: usize, // temp_counter
    // ID único para el contexto (id)
    pub ctx_id: usize, // id
    // ID de scope actual (scope_id)
    scope_counter: i32, // scope_id
    // Tipos de variables temporales (temp_types)
    pub temp_var_types: HashMap<String, String>, // temp_types
    // Literales de string (string_literals)
    pub string_literals_map: HashMap<String, String>, // string_literals
    // Siguiente ID de literal de string (next_string_id)
    pub next_str_lit_id: usize, // next_string_id
    // Funciones de runtime usadas (runtime_functions)
    pub used_runtime_funcs: HashSet<String>, // runtime_functions
    // Variables en el scope actual (variables)
    pub scoped_vars: HashMap<String, String>, // variables
    // Pila de scopes (scopes)
    pub scopes_stack: Vec<HashMap<String, String>>, // scopes
    // (type, function_name) -> function_llvm_name (function_member_llvm_names)
    pub member_func_llvm_names: HashMap<(String, String), String>, // function_member_llvm_names
    // (type) -> type_parent (inherits)
    pub type_inheritance: HashMap<String, String>, // inherits
    // (type) -> type_constructor_args (constructor_args_types)
    pub ctor_arg_types: HashMap<String, Vec<String>>, // constructor_args_types
    // (type, function_name, function_index) -> function_arguments (types_members_functions)
    pub member_func_args: HashMap<(String,String,i32), Vec<String>>, // types_members_functions
    // (type, member) -> member_type (type_members_types)
    pub member_types: HashMap<(String, String), String>, // type_members_types
    // (type, member) -> member_index_on_type_struct (type_members_ids)
    pub member_indices: HashMap<(String, String), i32>, // type_members_ids
    // (type, member) -> function_index_on_v_table (type_functions_ids)
    pub func_indices: HashMap<(String,String),i32>, // type_functions_ids
    // Tipo self actual (current_self)
    pub current_self_type: Option<String>, // current_self
    // Máximo de funciones en vtable (max_functions)
    pub max_vtable_funcs: i32, // max_functions
    // Conteo de tipos definidos (count_types)
    pub defined_types_count: i32, // count_types
    // (type_name) -> type_id (type_id)
    pub type_ids: HashMap<String, i32>, // type_id
    // VTable por tipo (types_vtables)
    pub vtables_per_type: Vec<String>, // types_vtables
    // (type) -> [(function_name, function_llvm_name)] (types_functions)
    pub type_functions_map: HashMap<String, Vec<(String,String)>>, // types_functions
    // (type) -> [(member_name,member_type)] (types_members)
    pub type_members_map: HashMap<String, Vec<(String,String)>>, // types_members
}

#[derive(Clone)]
pub struct VarInfo { // VariableInfo
    pub temp_var: String, // temp
    pub var_type: String, // ty
}

impl Default for CodegenCtx { // CodeGenContext
    fn default() -> Self {
        Self {
            generated_lines: Vec::new(), // code
            global_vars: Vec::new(), // globals
            global_consts: HashSet::new(), // global_constants
            string_consts: Vec::new(), // str_constants
            string_const_counter: 0, // str_counter
            temp_var_counter: 1, // temp_counter
            ctx_id: 1, // id
            scope_counter: 0, // scope_id
            temp_var_types: HashMap::new(), // temp_types
            string_literals_map: HashMap::new(), // string_literals
            next_str_lit_id: 0, // next_string_id
            used_runtime_funcs: HashSet::new(), // runtime_functions
            scoped_vars: HashMap::new(), // variables
            scopes_stack: Vec::new(), // scopes
            member_func_llvm_names: HashMap::new(), // function_member_llvm_names
            type_inheritance: HashMap::new(), // inherits
            ctor_arg_types: HashMap::new(), // constructor_args_types
            member_func_args: HashMap::new(), // types_members_functions
            member_types: HashMap::new(), // type_members_types
            member_indices: HashMap::new(), // type_members_ids
            func_indices: HashMap::new(), // type_functions_ids
            current_self_type: None, // current_self
            max_vtable_funcs: 0, // max_functions
            defined_types_count: 0, // count_types
            type_ids: HashMap::new(), // type_id
            vtables_per_type: Vec::new(), // types_vtables
            type_functions_map: HashMap::new(), // types_functions
            type_members_map: HashMap::new(), // types_members
        }
    }
}

impl CodegenCtx { // CodeGenContext
    pub fn new() -> Self {
        Self::default()
    }

    pub fn append_line(&mut self, line: String) { // add_line
        self.generated_lines.push(line);
    }

    pub fn fresh_temp_var(&mut self, var_type: String) -> String { // new_temp
        let id = self.temp_var_counter;
        self.temp_var_counter += 1;
        let name = format!("%{}", id);
        self.temp_var_types.insert(name.clone(), var_type);
        name
    }

    pub fn take_generated(&mut self) -> Vec<String> { // take_code
        let mut result = Vec::new();
        result.extend(std::mem::take(&mut self.global_vars));
        result.extend(std::mem::take(&mut self.generated_lines));
        result
    }

    pub fn add_global_const(&mut self, name: &str) { // add_global_constant
        self.global_consts.insert(name.to_string());
    }
    
    pub fn is_global_const(&self, name: &str) -> bool { // is_global_constant
        self.global_consts.contains(name)
    }

    pub fn take_globals(&mut self) -> Vec<String> { // take_globals
        std::mem::take(&mut self.global_vars)
    }

    pub fn take_body(&mut self) -> Vec<String> { // take_body
        std::mem::take(&mut self.generated_lines)
    }

    pub fn get_temp_var_type(&mut self, temp: &str) -> String { // get_type
        self.temp_var_types.get(temp).expect("Unknown temporary").clone()
    }

    pub fn is_bool_type(&mut self, name: &str) -> bool { // is_bool
        self.get_temp_var_type(name) == "Boolean"
    }

    pub fn is_string_type(&mut self, name: &str) -> bool { // is_string
        self.get_temp_var_type(name) == "String"
    }

    pub fn push_scope(&mut self) { // enter_scope
        self.scope_counter += 1;
        self.scopes_stack.push(self.scoped_vars.clone())
    }

    pub fn pop_scope(&mut self) { // exit_scope
        self.scoped_vars = self.scopes_stack.pop().unwrap_or_default();
    }

    pub fn current_scope_id(&self) -> i32 { // get_scope
        self.scope_counter
    }

    pub fn add_string_literal(&mut self, value: &str) -> String { // add_string_literal
        if let Some(name) = self.string_literals_map.get(value) {
            return name.clone();
        }

        let escaped = value
            .replace("\\", "\\\\")
            .replace("\"", "\\\"")
            .replace("\n", "\\n")
            .replace("\t", "\\t");

        let name = format!("@.str.{}", self.next_str_lit_id);
        self.next_str_lit_id += 1;

        let global = format!(
            "{} = private unnamed_addr constant [{} x i8] c\"{}\\00\"",
            name,
            escaped.len() + 1,
            escaped
        );
        self.global_vars.push(global);
        self.string_literals_map.insert(value.to_string(), name.clone());
        name
    }

    pub fn add_string_const(&mut self, value: String, len: usize) -> String { // add_str_const
        let constant_name = format!("@.str.{}", self.string_const_counter);
        self.string_const_counter += 1;
        
        let escaped_value = value
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', "\\0A")
            .replace('\0', "\\00");
        
        let line = format!(
            "{} = private unnamed_addr constant [{} x i8] c\"{}\\00\"", 
            constant_name, len + 1, escaped_value
        );
        
        if !self.string_consts.contains(&line) { 
            self.string_consts.push(line);
        }
        constant_name
    }
    
    pub fn add_global_decl(&mut self, decl: String) { // add_global_declaration
        self.global_vars.push(decl);
    }
    
    pub fn add_var(&mut self, name: String, var_type: String) { // add_variable
        self.scoped_vars.insert(
            name,
            var_type,
        );
    }
    
    pub fn get_var(&self, name: String) -> String { // get_variable
        let mut current_scope = self.scope_counter.clone();
        while current_scope >= 0 {
            let register = format!("%{}.{}",name,current_scope);
            if let Some(_) = self.scoped_vars.get(&register) {
                return register;
            }
            current_scope -= 1;
        }
        panic!("Variable not found: {}",name.to_string())
    }

    pub fn fresh_id(&mut self) -> usize { // new_id
        let id = self.ctx_id;
        self.ctx_id += 1;
        id
    }
}
