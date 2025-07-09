
pub fn to_llvm_type(type_node: String) -> String {
    match type_node.as_str() {
        "Number" => "double".to_string(),
        "Boolean" => "i1".to_string(),
        "String" => "ptr".to_string(),
        _ => "ptr".to_string(), // Default to pointer type for unknown types
    }
}