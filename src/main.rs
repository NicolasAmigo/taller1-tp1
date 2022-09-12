use std::env;
pub mod tablero_procesado;
pub mod tablero_sin_procesar;

fn main() {
    let args: Vec<String> = env::args().collect();
    tp1::buscaminas(&args[1]);
}
