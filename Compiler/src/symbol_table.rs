use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug)]
pub struct SymbolTable {
    symbols: HashMap<String, Symbol>,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Symbol {
    name: String,
    symbol_type: SymbolType,
    scope: String,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum SymbolType {
    Variable,
    Function,
    // Añadir más tipos según se necesite
}
