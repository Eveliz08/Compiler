use std::process::Command;
// use std::env;

pub fn run_llvm_ir(filename: &str) {
    let (output, clang_args): (&str, Vec<&str>) = if cfg!(target_os = "windows") {
        (
            "output.exe",
            vec![
                filename,
                "-o",
                "output.exe",
                "-fuse-ld=lld",
                "--target=x86_64-w64-windows-gnu",
            ],
        )
    } else if cfg!(target_os = "macos") {
        (
            "output_macos",
            vec![
                filename,
                "-o",
                "output_macos",
                "--target=x86_64-apple-darwin",
            ],
        )
    } else {
        // Assume Linux
        (
            "output_linux",
            vec![
                filename,
                "-o",
                "output_linux",
                "-fuse-ld=lld",
                "--target=x86_64-pc-linux-gnu",
            ],
        )
    };

    if !Command::new("clang")
        .args(&clang_args)
        .status()
        .map_or(false, |s| s.success())
    {
        eprintln!("Falló la compilación con clang");
        return;
    }

    let exec_cmd = if cfg!(target_os = "windows") {
        format!(".\\{}", output)
    } else {
        format!("./{}", output)
    };

    if !Command::new(exec_cmd)
        .status()
        .map_or(false, |s| s.success())
    {
        eprintln!("Falló la ejecución del ejecutable generado");
    }
}
