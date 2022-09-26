pub mod tablero_procesado;
pub mod tablero_sin_procesar;

use std::io;
/// Dado un path name imprime por stdout un buscaminas transformado
/// siguiendo las reglas del tp.
/// En caso de falla imprimira por stdout la causa
pub fn buscaminas(path_file: &String) {
    let tablero = tablero_sin_procesar::TableroSinProcesar::new(path_file);
    if let Err(e) = tablero {
        println!("Error al leer archivo: {}", e);
        return;
    }
    let tablero = tablero.unwrap().procesar();
    if let Err(e) = tablero.imprimir(io::stdout()) {
        println!("Error al imprimir: {}", e);
    }
}
