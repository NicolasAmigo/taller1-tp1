use std::collections::HashMap;
use std::io::Write;

#[derive(Debug)]
/// Representacion de un tablero de buscaminas despues de aplicar su transformacion.
/// Solo se puede crear procesando un TableroSinProcesar
pub struct TableroProcesado {
    pub tablero: HashMap<(i32, i32), u8>,
}

impl TableroProcesado {
    // Dado un tablero procesado escribe al output ingresado el tablero.
    // Si falla devuelve la razon de fallo
    pub fn imprimir<T: Write>(self, mut output: T) -> Result<(), String> {
        let mut x = 0;
        let mut y = 0;
        let mut line: Vec<u8>;
        let mut texto: Vec<Vec<u8>> = vec![];

        while self.tablero.get(&(x, y)) != Option::None {
            line = vec![];
            while self.tablero.get(&(x, y)) != Option::None {
                line.push(match self.tablero.get(&(x, y)) {
                    None => return Err("Error al procesar".to_string()),
                    Some(x) => *x,
                });
                y += 1;
            }
            line.push(b'\n');
            texto.push(line);

            x += 1;
            y = 0;
        }
        for linea in texto.iter() {
            if output.write_all(linea).is_err() {
                return Err("Error al escribir".to_string());
            }
        }

        Ok(())
    }
    #[allow(dead_code)]
    // Dada una posicion devuelve el valor del tablero asociado
    // Falla en caso de no existir en esa posicion
    pub fn get(&self, x: i32, y: i32) -> Option<&u8> {
        self.tablero.get(&(x, y))
    }
}
