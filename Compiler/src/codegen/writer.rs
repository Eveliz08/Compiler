use std::fs::File;
use std::io::Write;

pub fn write_to_file(code: &str, filename: &str) {
    let mut file = File::create(filename).expect("No se pudo crear el archivo LLVM");
    file.write_all(code.as_bytes()).expect("Error al escribir el archivo LLVM");
}
