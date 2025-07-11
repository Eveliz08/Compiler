use super::context::CodegenCtx;

/// Convierte el tipo de tu lenguaje al tipo correspondiente de LLVM.
pub fn to_llvm_type(type_node: String) -> String {
    match type_node.as_str() {
        "Number" => "double".to_string(),
        "Boolean" => "i1".to_string(),
        "String" => "ptr".to_string(),
        _ => "ptr".to_string(), // Default to pointer type for unknown types
    }
}

/// Emite las constantes globales de cadena y la declaración de printf.
pub fn declare_global(output: &mut Vec<String>, context: &mut CodegenCtx) {
    output.push(r#"@.str.f = private unnamed_addr constant [4 x i8] c"%f\0A\00", align 1"#.into());
    output.push(r#"@.str.d = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1"#.into());
    output.push(r#"@.str.s = private unnamed_addr constant [4 x i8] c"%s\0A\00", align 1"#.into());
    output.push(r#"@.true_str = private  constant [6 x i8] c"true\0A\00", align 1"#.into());
    output.push(r#"@.false_str = private constant [7 x i8] c"false\0A\00", align 1"#.into());
    output.push(r#"@.newline = private unnamed_addr constant [2 x i8] c"\0A\00", align 1"#.into());
    output.push("declare i32 @printf(ptr, ...)".into());
    output.push("declare i32 @strlen( ptr )".into());
    output.push("declare ptr @strcpy(ptr,ptr)".into());
    output.push("declare ptr @strcat(ptr,ptr)".into());
    output.push("declare i32 @strcmp(ptr ,ptr)".into());
    output.push("declare i8* @malloc(i64)".into());
    output.push("declare double @llvm.frem.f64(double, double)".into());
    output.push("declare double @llvm.pow.f64(double, double)".into());
}

/// Emite una llamada a printf con el formato y valor dados.
pub fn generate_printf(context: &mut CodegenCtx, value: &str, fmt: &str) {
    let (global_name, arg_type) = match fmt {
        "%f" => ("@.str.f", "double"),
        "%d" => ("@.str.d", "i32"),
        "%s" => ("@.str.s", "i8*"),
        _ => panic!("Unsupported format string: {}", fmt),
    };

    context.append_line(format!(
        "call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* {}, i64 0, i64 0), {} {})",
        global_name,
        arg_type,
        value
    ));
}



/// Emite declaraciones para funciones auxiliares de runtime (fmod, pow, concat).
pub fn generate_runtime_declarations(output: &mut Vec<String>) {
    output.push("".into());
    output.push("; Runtime function declarations".into());
    output.push("declare double @fmod(double, double)".into());
    output.push("declare double @pow(double, double)".into());

    // // Cuerpo de fmod: simplemente llama a la función externa
    // output.push("define double @fmod(double %x, double %y) {".into());
    // output.push("entry:".into());
    // output.push("  %result = call double @llvm.frem.f64(double %x, double %y)".into());
    // output.push("  ret double %result".into());
    // output.push("}".into());

    // // Cuerpo de pow: llama a la función externa de LLVM
    // output.push("define double @pow(double %x, double %y) {".into());
    // output.push("entry:".into());
    // output.push("  %result = call double @llvm.pow.f64(double %x, double %y)".into());
    // output.push("  ret double %result".into());
    // output.push("}".into());
    // Cuerpo de concat: implementa la concatenación de cadenas
    output.push("define i8* @concat(i8* %s1, i8* %s2) {".into());
    output.push("entry:".into());
    output.push("  %len1 = call i64 @strlen(i8* %s1)".into());
    output.push("  %len2 = call i64 @strlen(i8* %s2)".into());
    output.push("  %totallen = add i64 %len1, %len2".into());
    output.push("  %totallen1 = add i64 %totallen, 1".into());
    output.push("  %buf = call i8* @malloc(i64 %totallen1)".into());
    output.push("  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %buf, i8* %s1, i64 %len1, i1 false)".into());
    output.push("  %buf_offset = getelementptr i8, i8* %buf, i64 %len1".into());
    output.push("  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %buf_offset, i8* %s2, i64 %len2, i1 false)".into());
    output.push("  %last = getelementptr i8, i8* %buf, i64 %totallen".into());
    output.push("  store i8 0, i8* %last".into());
    output.push("  ret i8* %buf".into());
    output.push("}".into());
}