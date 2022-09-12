pub mod tablero_procesado;
pub mod tablero_sin_procesar;
/// Dado un path name imprime por stdout un buscaminas transformado
/// siguiendo las reglas del tp.
pub fn buscaminas(path_file: &String) {
    let tablero = tablero_sin_procesar::TableroSinProcesar::new(path_file);
    let tablero = tablero.procesar();
    tablero.imprimir();
}
