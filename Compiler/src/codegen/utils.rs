use super::context::CodegenCtx;

pub fn to_llvm_type(type_node: String) -> String {
    match type_node.as_str() {
        "Number" => "double".to_string(),
        "Boolean" => "i1".to_string(),
        "String" => "ptr".to_string(),
        _ => "ptr".to_string(), // Default to pointer type for unknown types
    }
}

/// Emit the global string constants and the printf declaration.
pub fn declare_global(output: &mut Vec<String>, context: &mut CodegenCtx) {
    output.push("@PI = constant double 0x400921FB54442D18".into()); // π
    output.push("@E = constant double 0x4005BF0A8B145769".into()); // e
    context.add_global_const("PI");
    context.add_global_const("E");
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
}

/// Emit a call to printf with the given format and value.
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

/// Emit the module header—ModuleID, data layout, and target triple—dynamically
/// obtained from environment variables set by build.rs.
pub fn generate_header(output: &mut Vec<String>) {
    output.push("; ModuleID = 'hulk'".into());
    output.push("target datalayout = \"e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128\"".into());
    output.push("target triple = \"x86_64-pc-linux-gnu\"".into());
}

/// Emit the `main` wrapper around the generated body.
pub fn generate_main_wrapper(output: &mut Vec<String>, body: &[String], _global_consts: Vec<String>) {
    output.push("define i32 @main() {".into());
    output.push("entry:".into());
    for line in body {
        output.push(format!("  {}", line));
    }
    output.push("  ret i32 0".into());
    output.push("}".into());
}

/// Emit declarations for runtime helper functions (fmod, pow, concat).
pub fn generate_runtime_declarations(output: &mut Vec<String>) {
    output.push("".into());
    output.push("; Runtime function declarations".into());
    output.push("declare double @fmod(double, double)".into());
    output.push("declare double @pow(double, double)".into());
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